//! Error types for the non-empty types.

use crate::NonEmptyVec;

/// Errors that can occur when working with non-empty types..
#[derive(Debug)]
pub enum NonEmptyError {
    /// Encountered an empty collection when it was expected to be non-empty.
    Empty,

    /// Attempted to remove an element from a singleton [NonEmptyVec].
    AlreadySingleton,
}