use crate::errors::NonEmptyError;
use crate::errors::NonEmptyError::Empty;
use std::collections::btree_set::{IntoIter, Iter};
use std::collections::BTreeSet;

/// An ordered non-empty set type guaranteeing at least one element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NEOrderedSet<T: Ord>(BTreeSet<T>);

impl<T: Ord> NEOrderedSet<T> {
    /// Creates a new non-empty ordered set from one element and optional additional elements.
    pub fn new(head: T, tail: Vec<T>) -> Self {
        let mut set = BTreeSet::new();
        set.insert(head);
        set.extend(tail);
        Self(set)
    }

    /// Creates a singleton ordered non-empty set.
    pub fn singleton(value: T) -> Self {
        let mut set = BTreeSet::new();
        set.insert(value);
        Self(set)
    }

    /// Attempts to create a non-empty ordered set from a BTreeSet.
    /// Returns an error if the provided set is empty.
    pub fn from(set: BTreeSet<T>) -> Result<Self, NonEmptyError> {
        if set.is_empty() {
            Err(Empty)
        } else {
            Ok(Self(set))
        }
    }

    /// Hidden constructor used internally by macros.
    #[doc(hidden)]
    pub fn __from_set_unsafe(set: BTreeSet<T>) -> Self {
        debug_assert!(!set.is_empty());
        Self(set)
    }

    /// Extracts the underlying ordered set.
    pub fn into_set(self) -> BTreeSet<T> {
        self.0
    }

    /// Always returns false since the set is never empty.
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Adds an element. Returns true if the set did not already contain the value.
    pub fn insert(&mut self, value: T) -> bool {
        self.0.insert(value)
    }

    /// Removes an element. Returns true if the set contained the value.
    pub fn remove(&mut self, value: &T) -> bool {
        if self.0.len() == 1 && self.0.contains(&value) {
            false // Prevent removal to maintain non-empty invariant
        } else {
            self.0.remove(&value)
        }
    }

    /// Returns true if the set contains a value.
    pub fn contains(&self, value: &T) -> bool {
        self.0.contains(value)
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T: Ord> From<NEOrderedSet<T>> for BTreeSet<T> {
    fn from(set: NEOrderedSet<T>) -> Self {
        set.into_set()
    }
}

impl<T: Ord> TryFrom<BTreeSet<T>> for NEOrderedSet<T> {
    type Error = NonEmptyError;

    fn try_from(set: BTreeSet<T>) -> Result<Self, Self::Error> {
        Self::from(set)
    }
}

impl<T: Ord> IntoIterator for NEOrderedSet<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T: Ord> IntoIterator for &'a NEOrderedSet<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
