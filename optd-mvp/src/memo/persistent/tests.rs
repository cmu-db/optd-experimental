use crate::{expression::*, memo::persistent::PersistentMemo};

/// Tests that exact expression matches are detected and handled by the memo table.
#[ignore]
#[tokio::test]
async fn test_simple_logical_duplicates() {
    let memo = PersistentMemo::new().await;
    memo.cleanup().await;

    let scan = scan("t1".to_string());
    let scan1a = scan.clone();
    let scan1b = scan.clone();
    let scan2a = scan.clone();
    let scan2b = scan.clone();

    // Insert a new group and its corresponding expression.
    let (group_id, logical_expression_id) = memo.add_group(scan, &[]).await.unwrap().ok().unwrap();

    // Test `add_logical_expression`.
    {
        // Attempting to create a new group with a duplicate expression should fail every time.
        let (group_id_1a, logical_expression_id_1a) =
            memo.add_group(scan1a, &[]).await.unwrap().err().unwrap();
        assert_eq!(group_id, group_id_1a);
        assert_eq!(logical_expression_id, logical_expression_id_1a);

        // Try again just in case...
        let (group_id_1b, logical_expression_id_1b) =
            memo.add_group(scan1b, &[]).await.unwrap().err().unwrap();
        assert_eq!(group_id, group_id_1b);
        assert_eq!(logical_expression_id, logical_expression_id_1b);
    }

    // Test `add_logical_expression_to_group`.
    {
        // Attempting to add a duplicate expression into the same group should also fail every time.
        let (group_id_2a, logical_expression_id_2a) = memo
            .add_logical_expression_to_group(group_id, scan2a, &[])
            .await
            .unwrap()
            .err()
            .unwrap();
        assert_eq!(group_id, group_id_2a);
        assert_eq!(logical_expression_id, logical_expression_id_2a);

        let (group_id_2b, logical_expression_id_2b) = memo
            .add_logical_expression_to_group(group_id, scan2b, &[])
            .await
            .unwrap()
            .err()
            .unwrap();
        assert_eq!(group_id, group_id_2b);
        assert_eq!(logical_expression_id, logical_expression_id_2b);
    }

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
    let memo = PersistentMemo::new().await;
    memo.cleanup().await;

    // Insert a new group and its corresponding expression.
    let scan = scan("t1".to_string());
    let (group_id, _) = memo.add_group(scan, &[]).await.unwrap().ok().unwrap();

    // Insert two identical physical expressions into the _same_ group.
    let table_scan_1 = table_scan("t1".to_string());
    let table_scan_2 = table_scan_1.clone();

    let physical_expression_id_1 = memo
        .add_physical_expression_to_group(group_id, table_scan_1, &[])
        .await
        .unwrap();

    let physical_expression_id_2 = memo
        .add_physical_expression_to_group(group_id, table_scan_2, &[])
        .await
        .unwrap();

    // Since physical expressions do not need duplicate detection,
    assert_ne!(physical_expression_id_1, physical_expression_id_2);

    memo.cleanup().await;
}

