/// Filter that determines that particular event
/// is suitable for particular handler.
pub trait Filter<T> {
    /// Passes (return true) if event is suitable (otherwise return false)
    fn test(&self, value: &T) -> bool;
}

/// ```
/// use async_telegram_bot::dispatcher::filter::Filter;
///
/// let closure = |i: &i32| -> bool { *i >= 42 };
/// assert!(closure.test(&42));
/// assert!(closure.test(&100));
///
/// assert_eq!(closure.test(&41), false);
/// assert_eq!(closure.test(&0), false);
/// ```
impl<T, F: Fn(&T) -> bool> Filter<T> for F {
    fn test(&self, value: &T) -> bool {
        (self)(value)
    }
}

/// ```
/// use async_telegram_bot::dispatcher::filter::Filter;
///
/// assert!(true.test(&()));
/// assert_eq!(false.test(&()), false);
/// ```
impl<T> Filter<T> for bool {
    fn test(&self, _: &T) -> bool { *self }
}

/// And filter.
///
/// Passes if both underlying filters pass.
///
/// **NOTE**: if one of filters don't pass
/// it is **not** guaranteed that other will be executed.
///
/// ## Examples
/// ```
/// use async_telegram_bot::dispatcher::filter::{And, Filter};
///
/// // Note: bool can be treated as `Filter` that always return self.
/// assert_eq!(And::new(true, false).test(&()), false);
/// assert_eq!(And::new(true, false).test(&()), false);
/// assert!(And::new(true, true).test(&()));
/// assert!(And::new(true, And::new(|_: &()| true, true)).test(&()));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct And<A, B>(A, B);

impl<A, B> And<A, B> {
    pub fn new(a: A, b: B) -> Self {
        And(a, b)
    }
}

impl<T, A, B> Filter<T> for And<A, B>
where
    A: Filter<T>,
    B: Filter<T>,
{
    fn test(&self, value: &T) -> bool {
        self.0.test(value) && self.1.test(value)
    }
}

/// Alias for [`And::new`]
///
/// ## Examples
/// ```
/// use async_telegram_bot::dispatcher::filter::{and, Filter};
///
/// assert!(and(true, true).test(&()));
/// assert_eq!(and(true, false).test(&()), false);
/// ```
///
/// [`And::new`]: crate::dispatcher::filter::And::new
pub fn and<A, B>(a: A, b: B) -> And<A, B> {
    And::new(a, b)
}


/// Or filter.
///
/// Passes if at least one underlying filters passes.
///
/// **NOTE**: if one of filters passes
/// it is **not** guaranteed that other will be executed.
///
/// ## Examples
/// ```
/// use async_telegram_bot::dispatcher::filter::{Or, Filter};
///
/// // Note: bool can be treated as `Filter` that always return self.
/// assert!(Or::new(true, false).test(&()));
/// assert!(Or::new(false, true).test(&()));
/// assert!(Or::new(false, Or::new(|_: &()| true, false)).test(&()));
/// assert_eq!(Or::new(false, false).test(&()), false);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Or<A, B>(A, B);

impl<A, B> Or<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Or(a, b)
    }
}

impl<T, A, B> Filter<T> for Or<A, B>
where
    A: Filter<T>,
    B: Filter<T>,
{
    fn test(&self, value: &T) -> bool {
        self.0.test(value) || self.1.test(value)
    }
}

/// Alias for [`Or::new`]
///
/// ## Examples
/// ```
/// use async_telegram_bot::dispatcher::filter::{or, Filter};
///
/// assert!(or(true, false).test(&()));
/// assert_eq!(or(false, false).test(&()), false);
/// ```
///
/// [`Or::new`]: crate::dispatcher::filter::Or::new
pub fn or<A, B>(a: A, b: B) -> Or<A, B> {
    Or::new(a, b)
}


/// Not filter.
///
/// Passes if underlying filter don't pass.
///
/// ## Examples
/// ```
/// use async_telegram_bot::dispatcher::filter::{Not, Filter};
///
/// // Note: bool can be treated as `Filter` that always return self.
/// assert!(Not::new(false).test(&()));
/// assert_eq!(Not::new(true).test(&()), false);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Not<A>(A);

impl<A> Not<A> {
    pub fn new(a: A) -> Self {
        Not(a)
    }
}

impl<T, A> Filter<T> for Not<A>
where
    A: Filter<T>,
{
    fn test(&self, value: &T) -> bool {
        !self.0.test(value)
    }
}

/// Alias for [`Not::new`]
///
/// ## Examples
/// ```
/// use async_telegram_bot::dispatcher::filter::{not, Filter};
///
/// assert!(not(false).test(&()));
/// assert_eq!(not(true).test(&()), false);
/// ```
///
/// [`Not::new`]: crate::dispatcher::filter::Not::new
pub fn not<A>(a: A) -> Not<A> {
    Not::new(a)
}

/// Return [filter] that passes if and only if all given filters passes.
///
/// **NOTE**: if one of filters don't pass
/// it is **not** guaranteed that other will be executed.
///
/// ## Examples
/// ```
/// use async_telegram_bot::{all, dispatcher::filter::Filter};
///
/// assert!(all![true].test(&()));
/// assert!(all![true, true].test(&()));
/// assert!(all![true, true, true].test(&()));
///
/// assert_eq!(all![false].test(&()), false);
/// assert_eq!(all![true, false].test(&()), false);
/// assert_eq!(all![false, true].test(&()), false);
/// assert_eq!(all![false, false].test(&()), false);
/// ```
///
/// [filter]: crate::dispatcher::filter::Filter
#[macro_export]
macro_rules! all {
    ($one:expr) => { $one };
    ($head:expr, $($tail:tt)+) => {
        $crate::dispatcher::filter::And::new(
            $head,
            $crate::all!($($tail)+)
        )
    };
}

/// Return [filter] that passes if and only if any given filters passes.
///
/// **NOTE**: if one of filters passes
/// it is **not** guaranteed that other will be executed.
///
/// ## Examples
/// ```
/// use async_telegram_bot::{any, dispatcher::filter::Filter};
///
/// assert!(any![true].test(&()));
/// assert!(any![true, true].test(&()));
/// assert!(any![false, true].test(&()));
/// assert!(any![true, false, true].test(&()));
///
/// assert_eq!(any![false].test(&()), false);
/// assert_eq!(any![false, false].test(&()), false);
/// assert_eq!(any![false, false, false].test(&()), false);
/// ```
///
/// [filter]: crate::dispatcher::filter::Filter
#[macro_export]
macro_rules! any {
    ($one:expr) => { $one };
    ($head:expr, $($tail:tt)+) => {
        $crate::dispatcher::filter::Or::new(
            $head,
            $crate::all!($($tail)+)
        )
    };
}
