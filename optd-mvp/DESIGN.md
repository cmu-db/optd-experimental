# Duplicate Elimination Memo Table

_Connor Tsui, December 2024_

Note that most of the details are in `src/memo/persistent/implementation.rs`.

For this document, we are assuming that the memo table is backed by a database / ORM. Both the
problems and the features detailed in this document are unique to this design, and likely do not
apply to an in-memory memo table.

## Group Merging

During logical exploration, there will be rules that create cycles between groups. The easy solution
for this is to immediately merge two groups together when the engine determines that adding an
expression would result in a duplicate expression from another group.

However, if we want to support parallel exploration, this could be prone to high contention. By
definition, merging group 1 into group 2 would mean that _every expression_ that has a child of
group 1 with would need to be rewritten to point to group 2 instead.

This is prohibitive in a parallel setting, as that would mean every single task that gets affected
would need to either wait for the rewrites to happen before resuming work, or potentially need to
abort their work because data has changed underneath them.

So immediate / eager group merging is not a great idea for parallel exploration. However, if we
don't merge two groups that are equivalent, we are subject to doing duplicate work for every
duplicate expression in the memo table during physical optimization.

Instead of merging groups together immediately, we can instead maintain an auxiliary data structure
that records the groups that _eventually_ need to get merged, and "lazily" merge those groups
together once every group has finished exploration. We will refer to merging groups as the act of
recording that the groups should eventually be merged together after exploration is finished.

## Union-Find Group Sets

We use the well-known Union-Find algorithm and corresponding data structure as the auxiliary data
structure that tracks the to-be-merged groups.

