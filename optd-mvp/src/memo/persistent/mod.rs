//! This module contains the definition and implementation of the [`PersistentMemo`] type, which
//! implements the `Memo` trait and supports memo table operations necessary for query optimization.

use crate::{
    entities::{prelude::*, *},
    DATABASE_URL,
};
use sea_orm::*;

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

impl PersistentMemo {
    /// Creates a new `PersistentMemo` struct by connecting to a database defined at
    /// [`DATABASE_URL`].
    ///
    /// TODO remove dead code and write docs.
    #[allow(dead_code)]
    pub async fn new() -> Self {
        Self {
            db: Database::connect(DATABASE_URL).await.unwrap(),
        }
    }

    /// Since there is no asynchronous drop yet in Rust, we must do this manually.
    ///
    /// TODO remove dead code and write docs.
    #[allow(dead_code)]
    pub async fn cleanup(&self) {
        cascades_group::Entity::delete_many()
            .exec(&self.db)
            .await
            .unwrap();
        fingerprint::Entity::delete_many()
            .exec(&self.db)
            .await
            .unwrap();
        logical_expression::Entity::delete_many()
            .exec(&self.db)
            .await
            .unwrap();
        logical_children::Entity::delete_many()
            .exec(&self.db)
            .await
            .unwrap();
        physical_expression::Entity::delete_many()
            .exec(&self.db)
            .await
            .unwrap();
        physical_children::Entity::delete_many()
            .exec(&self.db)
            .await
            .unwrap();
    }
}

mod implementation;
