# Duplicate Elimination Memo Table

Note that most of the details are in `src/memo/persistent/implementation.rs`.

For this document, we are assuming that the memo table is backed by a database / ORM. A lot of these
problems would likely not be an issue if everything was in memory.

## Group Merging

During logical exploration, there will be rules that create cycles between groups. The easy solution
for this is to immediately merge two groups together when the engine determines that adding an
expression would result in a duplicate expression from another group.

However, if we want to support parallel exploration, this could be prone to high contention. By
definition, merging group G1 into group G2 would mean that _every expression_ that has a child of
group G1 with would need to be rewritten to point to group G2 instead.

This is unacceptable in a parallel setting, as that would mean every single task that gets affected
would need to either wait for the rewrites to happen before resuming work, or need to abort their
work because data has changed underneath them.

So immediate / eager group merging is not a great idea for parallel exploration. However, if we do
not ever merge two groups that are identical, we are subject to doing duplicate work for every
duplicate expression in the memo table during physical optimization.

Instead of merging groups together immediately, we can instead maintain an auxiliary data structure
that records the groups that _eventually_ need to get merged, and "lazily" merge those groups
together once every group has finished exploration.

## Union-Find Group Sets

We use the well-known Union-Find algorithm and corresponding data structure as the auxiliary data
structure that tracks the to-be-merged groups.

Union-Find supports `Union` and `Find` operations, where `Union` merges sets and `Find` searches for
a "canonical" or "root" element that is shared between all elements in a given set.

For more information about Union-Find, see these
[15-451 lecture notes](https://www.cs.cmu.edu/~15451-f24/lectures/lecture08-union-find.pdf).

Here, we make the elements the groups themselves (really the Group IDs), which allows us to merge
group sets together and also determine a "root group" that all groups in a set can agree on.

When every group in a group set has finished exploration, we can safely begin to merge them
together by moving all expressions from every group in the group set into a single large group.
Other than making sure that any reference to an old group in the group set points to this new large
group, exploration of all groups are done and physical optimization can start.

RFC: Do we need to support incremental search?

Note that since we are now waiting for exploration of all groups to finish, this algorithm is much
closer to the Volcano framework than the Cascades' incremental search. However, since we eventually
will want to store trails / breadcrumbs of decisions made to skip work in the future, and since we
essentially have unlimited space due to the memo table being backed by a DBMS, this is not as much
of a problem.

## Duplicate Detection

TODO explain the fingerprinting algorithm and how it relates to group merging

When taking the fingerprint of an expression, the child groups of an expression need to be root groups. If they are not, we need to try again.
Assuming that all children are root groups, the fingerprint we make for any expression that fulfills that is valid and can be looked up for duplicates.
In order to maintain that correctness, on a merge of two sets, the smaller one requires that a new fingerprint be generated for every expression that has a group in that smaller set.
For example, let's say we need to merge { 1, 2 } (root group 1) with { 3, 4, 5, 6, 7, 8 } (root group 3). We need to find every single expression that has a child group of 1 or 2 and we need to generate a new fingerprint for each where the child groups have been "rewritten" to 3

TODO this is incredibly expensive, but is potentially easily parallelizable?

