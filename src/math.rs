
use crate::Function;
/// Church numeral for zero.
pub type Zero = crate::primitives::Second;

define! {
    /// Successor function. Returns a number plus one.
    /// ```ignore
    /// λn.λf.λx.nf(fx)
    /// ```
    pub fn Successor ( N: NUM, Fn: FUNC, Input: I ) where 
        N: Function<Fn>,
        call!(N { Fn }): Function<Input>,
        Fn: Function<Input>,
        Fn: Function<call!(Fn { Input })>,
        call!(N { Fn }): Function<call! { Fn { Input } }>
    => call!(N { Fn } { Fn { Input } });


    /// Converts a church numeral to a constant number. See [`ConstNumber`].
    /// ```ignore
    /// λn.n(λ{X}.{X + 1}){0}
    /// ```
    pub fn ToNumber (N: NUM) where
        N: Function<ConstIncrement>,
        call!(N { ConstIncrement }): Function<ConstNumber<0>>
    => call!(N { ConstIncrement } { ConstNumber<0> });
}

// Due to const generics, this has to be explicitly declared.
/// Increments a [`ConstNumber`] by one. Used to define [`ToNumber`].
/// ```ignore
/// λ{X}.{X + 1}
/// ```
pub struct ConstIncrement;
/// Constant number returned by converting a church numeral.
pub struct ConstNumber<const N: u64>;

impl<const N: u64> Function<ConstNumber<N>> for ConstIncrement where ConstNumber<{N + 1}>: Sized {
    type Output = ConstNumber<{N + 1}>;
}