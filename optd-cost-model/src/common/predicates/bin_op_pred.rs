/// TODO: documentation
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum BinOpType {
    // numerical
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // comparison
    Eq,
    Neq,
    Gt,
    Lt,
    Geq,
    Leq,
}

impl std::fmt::Display for BinOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl BinOpType {
    pub fn is_numerical(&self) -> bool {
        matches!(
            self,
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Mod
        )
    }

    pub fn is_comparison(&self) -> bool {
        matches!(
            self,
            Self::Eq | Self::Neq | Self::Gt | Self::Lt | Self::Geq | Self::Leq
        )
    }
}
