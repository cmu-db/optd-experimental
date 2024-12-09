use crate::{expression::*, memo::persistent::PersistentMemo};

/// Tests that exact expression matches are detected and handled by the txn table.
#[ignore]
#[tokio::test]
async fn test_simple_logical_duplicates() {
    let memo = PersistentMemo::<DefaultLogicalExpression, DefaultPhysicalExpression>::new().await;
    memo.cleanup().await;

    let mut txn = memo.begin().await.unwrap();

    let scan = scan("t1".to_string());
    let scan1a = scan.clone();
    let scan1b = scan.clone();
    let scan2a = scan.clone();
    let scan2b = scan.clone();

    // Insert a new group and its corresponding expression.
    let (group_id, logical_expression_id) = txn.add_group(scan, &[]).await.unwrap().ok().unwrap();

    // Test `add_logical_expression`.
    {
        // Attempting to create a new group with a duplicate expression should fail every time.
        let (group_id_1a, logical_expression_id_1a) =
            txn.add_group(scan1a, &[]).await.unwrap().err().unwrap();
        assert_eq!(group_id, group_id_1a);
        assert_eq!(logical_expression_id, logical_expression_id_1a);

        // Try again just in case...
        let (group_id_1b, logical_expression_id_1b) =
            txn.add_group(scan1b, &[]).await.unwrap().err().unwrap();
        assert_eq!(group_id, group_id_1b);
        assert_eq!(logical_expression_id, logical_expression_id_1b);
    }

    // Test `add_logical_expression_to_group`.
    {
        // Attempting to add a duplicate expression into the same group should also fail every time.
        let (group_id_2a, logical_expression_id_2a) = txn
            .add_logical_expression_to_group(group_id, scan2a, &[])
            .await
            .unwrap()
            .err()
            .unwrap();
        assert_eq!(group_id, group_id_2a);
        assert_eq!(logical_expression_id, logical_expression_id_2a);

        let (group_id_2b, logical_expression_id_2b) = txn
            .add_logical_expression_to_group(group_id, scan2b, &[])
            .await
            .unwrap()
            .err()
            .unwrap();
        assert_eq!(group_id, group_id_2b);
        assert_eq!(logical_expression_id, logical_expression_id_2b);
    }

    txn.commit().await.unwrap();
    memo.cleanup().await;
}

/// Tests that physical expression are _not_ subject to duplicate detection and elimination.
///
/// !!! Important !!! Note that this behavior should not actually be seen during query optimization,
/// since if logical expression have been deduplicated, there should not be any duplicate physical
/// expressions as they are derivative of the deduplicated logical expressions.
#[ignore]
#[tokio::test]
async fn test_simple_add_physical_expression() {
    let memo = PersistentMemo::<DefaultLogicalExpression, DefaultPhysicalExpression>::new().await;
    memo.cleanup().await;

    let mut txn = memo.begin().await.unwrap();

    // Insert a new group and its corresponding expression.
    let scan = scan("t1".to_string());
    let (group_id, _) = txn.add_group(scan, &[]).await.unwrap().ok().unwrap();

    // Insert two identical physical expressions into the _same_ group.
    let table_scan_1 = table_scan("t1".to_string());
    let table_scan_2 = table_scan_1.clone();

    let physical_expression_id_1 = txn
        .add_physical_expression_to_group(group_id, table_scan_1, &[])
        .await
        .unwrap();

    let physical_expression_id_2 = txn
        .add_physical_expression_to_group(group_id, table_scan_2, &[])
        .await
        .unwrap();

    // Since physical expressions do not need duplicate detection,
    assert_ne!(physical_expression_id_1, physical_expression_id_2);

    txn.commit().await.unwrap();
    memo.cleanup().await;
}

/// Tests if the txn tables able to correctly retrieve a group's expressions.
#[ignore]
#[tokio::test]
async fn test_simple_tree() {
    let memo = PersistentMemo::<DefaultLogicalExpression, DefaultPhysicalExpression>::new().await;
    memo.cleanup().await;

    let mut txn = memo.begin().await.unwrap();

    // Create two scan groups.
    let scan1: DefaultLogicalExpression = scan("t1".to_string());
    let scan2 = scan("t2".to_string());
    let (scan_id_1, scan_expr_id_1) = txn.add_group(scan1, &[]).await.unwrap().ok().unwrap();
    let (scan_id_2, scan_expr_id_2) = txn.add_group(scan2, &[]).await.unwrap().ok().unwrap();

    assert_eq!(
        txn.get_logical_children(scan_id_1).await.unwrap(),
        &[scan_expr_id_1]
    );
    assert_eq!(
        txn.get_logical_children(scan_id_2).await.unwrap(),
        &[scan_expr_id_2]
    );

    // Create two join expression that should be in the same group.
    let join1 = join(scan_id_1, scan_id_2, "t1.a = t2.b".to_string());
    let join2 = join(scan_id_2, scan_id_1, "t1.a = t2.b".to_string());

    // Create the group, adding the first expression.
    let (join_id, join_expr_id_1) = txn
        .add_group(join1, &[scan_id_1, scan_id_2])
        .await
        .unwrap()
        .ok()
        .unwrap();
    // Add the second expression.
    let join_expr_id_2 = txn
        .add_logical_expression_to_group(join_id, join2, &[scan_id_2, scan_id_1])
        .await
        .unwrap()
        .ok()
        .unwrap();

    assert_ne!(join_expr_id_1, join_expr_id_2);
    assert_eq!(
        txn.get_logical_children(join_id).await.unwrap(),
        &[join_expr_id_1, join_expr_id_2]
    );

    txn.commit().await.unwrap();
    memo.cleanup().await;
}