/// Tests if the memo tables able to correctly retrieve a group's expressions.
#[ignore]
#[tokio::test]
async fn test_simple_tree() {
    let memo = PersistentMemo::new().await;
    memo.cleanup().await;

    // Create two scan groups.
    let scan1: LogicalExpression = scan("t1".to_string());
    let scan2 = scan("t2".to_string());
    let (scan_id_1, scan_expr_id_1) = memo.add_group(scan1, &[]).await.unwrap().ok().unwrap();
    let (scan_id_2, scan_expr_id_2) = memo.add_group(scan2, &[]).await.unwrap().ok().unwrap();

    assert_eq!(
        memo.get_logical_children(scan_id_1).await.unwrap(),
        &[scan_expr_id_1]
    );
    assert_eq!(
        memo.get_logical_children(scan_id_2).await.unwrap(),
        &[scan_expr_id_2]
    );

    // Create two join expression that should be in the same group.
    // TODO: Eventually, the predicates will be in their own table, and the predicate representation
    // will be a foreign key. For now, we represent them as strings.
    let join1 = join(scan_id_1, scan_id_2, "t1.a = t2.b".to_string());
    let join2 = join(scan_id_2, scan_id_1, "t1.a = t2.b".to_string());

    // Create the group, adding the first expression.
    let (join_id, join_expr_id_1) = memo
        .add_group(join1, &[scan_id_1, scan_id_2])
        .await
        .unwrap()
        .ok()
        .unwrap();
    // Add the second expression.
    let join_expr_id_2 = memo
        .add_logical_expression_to_group(join_id, join2, &[scan_id_2, scan_id_1])
        .await
        .unwrap()
        .ok()
        .unwrap();

    assert_ne!(join_expr_id_1, join_expr_id_2);
    assert_eq!(
        memo.get_logical_children(join_id).await.unwrap(),
        &[join_expr_id_1, join_expr_id_2]
    );

    memo.cleanup().await;
}

/// Tests basic group merging. See comments in the test itself for more information.
#[ignore]
#[tokio::test]
async fn test_simple_group_link() {
    let memo = PersistentMemo::new().await;
    memo.cleanup().await;

    // Create two scan groups.
    let scan1 = scan("t1".to_string());
    let scan2 = scan("t2".to_string());
    let (scan_id_1, _) = memo.add_group(scan1, &[]).await.unwrap().ok().unwrap();
    let (scan_id_2, _) = memo.add_group(scan2, &[]).await.unwrap().ok().unwrap();

    // Create two join expression that should be in the same group.
    // Even though these are obviously the same expression (to humans), the fingerprints will be
    // different, and so they will be put into different groups.
    let join1 = join(scan_id_1, scan_id_2, "t1.a = t2.b".to_string());
    let join2 = join(scan_id_2, scan_id_1, "t2.b = t1.a".to_string());
    let join_unknown = join2.clone();

    let (join_group_1, _) = memo
        .add_group(join1, &[scan_id_1, scan_id_2])
        .await
        .unwrap()
        .ok()
        .unwrap();
    let (join_group_2, join_expr_2) = memo
        .add_group(join2, &[scan_id_2, scan_id_1])
        .await
        .unwrap()
        .ok()
        .unwrap();
    assert_ne!(join_group_1, join_group_2);

    // Assume that some rule was applied to `join1`, and it outputs something like `join_unknown`.
    // The memo table will tell us that `join_unknown == join2`.
    // Take note here that `join_unknown` is a clone of `join2`, not `join1`.
    let (existing_group, not_actually_new_expr_id) = memo
        .add_logical_expression_to_group(join_group_1, join_unknown, &[scan_id_2, scan_id_1])
        .await
        .unwrap()
        .err()
        .unwrap();
    assert_eq!(existing_group, join_group_2);
    assert_eq!(not_actually_new_expr_id, join_expr_2);

    // The above tells the application that the expression already exists in the memo, specifically
    // under `existing_group`. Thus, we should link these two groups together.
    // Here, we arbitrarily choose to link group 1 into group 2.
    memo.update_group_parent(join_group_1, join_group_2)
        .await
        .unwrap();

    let test_root_1 = memo.get_root_group(join_group_1).await.unwrap();
    let test_root_2 = memo.get_root_group(join_group_2).await.unwrap();
    assert_eq!(test_root_1, test_root_2);

    // TODO(Connor)
    //
    // We now need to find all logical expressions that had group 1 (or whatever the root group of
    // the set that group 1 belongs to is, in this case it is just group 1) as a child, and add a
    // new fingerprint for each one that uses group 2 as a child instead.
    //
    // In order to do this, we need to iterate through every group in group 1's set.

    memo.cleanup().await;
}
