//! This module contains the definition and implementation of the [`PersistentMemo`] type, which
//! implements the `Memo` trait and supports memo table operations necessary for query optimization.

use sea_orm::DatabaseConnection;

#[cfg(test)]
mod tests;

/// A persistent memo table, backed by a database on disk.
///
/// TODO more docs.
pub struct PersistentMemo {
    /// This `PersistentMemo` is reliant on the SeaORM [`DatabaseConnection`] that stores all of the
    /// objects needed for query optimization.
    db: DatabaseConnection,
}

mod implementation;