/// Tests a single group merge. See comments in the test itself for more information.
#[ignore]
#[tokio::test]
async fn test_simple_group_link() {
    let memo = PersistentMemo::<DefaultLogicalExpression, DefaultPhysicalExpression>::new().await;
    memo.cleanup().await;

    let mut txn = memo.begin().await.unwrap();

    // Create two scan groups.
    let scan1 = scan("t1".to_string());
    let scan2 = scan("t2".to_string());
    let (scan_id_1, _) = txn.add_group(scan1, &[]).await.unwrap().ok().unwrap();
    let (scan_id_2, _) = txn.add_group(scan2, &[]).await.unwrap().ok().unwrap();

    // Create two join expression that should be in the same group.
    // Even though these are obviously the same expression (to humans), the fingerprints will be
    // different, and so they will be put into different groups.
    let join1 = join(scan_id_1, scan_id_2, "t1.a = t2.b".to_string());
    let join2 = join(scan_id_2, scan_id_1, "t2.b = t1.a".to_string());
    let join_unknown = join2.clone();

    let (join_group_1, _) = txn
        .add_group(join1, &[scan_id_1, scan_id_2])
        .await
        .unwrap()
        .ok()
        .unwrap();
    let (join_group_2, join_expr_2) = txn
        .add_group(join2, &[scan_id_2, scan_id_1])
        .await
        .unwrap()
        .ok()
        .unwrap();
    assert_ne!(join_group_1, join_group_2);

    // Assume that some rule was applied to `join1`, and it outputs something like `join_unknown`.
    // The txn table will tell us that `join_unknown == join2`.
    // Take note here that `join_unknown` is a clone of `join2`, not `join1`.
    let (existing_group, not_actually_new_expr_id) = txn
        .add_logical_expression_to_group(join_group_1, join_unknown, &[scan_id_2, scan_id_1])
        .await
        .unwrap()
        .err()
        .unwrap();
    assert_eq!(existing_group, join_group_2);
    assert_eq!(not_actually_new_expr_id, join_expr_2);

    // The above tells the application that the expression already exists in the txn, specifically
    // under `existing_group`. Thus, we should link these two groups together.
    txn.merge_groups(join_group_1, join_group_2).await.unwrap();

    let test_root_1 = txn.get_root_group(join_group_1).await.unwrap();
    let test_root_2 = txn.get_root_group(join_group_2).await.unwrap();
    assert_eq!(test_root_1, test_root_2);

    txn.commit().await.unwrap();
    memo.cleanup().await;
}

/// Tests merging groups up a chain.
#[ignore]
#[tokio::test]
async fn test_group_merge_ladder() {
    let memo = PersistentMemo::<DefaultLogicalExpression, DefaultPhysicalExpression>::new().await;
    memo.cleanup().await;

    let mut txn = memo.begin().await.unwrap();

    // Build up a tree of true filters that should be collapsed into a single table scan.
    let scan_base = scan("t1".to_string());
    let (scan_id, _) = txn.add_group(scan_base, &[]).await.unwrap().ok().unwrap();

    let filter0 = filter(scan_id, "true".to_string());
    let (filter_id_0, _) = txn
        .add_group(filter0, &[scan_id])
        .await
        .unwrap()
        .ok()
        .unwrap();

    let filter1 = filter(filter_id_0, "true".to_string());
    let (filter_id_1, _) = txn
        .add_group(filter1, &[scan_id])
        .await
        .unwrap()
        .ok()
        .unwrap();

    let filter2 = filter(filter_id_1, "true".to_string());
    let (filter_id_2, _) = txn
        .add_group(filter2, &[scan_id])
        .await
        .unwrap()
        .ok()
        .unwrap();

    let filter3 = filter(filter_id_2, "true".to_string());
    let (filter_id_3, _) = txn
        .add_group(filter3, &[scan_id])
        .await
        .unwrap()
        .ok()
        .unwrap();

    let mut groups = vec![scan_id, filter_id_0, filter_id_1, filter_id_2, filter_id_3];

    let m0 = txn.merge_groups(filter_id_3, filter_id_2).await.unwrap();
    let m1 = txn.merge_groups(filter_id_2, filter_id_1).await.unwrap();
    let m2 = txn.merge_groups(filter_id_1, filter_id_0).await.unwrap();
    let root = txn.merge_groups(filter_id_0, scan_id).await.unwrap();
    groups.extend_from_slice(&[m0, m1, m2, root]);

    for group_id in groups {
        assert_eq!(root, txn.get_root_group(group_id).await.unwrap());
    }

    txn.commit().await.unwrap();
    memo.cleanup().await;
}

