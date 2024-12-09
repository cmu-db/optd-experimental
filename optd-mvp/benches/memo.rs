use optd_mvp::{
    expression::{logical_expression::LogicalExpression, physical_expression::PhysicalExpression},
    memo::{persistent::PersistentMemo, GroupId},
};

/// The state we need to run a benchmark.
pub struct BenchmarkScenario<L, P> {
    pub name: String,
    pub ops: Vec<MemoOp<L, P>>,
}

/// Different types of operations we can do on the memo table.
#[derive(Debug, Clone)]
pub enum MemoOp<L, P> {
    AddGroup {
        expr: L,
        children: Vec<GroupId>,
    },
    MergeGroups {
        left: GroupId,
        right: GroupId,
    },
    AddLogicalExpression {
        group_id: GroupId,
        expr: L,
        children: Vec<GroupId>,
    },
    AddPhysicalExpression {
        group_id: GroupId,
        expr: P,
        children: Vec<GroupId>,
    },
}

impl<L, P> BenchmarkScenario<L, P> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ops: Vec::new(),
        }
    }

    pub fn add_op(mut self, op: MemoOp<L, P>) -> Self {
        self.ops.push(op);
        self
    }

    pub fn add_group(self, expr: L, children: Vec<GroupId>) -> Self {
        self.add_op(MemoOp::AddGroup { expr, children })
    }

    pub fn merge_groups(self, left: GroupId, right: GroupId) -> Self {
        self.add_op(MemoOp::MergeGroups { left, right })
    }

    pub fn add_logical_expression(
        self,
        group_id: GroupId,
        expr: L,
        children: Vec<GroupId>,
    ) -> Self {
        self.add_op(MemoOp::AddLogicalExpression {
            group_id,
            expr,
            children,
        })
    }

    pub fn add_physical_expression(
        self,
        group_id: GroupId,
        expr: P,
        children: Vec<GroupId>,
    ) -> Self {
        self.add_op(MemoOp::AddPhysicalExpression {
            group_id,
            expr,
            children,
        })
    }

    // Generic runner for benchmarks.
    pub async fn run(&self, memo: &PersistentMemo<L, P>)
    where
        L: LogicalExpression,
        P: PhysicalExpression,
    {
        for op in &self.ops {
            match op {
                MemoOp::AddGroup { expr, children } => {
                    // We do not care about whether this operation succeded or failed.
                    let _ = memo.add_group(expr.clone(), children).await.unwrap();
                }
                MemoOp::MergeGroups { left, right } => {
                    memo.merge_groups(*left, *right).await.unwrap();
                }
                MemoOp::AddLogicalExpression {
                    group_id,
                    expr,
                    children,
                } => {
                    // We do not care about whether this operation succeded or failed.
                    let _ = memo
                        .add_logical_expression_to_group(*group_id, expr.clone(), children)
                        .await
                        .unwrap();
                }
                MemoOp::AddPhysicalExpression {
                    group_id,
                    expr,
                    children,
                } => {
                    memo.add_physical_expression_to_group(*group_id, expr.clone(), children)
                        .await
                        .unwrap();
                }
            }
        }
    }
}

// Generic benchmark runner
// pub fn run_benchmarks<L, P>(c: &mut Criterion, scenarios: Vec<BenchmarkScenario<L, P>>)
// where
//     L: LogicalExpression + Send + 'static,
//     P: PhysicalExpression + Send + 'static,
// {
//     let rt = Runtime::new().unwrap();

//     for scenario in scenarios {
//         c.bench_function(&scenario.name, |b| {
//             b.to_async(&rt).iter(|| async {
//                 let memo = PersistentMemo::<L, P>::new().await;
//                 memo.cleanup().await;
//                 scenario.run(&memo).await;
//                 memo.cleanup().await;
//             });
//         });
//     }
// }