Union-Find supports `Union` and `Find` operations, where `Union` merges sets and `Find` searches for
a "canonical" or "root" element that is shared between all elements in a given set. Note that we
will also support an iteration operation that iterates over all elements in a given set. We will
need this for [duplicate detection](#fingerprinting--group-merge), which is explained below.

For more information about Union-Find, see these
[15-451 lecture notes](https://www.cs.cmu.edu/~15451-f24/lectures/lecture08-union-find.pdf). We will
use the exact same data structure, but add an additional `next` pointer for each node that embeds
a circular linked list for each set.

Here, we make the elements the groups themselves (really the group IDs), which allows us to merge
group sets together and also determine a "root group" that all groups in a set can agree on.

When every group in a group set has finished exploration, we can safely begin to merge them
together by moving all expressions from every group in the group set into a single large group.
Other than making sure that any reference to an old group in the group set points to this new large
group, exploration of all groups is done and physical optimization can start.

Note that since we are now waiting for exploration of all groups to finish, this algorithm is much
closer to the Volcano framework than the Cascades' incremental search. However, since we eventually
will want to store trails / breadcrumbs of decisions made to skip work in the future, and since we
essentially have unlimited space due to the memo table being backed by a DBMS, this is not as much
of a problem.

## Duplicate Detection

Deciding that we will merge groups lazily does not solve all of our problems. We have to know _when_
we want to merge these groups.

A naive approach is to simply loop over every expression in the memo table and check if we are about
to insert a duplicate. This, of course, is bad for performance.

We will use a fingerprinting / hashing method to detect when a duplicate expression might be
inserted into the memo table (returning an error instead of inserting), and we will use that to
trigger group merges.

For every logical expression we insert into the memo table, we will create a fingerprint that
contains both the kind of expression / relation (Scan, Filter, Join) and a hash of all
information that makes that expression unique. For example:

-   The fingerprint of a Scan should probably contain a hash of the table name and the pushdown
    predicate.
-   The fingerprint of a Filter should probably contain a hash of its child group ID and predicate.
-   The fingerprint of a Join should probably contain a hash of the left group ID and the right group
    ID, as well as the join predicate.

Note that the above descriptions are slightly inaccurate, and we'll explain why in a later
[section](#fingerprinting--group-merge).

Also, if we have duplicate detection for logical expression, and we do not start physical
optimization until after full plan enumeration, then we do not actually need to do duplicate
detection of physical expressions, since they are derivative of the deduplicated logical
expressions.

### Fingerprint Matching Algorithm

When an expression is added to the memo table, it will first calculate the fingerprint of the
expression. The memo table will compare this fingerprint with every fingerprint in the memo table to
check if we have seen this expression before (in any group). While this is effectively a scan
through every expression, supporting the fingerprint table with an B+tree index will speed up this
operation dramatically (since these fingerprints can be sorted by expression / relation kind).

If there are no identical fingerprints, then there is no duplicate expression, and we can safely
add the expression into the memo table. However, if there are matching fingerprints, we need to
further check for false positives due to hash collisions.

We do full exact match equality checks with every expression that had a fingerprint match. If there
are no exact matches, then we can safely add the expression into the memo table. However, if we find
an exact match (note that there can be at most one exact match since we have an invariant that there
cannot be duplicate expressions), then we know that the expression we are trying to add already
exists in the memo table.

### Fingerprinting + Group Merge

There is a slight problem with the algorithm described above. It does not account for when a child
group has merged into another group.

For example, let's say we have groups 1, 2, and 3. We insert an expression Join(1, 2) into the
memo table with its fingerprint calculated with groups 1 and 2. It is possible that we find out that
groups 2 and 3 need to merged. This means that Join(1, 2) and Join (1, 3) are actually identical
expressions, and the fingerprinting strategies for expressions described above do not handle this.

We will solve this problem by adding allowing multiple fingerprints to reference the same logical
expression, and we will generate a new fingerprint for every expression that is affected by a group
merge / every expression who's parent group now has a new root group.

In the above scenario, we will find every expression in the memo table that has group 2 as a child.
For each expression, we will generate another fingerprint with group 2 "rewritten" as group 3 in the
hash. Note that we _do not_ modify the original expression, we are simply adding another fingerprint
into the memo table.

Finally, we need to handle when multiple groups in a group set are merged into another group set.
For example, if a left group set { 1, 2, 3, 4, 5 } with root 1 needs to be merged into a right group
set { 6, 7, 8, 9, 10 } with root 6, then we need to generate a new fingerprint for every expression
in groups 1, 2, 3, 4, and 5 with group 1 "rewritten" as group 6.

More formally, we are maintaining this invariant:
**For every expression, there exists a fingerprint that maps back to the expression that uses the**
**root groups of their children to calculate the hash.**

For example, if we have a group set { 1, 3, 5 } with root group 1 and group set { 2, 4, 6 } with
root group 2, the fingerprint of Join(5, 4) should really be a fingerprint of Join(1, 2).

This invariant means that when we are checking if some expression already exists, we should use the
root groups of the child groups in our expression to calculate the fingerprint, and we can guarantee
that no fingerprint matches implies no duplicates.

A further implication of this invariant means that new fingerprints need to be generated every time
we merge groups. If we have a left group set { 1, 3, 5 } with root group 1 and right group set
{ 2, 4, 6 } with root group 2, and we merge the first group set into the second, then every
expression that has a child group of 1, 3, or 5 now has a stale fingerprint that uses root group 1
instead of root group 2.

Thus, when we merge the left group into the right group, we need to do the following:

1. Gather the group set, i.e. every single group that has root group 1 (iterate)
2. Retrieve every single expression that has a child group in the group set (via junction table)
3. Generate a new fingerprint for each expression and add it into the memo table

The speed of steps 2 and 3 above are largely dependent on the backing DBMS. However, we can support
step 1 directly in the union find data structure by maintain a circular linked list for every set.
Each group now tracks both a `parent` pointer and a `next` pointer. When merging / unioning a set
into another set, we swap the `next` pointers of the two roots to maintain the circular linked list.
This allows us to do step 1 in linear time relative to the size of the group set.

### Discovered Duplicates

The above algorithm has one more problem: merging groups can cause the memo table to "discover" that
there are duplicate expressions in the memo table.

Here is an example: let's say we have the following groups, each with one expression (note that the
example will work even with multiple expressions):

1. `Scan(1)`
2. `Scan(2)`
3. `Filter(1)`
4. `Filter(2)`
5. `Filter(4)`
6. `Join(3, 4)`
7. `Join(3, 5)`
8. `Sort(6)`
9. `Sort(7)`

Note how groups 5 is just a second filter on top of group 2. Suppose that we find out that
`(Filter(4) = Filter(Filter(2))) == Filter(2)`. In that case, we need to merge groups 4 and 5. The
problem here is that groups 6 and 7 are considered separate groups, but we have now discovered that
they are actually the same. The same is true for groups 8 and 9. In this scenario, the merging of
groups has "generated" a duplicate expression.

However, this is not as big of a problem as it might seem. The issue we want to avoid is lots of
duplicate work or even an infinite loop of rule application. Observe that if we apply a rule to both
the expression in group 6 and group 7 that we will get the same exact expression.

For example, if we apply join commutativity to the expression in group 6 (`Join(3, 4)`), we would
add `Join(4, 3)` into group 6. When we apply join commutativity to the expression in group 7
(`Join(3, 5)`), we would get back `Join(5, 3)`. However, the memo table will detect this as a
duplicate because it will use the root group of 4 and 5 to generate the fingerprint and see that
`Join(4, 3)` already exists. Again, similar logic applies for groups 8 and 9.

At a high level, almost all of our operations are lazy. Work does not need to be done unless it is
absolutely necessary for correctness. By allowing some amount of duplicates, we get some nice
properties with respect to parallelizing memo table access.

## Efficiency and Parallelism

Fingerprinting by itself is very efficient, as creating a fingerprint and looking up a fingerprint
can be made quite efficient with indexes. The real concern here is that merging two groups is very,
very expensive. Depending on the workload, it is both possible that the amortized cost is low or
that group merging takes a majority of the work.

However, we must remember that we want to parallelize access to the memo table. The above algorithms
are notably **read and append only**. There is never a point where we need to update an expression
to maintain invariants. This is important, as it means that we can add and lookup expression and
groups _without having to take any locks_. If we enforce a serializable isolation level, then every
method on the memo table can be done in parallel with relatively low contention due to there being
zero write-write conflicts.
