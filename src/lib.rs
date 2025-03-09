//! Non-empty containers.
//!
//! Non-emptiness is generally a very useful tool, when you need inherent guarantees in code but
//! want to avoid repeatedly writing the same checks. This module provides non-empty versions of
//! common container types, such as [Vec].

pub mod ne_vec;
pub use ne_vec::NEVec;

pub mod ne_set;
pub use ne_set::NESet;

pub mod ne_ordered_set;
pub use ne_ordered_set::NEOrderedSet;

#[cfg(feature = "arbitrary")]
mod arbitrary;

#[macro_use]
mod macros;
mod errors;
