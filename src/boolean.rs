use crate::{define, primitives};

/// Truthy church boolean.
pub type True = primitives::FirstOf;
/// Falsy church boolean.
pub type False = primitives::SecondOf;

define! {
    /// Boolean and operation. Takes two church booleans, and returns whether both are true.
    /// ```ignore
    /// λa.λb.aba
    /// ```
    pub fn And ::= {Lhs. Rhs. { Lhs, Rhs, Lhs }} where 
        Lhs: Rhs,
        {Lhs, Rhs}: Lhs;
    /// Boolean or operation. Takes two church booleans, and returns whether either is true.
    /// ```ignore
    /// λa.λb.aab
    /// ```
    pub fn Or ::= {Lhs. Rhs. { Lhs, Lhs, Rhs }} where
        Lhs: Lhs,
        {Lhs, Lhs}: Rhs;
    /// Boolean xor operation. Takes two church booleans, and returns whether only one is true.
    /// ```ignore
    /// N ::= Not
    /// λa.λb.a(Nb)b
    /// ```
    pub fn Xor ::= {Lhs. Rhs. { Lhs, {Not, Rhs}, Rhs }} where
        Not: Rhs,
        Lhs: { Not, Rhs },
        {Lhs, { Not, Rhs }}: Rhs;
    /// Boolean not operation. Takes a church boolean and returns its inverse.
    /// ```ignore
    /// T ::= True
    /// F ::= False
    /// λv.vFT
    /// ```
    pub fn Not ::= {Value. { Value, False, True }} where
        Value: False,
        {Value, False}: True;
    /// Alternation. Takes a church boolean and two values, and returns the first if the boolean is true, otherwise the second.
    /// ```ignore
    /// λp.λa.λb.pab
    /// ```
    pub fn If ::= {Predicate. Truthy. Falsy. { Predicate, Truthy, Falsy }} where
        Predicate: Truthy,
        {Predicate, Truthy}: Falsy;
}
