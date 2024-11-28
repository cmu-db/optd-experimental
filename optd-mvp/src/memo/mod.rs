//! This module contains items related to the memo table, which is key to the Cascades query
//! optimization framework.
//!
//! TODO more docs.

mod persistent;

mod interface;
pub use interface::{Memo, MemoError};
