use crate::{
    call, define,
    primitives::{self, Composed},
    Function,
};

/// Truthy church boolean.
pub type True = primitives::First;
/// Falsy church boolean.
pub type False = primitives::Second;

define! {
    /// Boolean and operation. Takes two church booleans, and returns whether both are true.
    /// ```ignore
    /// λa.λb.aba
    /// ```
    pub fn And ( Lhs: L, Rhs: R ) => Composed<Lhs, Composed<Rhs, Lhs>>;
    /// Boolean or operation. Takes two church booleans, and returns whether either is true.
    /// ```ignore
    /// λa.λb.aab
    /// ```
    pub fn Or ( Lhs: L, Rhs: R ) => Composed<Lhs, Composed<Lhs, Rhs>>;
    /// Boolean not operation. Takes a church boolean and returns its inverse.
    /// ```ignore
    /// λv.λt.λf.vft
    /// ```
    pub fn Not ( Predicate: P, Lhs: L, Rhs: R ) where
        Predicate: Function<Rhs>,
        call!(Predicate { Rhs }): Function<Lhs>
    => call!( Predicate { Rhs } { Lhs } );
}