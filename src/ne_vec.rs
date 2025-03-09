//! A non-empty vector type that guarantees at least one element is present. [NEVec] has an
//! interface similar to [Vec] with additional methods to enforce the invariant. Get started with:
//!
//! ```rust, no_run
//! # use nonempty_containers::{nev, NEVec};
//! #
//! let nev = NEVec::new(42, vec![1, 2, 3]);
//! let singleton = NEVec::singleton(42);
//! let r#macro = nev![1, 2, 3];
//! ```
//!
//! [NEVec] conforms to [Index], [IntoIterator], [Deref], and many more, so operations are
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
//! When the feature `arbitrary` is enabled, [NEVec] implements [arbitrary::Arbitrary]
//! for generation of randomly populated instances.

use crate::errors::NonEmptyError;
#[cfg(feature = "im")]
use im::Vector;
use std::collections::vec_deque::IntoIter;
use std::collections::vec_deque::{Iter, IterMut};
use std::collections::VecDeque;
use std::ops::Index;

/// Non-empty vector type.
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct NEVec<T>(VecDeque<T>);

impl<T> NEVec<T> {
    /// Creates a new [NEVec], ensuring at least one element is present.
    pub fn new(head: T, tail: Vec<T>) -> Self {
        // We can afford to call [Vec::len()] here because it's O(1).
        let mut vec = VecDeque::from(tail);
        vec.push_front(head);
        Self(vec)
    }

    /// Creates a new singleton [NEVec]. Semantically equivalent to:
    /// ```no_run
    /// # use nonempty_containers::NEVec;
    /// # let value = 42;
    /// #
    /// NEVec::new(value, Vec::new());
    /// ```
    pub fn singleton(value: T) -> Self {
        let mut vec = VecDeque::new();
        vec.push_back(value);
        Self(vec)
    }

    /// Returns the first element. This operation is safe as the invariant guarantees at least one
    /// element is present.
    pub fn head(&self) -> &T {
        self.0.front().expect("[NonEmptyVec] invariant violated.")
    }

