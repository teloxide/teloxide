/// Filter that determines that particular event
/// is suitable for particular handler.
pub trait Filter<T> {
    /// Passes (return true) if event is suitable (otherwise return false)
    fn test(&self, value: &T) -> bool;
}

/// ```
/// use telebofr::dispatching::filter::Filter;
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
/// use telebofr::dispatching::filter::Filter;
///
/// assert!(true.test(&()));
/// assert_eq!(false.test(&()), false);
/// ```
impl<T> Filter<T> for bool {
    fn test(&self, _: &T) -> bool {
        *self
    }
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
/// use telebofr::dispatching::filter::{And, Filter};
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
/// use telebofr::dispatching::filter::{and, Filter};
///
/// assert!(and(true, true).test(&()));
/// assert_eq!(and(true, false).test(&()), false);
/// ```
///
/// [`And::new`]: crate::dispatching::filter::And::new
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
/// use telebofr::dispatching::filter::{Filter, Or};
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
/// use telebofr::dispatching::filter::{or, Filter};
///
/// assert!(or(true, false).test(&()));
/// assert_eq!(or(false, false).test(&()), false);
/// ```
///
/// [`Or::new`]: crate::dispatching::filter::Or::new
pub fn or<A, B>(a: A, b: B) -> Or<A, B> {
    Or::new(a, b)
}

/// Not filter.
///
/// Passes if underlying filter don't pass.
///
/// ## Examples
/// ```
/// use telebofr::dispatching::filter::{Filter, Not};
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
/// use telebofr::dispatching::filter::{not, Filter};
///
/// assert!(not(false).test(&()));
/// assert_eq!(not(true).test(&()), false);
/// ```
///
/// [`Not::new`]: crate::dispatching::filter::Not::new
pub fn not<A>(a: A) -> Not<A> {
    Not::new(a)
}

/// Return [filter] that passes if and only if all of the given filters passes.
///
/// **NOTE**: if one of filters don't pass
/// it is **not** guaranteed that other will be executed.
///
/// ## Examples
/// ```
/// use telebofr::{all, dispatching::filter::Filter};
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
/// [filter]: crate::dispatching::filter::Filter
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

/// Return [filter] that passes if any of the given filters passes.
///
/// **NOTE**: if one of filters passes
/// it is **not** guaranteed that other will be executed.
///
/// ## Examples
/// ```
/// use telebofr::{any, dispatching::filter::Filter};
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
/// [filter]: crate::dispatching::filter::Filter
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

/// Simple wrapper around `Filter` that adds `|` and `&` operators.
///
/// ## Examples
/// ```
/// use telebofr::dispatching::filter::{f, And, Filter, Or, F};
///
/// let flt1 = |i: &i32| -> bool { *i > 17 };
/// let flt2 = |i: &i32| -> bool { *i < 42 };
/// let flt3 = |i: &i32| -> bool { *i % 2 == 0 };
///
/// let and = f(flt1) & flt2;
/// assert!(and.test(&19)); // both filters pass
///
/// assert_eq!(and.test(&50), false); // `flt2` doesn't pass
/// assert_eq!(and.test(&16), false); // `flt1` doesn't pass
///
/// let or = f(flt1) | flt3;
/// assert!(or.test(&19)); // `flt1` passes
/// assert!(or.test(&16)); // `flt2` passes
/// assert!(or.test(&20)); // both pass
///
/// assert_eq!(or.test(&17), false); // both don't pass
///
/// // Note: only first filter in chain should be wrapped in `f(...)`
/// let complicated: F<Or<And<_, _>, _>> = f(flt1) & flt2 | flt3;
/// assert!(complicated.test(&2)); // `flt3` passes
/// assert!(complicated.test(&21)); // `flt1` and `flt2` pass
///
/// assert_eq!(complicated.test(&15), false); // `flt1` and `flt3` don't pass
/// assert_eq!(complicated.test(&43), false); // `flt2` and `flt3` don't pass
/// ```
pub struct F<A>(A);

/// Constructor fn for [F]
///
/// [F]: crate::dispatching::filter::F;
pub fn f<A>(a: A) -> F<A> {
    F(a)
}

impl<T, A> Filter<T> for F<A>
where
    A: Filter<T>,
{
    fn test(&self, value: &T) -> bool {
        self.0.test(value)
    }
}

impl<A, B> std::ops::BitAnd<B> for F<A> {
    type Output = F<And<A, B>>;

    fn bitand(self, other: B) -> Self::Output {
        f(and(self.0, other))
    }
}

impl<A, B> std::ops::BitOr<B> for F<A> {
    type Output = F<Or<A, B>>;

    fn bitor(self, other: B) -> Self::Output {
        f(or(self.0, other))
    }
}

/* workaround for `E0207` compiler error */
/// Extensions for filters
pub trait FilterExt<T> {
    /// Alias for [`Not::new`]
    ///
    /// ## Examples
    /// ```
    /// use telebofr::dispatching::filter::{Filter, FilterExt};
    ///
    /// let flt = |i: &i32| -> bool { *i > 0 };
    /// let flt = flt.not();
    /// assert!(flt.test(&-1));
    /// assert_eq!(flt.test(&1), false);
    /// ```
    ///
    /// [`Not::new`]: crate::dispatching::filter::Not::new
    fn not(self) -> Not<Self>
    where
        Self: Sized,
    {
        Not::new(self)
    }

    /// Alias for [`And::new`]
    ///
    /// ## Examples
    /// ```
    /// use telebofr::dispatching::filter::{Filter, FilterExt};
    ///
    /// let flt = |i: &i32| -> bool { *i > 0 };
    /// let flt = flt.and(|i: &i32| *i < 42);
    ///
    /// assert!(flt.test(&1));
    /// assert_eq!(flt.test(&-1), false);
    /// assert_eq!(flt.test(&43), false);
    /// ```
    ///
    /// [`Not::new`]: crate::dispatching::filter::And::new
    fn and<B>(self, other: B) -> And<Self, B>
    where
        Self: Sized,
    {
        And::new(self, other)
    }

    /// Alias for [`Or::new`]
    ///
    /// ## Examples
    /// ```
    /// use telebofr::dispatching::filter::{Filter, FilterExt};
    ///
    /// let flt = |i: &i32| -> bool { *i < 0 };
    /// let flt = flt.or(|i: &i32| *i > 42);
    ///
    /// assert!(flt.test(&-1));
    /// assert!(flt.test(&43));
    /// assert_eq!(flt.test(&17), false);
    /// ```
    ///
    /// [`Not::new`]: crate::dispatching::filter::Or::new
    fn or<B>(self, other: B) -> Or<Self, B>
    where
        Self: Sized,
    {
        Or::new(self, other)
    }
}

// All methods implemented via defaults
impl<T, F> FilterExt<T> for F where F: Filter<T> {}
