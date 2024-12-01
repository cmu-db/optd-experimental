//! In-memory representation of logical and physical expression / operators / relations.
//!
//! TODO more docs.

mod logical_expression;
pub use logical_expression::*;

mod physical_expression;
pub use physical_expression::*;

/// The representation of an expression.
///
/// TODO more docs.
#[derive(Clone, Debug)]
pub enum Expression {
    Logical(LogicalExpression),
    Physical(PhysicalExpression),
}

/// Converts the database / JSON representation of a logical expression into an in-memory one.
impl From<crate::entities::logical_expression::Model> for Expression {
    fn from(value: crate::entities::logical_expression::Model) -> Self {
        Self::Logical(value.into())
    }
}

/// Converts the in-memory representation of a logical expression into the database / JSON version.
///
/// # Panics
///
/// This will panic if the [`Expression`] is [`Expression::Physical`].
impl From<Expression> for crate::entities::logical_expression::Model {
    fn from(value: Expression) -> Self {
        let Expression::Logical(expr) = value else {
            panic!("Attempted to convert an in-memory physical expression into a logical database / JSON expression");
        };

        expr.into()
    }
}

/// Converts the database / JSON representation of a physical expression into an in-memory one.
impl From<crate::entities::physical_expression::Model> for Expression {
    fn from(value: crate::entities::physical_expression::Model) -> Self {
        Self::Physical(value.into())
    }
}

/// Converts the in-memory representation of a physical expression into the database / JSON version.
///
/// # Panics
///
/// This will panic if the [`Expression`] is [`Expression::Physical`].
impl From<Expression> for crate::entities::physical_expression::Model {
    fn from(value: Expression) -> Self {
        let Expression::Physical(expr) = value else {
            panic!("Attempted to convert an in-memory logical expression into a physical database / JSON expression");
        };

        expr.into()
    }
}
