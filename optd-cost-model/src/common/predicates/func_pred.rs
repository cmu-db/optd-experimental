#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum FuncType {
    Scalar(datafusion_expr::BuiltinScalarFunction),
    Agg(datafusion_expr::AggregateFunction),
    Case,
}

impl std::fmt::Display for FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FuncType {
    pub fn new_scalar(func_id: datafusion_expr::BuiltinScalarFunction) -> Self {
        FuncType::Scalar(func_id)
    }

    pub fn new_agg(func_id: datafusion_expr::AggregateFunction) -> Self {
        FuncType::Agg(func_id)
    }
}