/// Tests merging a bunch of groups together in order to prevent duplicates from being added.
#[ignore]
#[tokio::test]
async fn test_group_merge() {
    let memo = PersistentMemo::<DefaultLogicalExpression, DefaultPhysicalExpression>::new().await;
    memo.cleanup().await;

    let mut txn = memo.begin().await.unwrap();

    // Create a base group.
    let scan1 = scan("t1".to_string());
    let (scan_id_1, _) = txn.add_group(scan1, &[]).await.unwrap().ok().unwrap();

    // Create a bunch of equivalent groups.
    let filter0 = filter(scan_id_1, "true".to_string());
    let filter1 = filter(scan_id_1, "1 < 2".to_string());
    let filter2 = filter(scan_id_1, "2 > 1".to_string());
    let filter3 = filter(scan_id_1, "42 != 100".to_string());
    let filter4 = filter(scan_id_1, "10000 > 0".to_string());
    let filter5 = filter(scan_id_1, "1 + 2 = 3".to_string());
    let filter6 = filter(scan_id_1, "true OR false".to_string());
    let filter7 = filter(scan_id_1, "(1 + 1 > -1 AND true) OR false".to_string());
    let (filter_id_0, _) = txn
        .add_group(filter0, &[scan_id_1])
        .await
        .unwrap()
        .ok()
        .unwrap();
    let (filter_id_1, _) = txn
        .add_group(filter1, &[scan_id_1])
        .await
        .unwrap()
        .ok()
        .unwrap();
    let (filter_id_2, _) = txn
        .add_group(filter2, &[scan_id_1])
        .await
        .unwrap()
        .ok()
        .unwrap();
    let (filter_id_3, _) = txn
        .add_group(filter3, &[scan_id_1])
        .await
        .unwrap()
        .ok()
        .unwrap();
    let (filter_id_4, _) = txn
        .add_group(filter4, &[scan_id_1])
        .await
        .unwrap()
        .ok()
        .unwrap();
    let (filter_id_5, _) = txn
        .add_group(filter5, &[scan_id_1])
        .await
        .unwrap()
        .ok()
        .unwrap();
    let (filter_id_6, _) = txn
        .add_group(filter6, &[scan_id_1])
        .await
        .unwrap()
        .ok()
        .unwrap();
    let (filter_id_7, _) = txn
        .add_group(filter7, &[scan_id_1])
        .await
        .unwrap()
        .ok()
        .unwrap();
    let filters = vec![
        filter_id_0,
        filter_id_1,
        filter_id_2,
        filter_id_3,
        filter_id_4,
        filter_id_5,
        filter_id_6,
        filter_id_7,
    ];

    // Merge them all together.
    let quarter_0 = txn.merge_groups(filters[0], filters[1]).await.unwrap();
    let quarter_1 = txn.merge_groups(filters[2], filters[3]).await.unwrap();
    let quarter_2 = txn.merge_groups(filters[4], filters[5]).await.unwrap();
    let quarter_3 = txn.merge_groups(filters[6], filters[7]).await.unwrap();
    let semi_0 = txn.merge_groups(quarter_0, quarter_1).await.unwrap();
    let semi_1 = txn.merge_groups(quarter_2, quarter_3).await.unwrap();
    let final_id = txn.merge_groups(semi_0, semi_1).await.unwrap();

    // Check that the group set is properly representative.
    {
        let set = txn.get_group_set(final_id).await.unwrap();
        assert_eq!(set.len(), 8);
        for id in set {
            assert!(filters.contains(&id));
        }
    }

    // Create another base group.
    let scan2 = scan("t2".to_string());
    let (scan_id_2, _) = txn.add_group(scan2, &[]).await.unwrap().ok().unwrap();

    // Add a join group.
    let join0 = join(filter_id_0, scan_id_2, "t1.a = t2.a".to_string());
    let (join_group_id, join_expr_id) = txn
        .add_group(join0, &[filter_id_0, scan_id_2])
        .await
        .unwrap()
        .ok()
        .unwrap();

    // Adding the duplicate join expressions should return a duplication error containing the IDs of
    // the already existing group and expression.
    for filter_id in filters {
        let join_test = join(filter_id, scan_id_2, "t1.a = t2.a".to_string());
        let (join_group_id_test, join_expr_id_test) = txn
            .add_group(join_test, &[filter_id, scan_id_2])
            .await
            .unwrap()
            .err()
            .unwrap();
        assert_eq!(join_group_id, join_group_id_test);
        assert_eq!(join_expr_id, join_expr_id_test);
    }

    txn.commit().await.unwrap();
    memo.cleanup().await;
}

