//! Implementations for [Arbitrary] for non-empty container types.

use crate::{NESet, NEVec};
use arbitrary::{Arbitrary, Unstructured};
use std::collections::HashSet;
use std::hash::Hash;

impl<'a, T: Arbitrary<'a>> Arbitrary<'a> for NEVec<T> {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let head = T::arbitrary(u)?;
        let mut tail = Vec::<T>::arbitrary(u)?;
        tail.push(head);
        Ok(Self::__from_vec_unsafe(tail))
    }
}

impl<'a, T: Arbitrary<'a> + Eq + Hash> Arbitrary<'a> for NESet<T> {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let head = T::arbitrary(u)?;
        let mut tail = HashSet::<T>::arbitrary(u)?;
        tail.insert(head);
        Ok(Self::__from_set_unsafe(tail))
    }
}
