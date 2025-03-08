//! A non-empty set type that guarantees at least one element is present. [NonEmptySet] has an 
//! interface similar to [HashSet] with additional methods to enforce the invariant. Get started
//! with:
//!
//! ```rust, no_run
//! # use nonempty_containers::{nes, NonEmptySet};
//! #
//! let nes = NonEmptySet::new(42, vec![1, 2, 3]);
//! let singleton = NonEmptySet::singleton(42);
//! let r#macro = nes![1, 2, 3];
//! ```
//!
//! [NonEmptySet] conforms to [Index], [IntoIterator], [Deref], and many more, so operations are
//! as [HashSet]-like as possible. They are also usually zero-cost.
//!
//! ```rust, no_run
//! # use nonempty_containers::{nes, NonEmptySet};
//! #
//! let nes = nes![42, 1, 2, 3];
//! assert!(nes.contains(&42));
//! assert_eq!(nes.len(), 4);
//! ```
//!
//! When the feature `arbitrary` is enabled, [NonEmptySet] implements [arbitrary::Arbitrary]
//! for generation of randomly populated instances.

use std::collections::hash_set::{Iter, IntoIter};
use std::collections::HashSet;
use std::hash::Hash;
use crate::errors::NonEmptyError;

/// Non-empty set type.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NonEmptySet<T: Eq + Hash>(HashSet<T>);

impl<T: Eq + Hash> NonEmptySet<T> {
    /// Creates a new [NonEmptySet], ensuring at least one element is present.
    pub fn new(head: T, tail: Vec<T>) -> Self {
        let mut set = HashSet::with_capacity(1 + tail.len());
        set.insert(head);
        set.extend(tail);
        Self(set)
    }
    
    /// Creates a new singleton [NonEmptySet]. Semantically equivalent to:
    /// ```no_run
    /// # use nonempty_containers::NonEmptySet;
    /// # let value = 42;
    /// #
    /// NonEmptySet::new(value, Vec::new());
    /// ```
    pub fn singleton(value: T) -> Self {
        let mut set = HashSet::new();
        set.insert(value);
        Self(set)
    }
    
    /// Creates a new [NonEmptySet] from a [HashSet]. Returns an error if the set is empty.
    pub fn from(set: HashSet<T>) -> Result<Self, NonEmptyError> {
        match set.is_empty() {
            true => Err(NonEmptyError::Empty),
            false => Ok(Self(set)),
        }
    }
    
    /// Creates a new [NonEmptySet] from a [HashSet] without checking the invariant. This is unsafe
    /// and should only be used by macros in this crate.
    #[doc(hidden)]
    pub fn __from_set_unsafe(set: HashSet<T>) -> Self {
        debug_assert!(!set.is_empty());
        Self(set)
    }
    
    /// Extracts the underlying [HashSet]. This operation is zero-cost.
    pub fn into_set(self) -> HashSet<T> {
        self.0
    }
    
    /// Returns the size of the set.
    pub fn len(&self) -> usize {
        self.0.len()
    }
    
    /// A [NonEmptySet] is always non-empty.
    pub fn is_empty(&self) -> bool {
        false
    }
    
    /// Adds an element to the set. If the element is already present, it is not modified.
    pub fn insert(&mut self, value: T) -> bool {
        self.0.insert(value)
    }
    
    /// Removes an element from the set. Returns `true` if the element was present.
    pub fn remove(&mut self, value: &T) -> bool {
        self.0.remove(value)
    }
    
    /// Checks if the set contains a value.
    pub fn contains(&self, value: &T) -> bool {
        self.0.contains(value)
    }
}

impl<T: Eq + Hash> From<NonEmptySet<T>> for HashSet<T> {
    fn from(value: NonEmptySet<T>) -> Self {
        value.into_set()
    }
}

impl<T: Eq + Hash> TryFrom<HashSet<T>> for NonEmptySet<T> {
    type Error = NonEmptyError;

    fn try_from(set: HashSet<T>) -> Result<Self, Self::Error> {
        NonEmptySet::from(set)
    }
}


impl<T: Eq + Hash> From<T> for NonEmptySet<T> {
    fn from(value: T) -> Self {
        Self::singleton(value)
    }
}

impl<'a, T: Eq + Hash> IntoIterator for &'a NonEmptySet<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<T: Eq + Hash> IntoIterator for NonEmptySet<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
