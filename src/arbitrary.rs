

impl <'a, T: Arbitrary<'a>> Arbitrary<'a> for NonEmptyVec<T> {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let head = T::arbitrary(u)?;
        let tail = Vec::<T>::arbitrary(u)?;
        Ok(Self::new(head, tail))
    }
}