/// Tests the exact same scenario as in the "Discovered Duplicates" section in `DESIGN.md`.
#[ignore]
#[tokio::test]
async fn test_cascading_merge() {
    let memo = PersistentMemo::<DefaultLogicalExpression, DefaultPhysicalExpression>::new().await;
    memo.cleanup().await;

    let mut txn = memo.begin().await.unwrap();

    // Create the base groups.
    let scan1 = scan("t1".to_string());
    let (g1, _) = txn.add_group(scan1, &[]).await.unwrap().ok().unwrap();
    let scan2 = scan("t2".to_string());
    let (g2, _) = txn.add_group(scan2, &[]).await.unwrap().ok().unwrap();

    let filter1 = filter(g1, "x > 1000".to_string());
    let (g3, _) = txn.add_group(filter1, &[g1]).await.unwrap().ok().unwrap();

    // Create two groups that will need to be merged.
    let filter2a = filter(g2, "a < 42".to_string());
    let (g4, _) = txn.add_group(filter2a, &[g2]).await.unwrap().ok().unwrap();
    let filter2b = filter(g4, "a < 42 AND 1 = 1".to_string());
    let (g5, _) = txn.add_group(filter2b, &[g4]).await.unwrap().ok().unwrap();

    // Create groups that are dependent on the to-be-merged groups.
    let join1 = join(g3, g4, "t1.x = t2.a".to_string());
    let (g6, _) = txn.add_group(join1, &[g3, g4]).await.unwrap().ok().unwrap();
    let join2 = join(g3, g5, "t1.x = t2.a".to_string());
    let (g7, _) = txn.add_group(join2, &[g3, g5]).await.unwrap().ok().unwrap();

    // Create more groups that are dependent on the to-be-merged groups.
    // TODO actually use a sort expression instead of a `filter` placeholder.
    let sort1 = filter(g6, "ORDER BY a".to_string());
    let (g8, _) = txn.add_group(sort1, &[g6]).await.unwrap().ok().unwrap();

    let sort2 = filter(g7, "ORDER BY a".to_string());
    let (g9, _) = txn.add_group(sort2, &[g7]).await.unwrap().ok().unwrap();

    // Now that everything is set up, we can merge groups 4 and 5 to begin the cascading process.
    let filter_root = txn.merge_groups(g4, g5).await.unwrap();
    assert_eq!(txn.get_root_group(g4).await.unwrap(), filter_root);
    assert_eq!(txn.get_root_group(g5).await.unwrap(), filter_root);

    // After merging, the join groups (6 and 7) are technically identical, but we have not merged
    // them together yet. However, applying rules will reveal that they are identical, and we will
    // know that they need to get merged.
    let join1_commute = join(g4, g3, "t1.x = t2.a".to_string());
    let join1_commute_id = txn
        .add_logical_expression_to_group(g6, join1_commute, &[g4, g3])
        .await
        .unwrap()
        .ok()
        .unwrap();

    // Adding this expression should now result in a duplication error and return the above ID.
    let join2_commute = join(g5, g3, "t1.x = t2.a".to_string());
    let (existing_g6, existing_id) = txn
        .add_logical_expression_to_group(g7, join2_commute, &[g5, g3])
        .await
        .unwrap()
        .err()
        .unwrap();
    assert_eq!(existing_g6, g6);
    assert_eq!(existing_id, join1_commute_id);

    // Since the txn table has told us these are duplicates, we can now merge groups 6 and 7.
    let join_root = txn.merge_groups(g6, g7).await.unwrap();
    assert_eq!(txn.get_root_group(g6).await.unwrap(), join_root);
    assert_eq!(txn.get_root_group(g7).await.unwrap(), join_root);

    // Do a similar thing for the sort groups. We'll skip the expression adding for now and just
    // merge them immediately, but remember that the application should observe a duplicate
    // somewhere in the txn table before deciding to merge groups.
    let sort_root = txn.merge_groups(g8, g9).await.unwrap();
    assert_eq!(txn.get_root_group(g8).await.unwrap(), sort_root);
    assert_eq!(txn.get_root_group(g9).await.unwrap(), sort_root);

    txn.commit().await.unwrap();
    memo.cleanup().await;
}