    /// Returns all elements except the last one. This may be empty if the [NEVec] is a
    /// singleton.
    pub fn init(&self) -> Iter<'_, T> {
        self.0.range(..self.0.len() - 1)
    }

    /// Returns all elements except the first one. This may be empty if the [NEVec] is a
    /// singleton.
    pub fn tail(&self) -> Iter<'_, T> {
        self.0.range(1..self.0.len())
    }

    /// Returns the last element. This operation is safe as the invariant guarantees at least one
    /// element is present.
    pub fn last(&self) -> &T {
        self.0.back().expect("[NonEmptyVec] invariant violated.")
    }

    /// Attempts to create a [NEVec] from a [Vec], returning [None] if the [Vec] is empty.
    /// ```rust
    /// # use nonempty_containers::NEVec;
    /// #
    /// assert!(NEVec::from_vec(vec![42]).is_ok());
    /// assert!(NEVec::from_vec(Vec::<u32>::new()).is_err());
    /// ```
    pub fn from_vec(vec: Vec<T>) -> Result<Self, NonEmptyError> {
        match vec.is_empty() {
            true => Err(NonEmptyError::Empty),
            false => Ok(Self(VecDeque::from(vec))),
        }
    }

    /// Attempts to create a [NEVec] from a [VecDeque], returning [None] if the [VecDeque] is
    /// empty.
    ///
    /// ```rust
    /// # use std::collections::VecDeque;
    /// # use nonempty_containers::NEVec;
    /// #
    /// assert!(NEVec::from_deque(VecDeque::from(vec![42])).is_ok());
    /// assert!(NEVec::from_deque(VecDeque::<u32>::new()).is_err());
    /// ```
    pub fn from_deque(deque: VecDeque<T>) -> Result<Self, NonEmptyError> {
        match deque.is_empty() {
            true => Err(NonEmptyError::Empty),
            false => Ok(Self(deque)),
        }
    }

    /// Attempts to create a [NEVec] from a [Vector], returning [None] if the [Vector] is
    /// empty. This is only available when the `im` feature is enabled. Additionally, [Vector]
    /// enforces that the element type must conform to [Clone].
    ///
    /// ```rust
    /// # use nonempty_containers::NEVec;
    /// #
    /// assert!(NEVec::from_vector(im::vector![42]).is_ok());
    /// assert!(NEVec::from_vector(im::Vector::<u32>::new()).is_err());
    /// ```
    #[cfg(feature = "im")]
    pub fn from_vector(vector: Vector<T>) -> Result<Self, NonEmptyError>
    where
        T: Clone,
    {
        match vector.is_empty() {
            true => Err(NonEmptyError::Empty),
            false => Ok(Self(VecDeque::from_iter(vector.into_iter()))),
        }
    }

    /// Creates a new [NEVec] from a [Vec] without checking if it's empty. This operation is
    /// unsafe and should only be used by macros in this crate!
    #[doc(hidden)]
    pub fn __from_vec_unsafe(vec: Vec<T>) -> Self {
        debug_assert!(!vec.is_empty());
        Self::from_vec(vec).unwrap()
    }

    /// Creates a new [NEVec] from a [VecDeque] without checking if it's empty. This operation
    /// is unsafe and should only be used by macros in this crate!
    #[doc(hidden)]
    pub fn __from_deque_unsafe(deque: VecDeque<T>) -> Self {
        debug_assert!(!deque.is_empty());
        Self::from_deque(deque).unwrap()
    }

    /// Creates a new [NEVec] from a [Vector] without checking if it's empty. This operation
    /// is unsafe and should only be used by macros in this crate!
    #[doc(hidden)]
    #[cfg(feature = "im")]
    pub fn __from_vector_unsafe(vector: Vector<T>) -> Self
    where
        T: Clone,
    {
        debug_assert!(!vector.is_empty());
        Self::from_vector(vector).unwrap()
    }

    /// Returns the length of this [NEVec].
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// A [NEVec] is always non-empty.
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Returns this [NEVec] as a slice.
    pub fn as_slice(&mut self) -> &[T] {
        self.0.make_contiguous();
        self.0.as_slices().0
    }

    /// Pushes an element to the front of the [NEVec].
    pub fn push_front(&mut self, value: T) {
        self.0.push_front(value);
    }

    /// Pushes an element to the back of the [NEVec].
    pub fn push_back(&mut self, value: T) {
        self.0.push_back(value);
    }

    /// Tries to remove the first element.
    pub fn pop_front(&mut self) -> Result<T, NonEmptyError> {
        match self.0.len() {
            0 => Err(NonEmptyError::Empty),
            1 => Err(NonEmptyError::AlreadySingleton),
            _ => Ok(self
                .0
                .pop_front()
                .expect("[NonEmptyVec] invariant violated.")),
        }
    }

    /// Tries to remove the last element.
    pub fn pop_back(&mut self) -> Result<T, NonEmptyError> {
        match self.0.len() {
            0 => Err(NonEmptyError::Empty),
            1 => Err(NonEmptyError::AlreadySingleton),
            _ => Ok(self
                .0
                .pop_back()
                .expect("[NonEmptyVec] invariant violated.")),
        }
    }

    /// Splits the [NEVec] into the first element and the rest. This operation is guaranteed
    /// to succeed because the invariant guarantees at least one element is present.
    pub fn split_first(&self) -> (&T, Iter<'_, T>) {
        (self.head(), self.tail())
    }

    /// Splits the [NEVec] into all elements except the last one and the last element. This
    /// operation is guaranteed to succeed because the invariant guarantees at least one element is
    /// present.
    pub fn split_last(&self) -> (Iter<'_, T>, &T) {
        (self.init(), self.last())
    }

    /// Like [NEVec::split_first], but consumes the [NEVec].
    pub fn take_split_first(self) -> (T, IntoIter<T>) {
        let mut iter = self.0.into_iter();
        let head = iter.next().expect("[NonEmptyVec] invariant violated.");
        (head, iter)
    }

    /// Like [NEVec::split_last], but consumes the [NEVec].
    pub fn take_split_last(self) -> (IntoIter<T>, T) {
        let mut iter = self.0.into_iter();
        let last = iter.next_back().expect("[NonEmptyVec] invariant violated.");
        (iter, last)
    }
    
    /// Returns an iterator over the elements of the [NEVec].
    pub fn iter(&self) -> Iter<'_, T> {
        self.0.iter()
    }
}

impl<T> From<NEVec<T>> for Vec<T> {
    fn from(ne: NEVec<T>) -> Self {
        ne.0.into()
    }
}

impl<T> From<NEVec<T>> for VecDeque<T> {
    fn from(ne: NEVec<T>) -> Self {
        ne.0
    }
}

#[cfg(feature = "im")]
impl<T> From<NEVec<T>> for Vector<T>
where
    T: Clone,
{
    fn from(ne: NEVec<T>) -> Self {
        Vector::from_iter(ne.0.into_iter())
    }
}

impl<T> TryFrom<Vec<T>> for NEVec<T> {
    type Error = NonEmptyError;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        NEVec::from_vec(vec)
    }
}

impl<T> From<(T, Vec<T>)> for NEVec<T> {
    fn from(value: (T, Vec<T>)) -> Self {
        let (head, tail) = value;
        Self::new(head, tail)
    }
}

impl<T> From<(T, VecDeque<T>)> for NEVec<T> {
    fn from(value: (T, VecDeque<T>)) -> Self {
        let (head, tail) = value;
        Self::new(head, Vec::from(tail))
    }
}

#[cfg(feature = "im")]
impl<T: Clone> From<(T, Vector<T>)> for NEVec<T> {
    fn from(value: (T, Vector<T>)) -> Self {
        let (head, tail) = value;
        Self::new(head, Vec::from_iter(tail.into_iter()))
    }
}

impl<T> From<T> for NEVec<T> {
    fn from(value: T) -> Self {
        Self::singleton(value)
    }
}

impl<'a, T> IntoIterator for &'a NEVec<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut NEVec<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T> IntoIterator for NEVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Index<usize> for NEVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
