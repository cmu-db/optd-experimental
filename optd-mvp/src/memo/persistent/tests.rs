use super::*;
use crate::{expression::*, memo::Memo};

/// Tests is exact expression matches are detected and handled by the memo table.
#[ignore]
#[tokio::test]
async fn test_simple_duplicates() {
    let memo = PersistentMemo::new().await;
    memo.cleanup().await;

    let scan = scan("(a int, b int)".to_string());
    let scan1 = scan.clone();
    let scan2 = scan.clone();

    let res0 = memo
        .add_logical_expression(scan.into(), &[])
        .await
        .unwrap()
        .ok();
    let res1 = memo
        .add_logical_expression(scan1.into(), &[])
        .await
        .unwrap()
        .err();
    let res2 = memo
        .add_logical_expression(scan2.into(), &[])
        .await
        .unwrap()
        .err();

    assert_eq!(res0, res1);
    assert_eq!(res0, res2);
    assert_eq!(res1, res2);

    memo.cleanup().await;
}
