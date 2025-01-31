use std::{error::Error, fmt::{Debug, Display, Formatter}, ops::Not};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct NonemptyVec<T>(Vec<T>);

#[derive(Debug)]
pub struct VecEmptyError;

impl Display for VecEmptyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec is empty")
    }
}

impl Error for VecEmptyError {}

impl <T> NonemptyVec<T> {
    /// Creates a singleton [NonemptyVec] from the given element.
    /// 
    /// * `value` - The first value in the [NonemptyVec].
    pub fn singleton(value: T) -> Self {
        Self([value].into())
    }

    /// Creates a non-empty vector with exactly one element.
    /// Alias for [NonemptyVec::singleton].
    /// 
    /// * `value` - The first value in the [NonemptyVec].
    pub fn new(value: T) -> Self {
        Self::singleton(value)
    }

    /// Returns `true` if and only if the [NonemptyVec] has exactly one element.
    pub fn is_singleton(&self) -> bool {
        self.0.len() == 1
    }

    /// Returns `true` if and only if the [NonemptyVec] satisfies its invariant of being nonempty.
    fn is_valid(&self) -> bool {
        self.0.is_empty()
            .not()
    }

    /// Attempts to convert from a `Vec` to a [NonemptyVec].
    /// Succeeds if and only if the given `Vec` is nonempty.
    pub fn from_vec(vec: Vec<T>) -> Result<Self, VecEmptyError> {
        Some(Self(vec))
            .filter(Self::is_valid)
            .ok_or(VecEmptyError)
    }

    /// Converts from a `Vec` to a [NonemptyVec], panicking on failure.
    /// 
    /// # Safety
    /// Panics if `vec` is empty.
    pub fn from_vec_asserted(vec: Vec<T>) -> Self {
        let res = Self(vec);
        assert!(res.is_valid());
        res
    }

    /// Converts from a `Vec` to a [NonemptyVec], assuming that the `vec` is nonempty.
    /// 
    /// # Safety
    /// If `vec` is indeed empty, then this method can cause UB when modifying the [NonemptyVec].
    pub unsafe fn from_vec_unchecked(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

impl <T> IntoIterator for NonemptyVec<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl <T> From<T> for NonemptyVec<T> {
    fn from(value: T) -> Self {
        Self(Vec::from([value]))
    }
}

impl <T> From<(T, Vec<T>)> for NonemptyVec<T> {
    fn from(value: (T, Vec<T>)) -> Self {
        let (head, tail) = value;
        let mut total = Vec::from([head]);
        total.extend(tail);
        Self(total)
    }
}

impl <T> TryFrom<Vec<T>> for NonemptyVec<T> {
    type Error = VecEmptyError;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        Self::from_vec(vec)
    }
}

impl <T> From<NonemptyVec<T>> for Vec<T> {
    fn from(value: NonemptyVec<T>) -> Self {
        value.0
    }
}

impl <T> AsRef<[T]> for NonemptyVec<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl <T> AsMut<[T]> for NonemptyVec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}