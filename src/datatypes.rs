

use crate::prelude::*;

define! {
    /// Pair of two values.
    /// ```ignore
    /// λx.λy.λf.fxy
    /// ```
    pub fn Pair ::= { X. Y. F. { F, X, Y }} where
        F: X,
        {F, X}: Y;
    /// Gets the first element of a pair.
    /// ```ignore
    /// T ::= True
    /// λp.pT
    /// ```
    pub fn First ::= { P. { P, True }} where P: True;
    /// Gets the second element of a pair.
    /// ```ignore
    /// F ::= False
    /// λp.pF
    /// ```
    pub fn Second ::= { P. { P, False }} where P: False;
    /// Tests whether a value is [`Nil`].
    /// ```ignore
    /// F ::= False
    /// λp.p(λx.λy.F)
    /// ```
    pub fn Null ::= { P. { P, Constant<Constant<False>> }} where P: (Constant<Constant<False>>);
}

/// An empty singly linked list.
pub type Nil = Constant<True>;

#[test]
fn t() {
    use crate::prelude::*;

    let _: call!{ Null, Nil } = True::default();
    let _: call!{ Null, {Pair, (), Nil} } = False::default();
}