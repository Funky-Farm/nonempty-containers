//! Non-empty containers.
//!
//! Non-emptiness is generally a very useful tool, when you need inherent guarantees in code but
//! want to avoid repeatedly writing the same checks. This module provides non-empty versions of
//! common container types, such as [Vec].

pub mod nonemptyvec;
pub use nonemptyvec::NonEmptyVec;

pub mod nonemptyset;
pub use nonemptyset::NonEmptySet;

#[cfg(feature = "arbitrary")]
mod arbitrary;

#[macro_use]
mod macros;
mod errors;
