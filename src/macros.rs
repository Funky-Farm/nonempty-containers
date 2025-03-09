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
/// # use nonempty_containers::{nev, NEVec};
/// #
/// let vec = vec![2, 3, 4];
/// let ne = nev![1; vec];
/// assert_eq!(ne, NEVec::from_vec(vec![1, 2, 3, 4]).unwrap());
/// ```
///
/// Note that unlike [Vec]s, it is not possible to create an empty [NonEmptyVec] using this macro!
#[macro_export]
macro_rules! nev {
    ($elem:expr; $n:expr) => (
        $crate::NEVec::new($elem, $n)
    );
    ($single:expr) => (
        $crate::NEVec::singleton($single)
    );
    ($head:expr, $($tail:expr),+ $(,)?) => (
        $crate::NEVec::new($head, vec![$($tail),+])
    );
}

#[macro_export]
macro_rules! nes {
    ($elem:expr; $n:expr) => (
        $crate::NESet::new($elem, $n)
    );
    ($single:expr) => (
        $crate::NESet::singleton($single)
    );
    ($head:expr, $($tail:expr),+ $(,)?) => (
        $crate::NESet::new($head, vec![$($tail),+])
    );
}

#[macro_export]
macro_rules! neos {
    ($elem:expr; $n:expr) => (
        $crate::NEOrderedSet::new($elem, $n)
    );
    ($single:expr) => (
        $crate::NEOrderedSet::singleton($single)
    );
    ($head:expr, $($tail:expr),+ $(,)?) => (
        $crate::NEOrderedSet::new($head, vec![$($tail),+])
    );
}
