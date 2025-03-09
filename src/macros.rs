//! Macros for creating non-empty containers.

/// Creates a [NonEmptyVec] containing the arguments.
///
/// This macro is very similar in goal to the standard library's `vec!` macro:
///
/// - Create a [NonEmptyVec] containing a given list of elements:
///
/// ```
/// # use nonempty_containers::nev;
/// # 
/// let ne = nev![1, 2, 3];
/// assert_eq!(ne[0], 1);
/// assert_eq!(ne[1], 2);
/// assert_eq!(ne[2], 3);
/// ```
///
/// - Create a [NonEmptyVec] from a given head element and tail vector.
///
/// ```
/// # use nonempty_containers::{nev, NonEmptyVec};
/// # 
/// let vec = vec![2, 3, 4];
/// let ne = nev![1; vec];
/// assert_eq!(ne, NonEmptyVec::from_vec(vec![1, 2, 3, 4]).unwrap());
/// ```
///
/// Note that unlike [Vec]s, it is not possible to create an empty [NonEmptyVec] using this macro!
#[macro_export]
macro_rules! nev {
    ($elem:expr; $n:ident) => (
        $crate::NonEmptyVec::new($elem, $n)
    );
    ($single:expr) => (
        $crate::NonEmptyVec::singleton($single)
    );
    ($head:expr, $($tail:expr),+ $(,)?) => (
        $crate::NonEmptyVec::new($head, vec![$($tail),+])
    );
}

#[macro_export]
macro_rules! nes {
    ($elem:expr; $n:expr) => (
        $crate::nonemptyset::NonEmptySet::__from_set_unsafe(std::iter::once($elem).chain(std::iter::repeat($elem).take($n - 1)).collect())
    );
    ($single:expr) => (
        $crate::nonemptyset::NonEmptySet::singleton($single)
    );
    ($head:expr, $($tail:expr),+ $(,)?) => (
        $crate::nonemptyset::NonEmptySet::new($head, vec![$($tail),+])
    );
}
