use crate::{
    boolean::{And, False, True},
    define,
    primitives::{Composed, Constant, Identity},
    Function,
};

/// Church numeral for zero.
pub type Zero = crate::primitives::SecondOf;

define! {
    /// Successor function. Returns a number plus one.
    /// ```ignore
    /// λn.λf.λx.f(nfx)
    /// ```
    pub fn Successor ::= {
        N. F. X. { F, { N, F, X }}
    } where
        N: F,
        {N, F}: X,
        F: {N, F, X};

    /// Adds two church numerals.
    /// ```ignore
    /// S ::= Successor
    /// λm.λn.mSn
    /// ```
    pub fn Add ::= { M. N. { M, Successor, N }} where
        M: Successor,
        {M, Successor}: N;

    /// Multiplies two church numerals - this is simply composition.
    pub fn Multiply ::= { X. Y. { Composed<X, Y> }};

    /// Predecessor function. Gets the number below a given church numeral.
    /// ```ignore
    /// λn.λf.λx. n (λg.λh. h (g f)) (λu.x) (λu.u)
    /// ```
    pub fn Predecessor ::= { N. F. X. { N, Pred_1<F>, Constant<X>, Identity }} where
        N: (Pred_1<F>),
        {N, Pred_1<F>}: (Constant<X>),
        {N, Pred_1<F>, Constant<X>}: Identity;

    fn Pred_1<F> ::= { G. H. { H, { G, F }}} where
        H: {G, F},
        G: F;
    
    /// Subtracts two church numerals.
    /// ```ignore
    /// P ::= Predecessor
    /// λm.λn.nPm
    /// ```
    pub fn Subtract ::= { M. N. { N, Predecessor, M }} where
        N: Predecessor,
        {N, Predecessor}: M;
    
    /// Raises a numeral to the power of another.
    /// ```ignore
    /// λm.λn.nm
    /// ```
    pub fn Exponent ::= { M. N. { N, M }} where N: M;


    /// Returns whether a number is zero.
    /// ```ignore
    /// F ::= False
    /// T ::= True
    /// λn.n(λx.F)T
    /// ```
    pub fn IsZero ::= { N. { N, Constant<False>, True }} where
        N: (Constant<False>),
        {N, Constant<False>}: True;
    
    
    /// Returns whether one number is less than or equal to another.
    /// ```ignore
    /// ? ::= IsZero
    /// - ::= Subtract
    /// λm.λn.?(-mn)
    /// ```
    pub fn Leq ::= { M. N. { IsZero, { Subtract, M, N }}} where
        Subtract: M,
        {Subtract, M}: N,
        IsZero: { Subtract, M, N };
    
    
    /// Returns whether two numbers are equal.
    /// ```ignore
    /// & ::= And
    /// ≤ ::= Leq
    /// λm.λn.&(≤mn)(≤nm)
    /// ```
    pub fn Eq ::= { M. N. { And, { Leq, M, N }, { Leq, N, M }}} where
        Leq: M, Leq: N,
        {Leq, M}: N, {Leq, N}: M,
        And: {Leq, M, N},
        {And, { Leq, M, N }}: { Leq, N, M };

    /// Converts a church numeral to a constant number. See [`ConstNumber`].
    /// ```ignore
    /// λn.n{0}(λ{X}.{X + 1})
    /// ```
    pub fn ToNumber ::= { N. { N, ConstIncrement, ConstNumber<0> }} where
        N: ConstIncrement,
        ConstIncrement: (ConstNumber<0>),
        {N, ConstIncrement}: (ConstNumber<0>);
}

// Due to const generics, this has to be explicitly declared.
/// Increments a [`ConstNumber`] by one. Used to define [`ToNumber`].
/// ```ignore
/// λ{X}.{X + 1}
/// ```
pub struct ConstIncrement;
/// Constant number returned by converting a church numeral.
pub struct ConstNumber<const N: u64>;

impl<const N: u64> Function<ConstNumber<N>> for ConstIncrement
where
    ConstNumber<{ N + 1 }>: Sized,
{
    type Output = ConstNumber<{ N + 1 }>;
}

#[cfg(test)]
mod test {
    #![allow(dead_code)]
    use crate::prelude::*;

    type Two = call!{Successor, { Successor, Zero }};
    type Three = call!{Successor, Two};

    #[test]
    fn main() {
        let _: call!{
            Eq, Three, Three
        } = <True>::default();
    }
}
