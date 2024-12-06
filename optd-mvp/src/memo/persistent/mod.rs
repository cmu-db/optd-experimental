//! This module contains the definition and implementation of the [`PersistentMemo`] type, which
//! implements the `Memo` trait and supports memo table operations necessary for query optimization.

use sea_orm::DatabaseConnection;
use std::marker::PhantomData;

#[cfg(test)]
mod tests;

/// A persistent memo table, backed by a database on disk.
///
/// TODO more docs.
pub struct PersistentMemo<L, P> {
    /// This `PersistentMemo` is reliant on the SeaORM [`DatabaseConnection`] that stores all of the
    /// objects needed for query optimization.
    db: DatabaseConnection,

    /// Generic marker for a generic logical expression.
    _phantom_logical: PhantomData<L>,

    /// Generic marker for a generic physical expression.
    _phantom_physical: PhantomData<P>,
}

mod implementation;
