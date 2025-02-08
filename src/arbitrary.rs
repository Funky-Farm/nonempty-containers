//! Implementations for [Arbitrary] for non-empty container types.

use crate::NonEmptyVec;
use arbitrary::{Arbitrary, Unstructured};

impl<'a, T: Arbitrary<'a>> Arbitrary<'a> for NonEmptyVec<T> {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let head = T::arbitrary(u)?;
        let tail = Vec::<T>::arbitrary(u)?;
        Ok(Self::new(head, tail))
    }
}
