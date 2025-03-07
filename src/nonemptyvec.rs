//! A non-empty vector type that guarantees at least one element is present. [NonEmptyVec] has an
//! interface similar to [Vec] with additional methods to enforce the invariant. Get started with:
//!
//! ```rust, no_run
//! # use nonempty_containers::{nev, NonEmptyVec};
//! #
//! let nev = NonEmptyVec::new(42, vec![1, 2, 3]);
//! let singleton = NonEmptyVec::singleton(42);
//! let r#macro = nev![1, 2, 3];
//! ```
//!
//! [NonEmptyVec] conforms to [Index], [IntoIterator], [Deref], and many more, so operations are
//! as [Vec]-like as possible. They are also usually zero-cost.
//!
//! ```rust, no_run
//! # use nonempty_containers::nev;
//! #
//! let nev = nev![42, 1, 2, 3];
//! assert_eq!(nev[0], 42);
//! assert_eq!(nev.len(), 4);
//! assert_eq!(nev.into_iter().sum::<i32>(), 48);
//! ```
//!
//! When the feature `arbitrary` is enabled, [NonEmptyVec] implements [arbitrary::Arbitrary]
//! for generation of randomly populated instances.

use std::ops::{Deref, Index};
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;
use crate::errors::NonEmptyError;

/// Non-empty vector type.
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct NonEmptyVec<T>(Vec<T>);

impl<T> NonEmptyVec<T> {
    /// Creates a new [NonEmptyVec], ensuring at least one element is present.
    pub fn new(head: T, tail: Vec<T>) -> Self {
        // We can afford to call [Vec::len()] here because it's O(1).
        let mut vec = Vec::with_capacity(1 + tail.len());
        vec.push(head);
        vec.extend(tail);
        Self(vec)
    }

    /// Creates a new singleton [NonEmptyVec]. Semantically equivalent to:
    /// ```no_run
    /// # use nonempty_containers::NonEmptyVec;
    /// # let value = 42;
    /// #
    /// NonEmptyVec::new(value, Vec::new());
    /// ```
    pub fn singleton(value: T) -> Self {
        Self(vec![value])
    }

    /// Returns the first element. This operation is safe as the invariant guarantees at least one
    /// element is present.
    pub fn head(&self) -> &T {
        &self.0[0]
    }

    /// Returns the initial elements. This slice may be empty if the [NonEmptyVec] is a singleton.
    pub fn init(&self) -> &[T] {
        &self.0[..self.0.len() - 1]
    }

    /// Returns the tail as a slice. This slice may be empty if the [NonEmptyVec] is a singleton.
    pub fn tail(&self) -> &[T] {
        &self.0[1..]
    }

    /// Returns the last element. This operation is safe as the invariant guarantees at least one
    /// element is present.
    pub fn last(&self) -> &T {
        self.0.last().unwrap()
    }

    /// Attempts to create a [NonEmptyVec] from a [Vec], returning [None] if the [Vec] is empty.
    /// ```rust
    /// # use nonempty_containers::NonEmptyVec;
    /// #
    /// assert!(NonEmptyVec::from_vec(vec![42]).is_ok());
    /// assert!(NonEmptyVec::from_vec(Vec::<u32>::new()).is_err());
    /// ```
    pub fn from_vec(vec: Vec<T>) -> Result<Self, NonEmptyError> {
        match vec.is_empty() {
            true => Err(NonEmptyError::Empty),
            false => Ok(Self(vec)),
        }
    }
    
    /// Creates a new [NonEmptyVec] from a [Vec] without checking if it's empty. This operation is
    /// unsafe and should only be used by macros in this crate!
    #[doc(hidden)]
    pub fn __from_vec_unsafe(vec: Vec<T>) -> Self {
        debug_assert!(!vec.is_empty());
        Self(vec)
    }

    /// Extracts the inner [Vec], consuming [self]. This operation is zero-cost.
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    /// Returns the length of this [NonEmptyVec].
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// A [NonEmptyVec] is always non-empty.
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Returns this [NonEmptyVec] as a slice.
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    /// Appends an element.
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    /// Tries to remove the last element, returning [NonEmptyError::AlreadySingleton] if the
    /// [NonEmptyVec] would become empty.
    pub fn pop(&mut self) -> Result<T, NonEmptyError> {
        match self.0.len() {
            0 => Err(NonEmptyError::Empty),
            1 => Err(NonEmptyError::AlreadySingleton),
            _ => Ok(self.0.pop().unwrap()),
        }
    }
}

impl<T> From<NonEmptyVec<T>> for Vec<T> {
    fn from(ne: NonEmptyVec<T>) -> Self {
        ne.0
    }
}

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = NonEmptyError;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        NonEmptyVec::from_vec(vec)
    }
}

impl<T> From<(T, Vec<T>)> for NonEmptyVec<T> {
    fn from(value: (T, Vec<T>)) -> Self {
        let (head, tail) = value;
        Self::new(head, tail)
    }
}

impl<T> From<T> for NonEmptyVec<T> {
    fn from(value: T) -> Self {
        Self::singleton(value)
    }
}

impl<T> Deref for NonEmptyVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> IntoIterator for &'a NonEmptyVec<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut NonEmptyVec<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T> IntoIterator for NonEmptyVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Index<usize> for NonEmptyVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
