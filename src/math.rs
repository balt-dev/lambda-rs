
use crate::{define, call, primitives::Composed, Function};
/// Church numeral for zero.
pub type Zero = crate::primitives::Second;

define! {
    /// Successor function. Returns a number plus one.
    /// ```ignore
    /// λn.λz.λs.s(nzs)
    /// ```
    pub fn Successor( N: NUM, S: F, Z: VAL ) where
        N: Function<S>,
        call!(N { S }): Function<Z>,
        S: Function<call!{N { S } { Z }}>
    => call!(S { N { S } { Z } });

    /// Adds two church numerals.
    /// ```ignore
    /// Z ::= Zero
    /// S ::= Successor
    /// λm.λn.mSn
    /// ```
    pub fn Add ( M: LHS, N: RHS ) where
        M: Function<Successor>,
        call!(M { Successor }): Function<N>
    => call!(M { Successor } { N });

    /// Multiplies two church numerals.
    pub fn Multiply ( X: X, Y: Y ) => Composed<X, Y>;

    /// Converts a church numeral to a constant number. See [`ConstNumber`].
    /// ```ignore
    /// λn.n{0}(λ{X}.{X + 1})
    /// ```
    pub fn ToNumber(N: NUM) where
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



#[cfg(test)]
mod test {
    #![allow(dead_code)]
    use crate::math::*;

    type Two = call!( Successor { Successor { Zero } } );
    type Three = call!( Successor { Two } );

    static _TEST: call!(
        ToNumber { Add { Two } { Three } }
    ) = ConstNumber::<5>;

    fn main() {}
}