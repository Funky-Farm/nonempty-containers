/// Creates a [NonEmptyVec] containing the arguments.
///
/// This macro is very similar in goal to the standard library's `vec!` macro:
///
/// - Create a [NonEmptyVec] containing a given list of elements:
///
/// ```
/// # use nonempty_containers::ne;
/// # 
/// let ne = ne![1, 2, 3];
/// assert_eq!(ne[0], 1);
/// assert_eq!(ne[1], 2);
/// assert_eq!(ne[2], 3);
/// ```
///
/// - Create a [NonEmptyVec] from a given element and size:
///
/// ```
/// # use nonempty_containers::{ne, NonEmptyVec};
/// # 
/// let ne = ne![1; 3];
/// assert_eq!(ne, NonEmptyVec::from_vec(vec![1, 1, 1]).unwrap());
/// ```
///
/// Note that unlike [Vec]s, it is not possible to create an empty [NonEmptyVec] using this macro!
#[macro_export]
macro_rules! ne {
    ($elem:expr; $n:expr) => (
        $crate::nonemptyvec::NonEmptyVec::__from_vec_unsafe(vec![$elem; $n])
    );
    ($single:expr) => (
        $crate::nonemptyvec::NonEmptyVec::singleton($single)
    );
    ($head:expr, $($tail:expr),+ $(,)?) => (
        $crate::nonemptyvec::NonEmptyVec::new($head, vec![$($tail),+])
    );
}
