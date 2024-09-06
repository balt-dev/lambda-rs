#![feature(generic_const_exprs)]

/*!
lambda-rs
===
Lambda calculus? In *my* type system? _(It's more likely than you think.)_
---

Implements [the Lambda Calculus](https://en.wikipedia.org/wiki/Lambda_calculus) in Rust's type system.

There is **zero** runtime functionality in this crate - it's all done using generics, traits, and associated types.

If you want to toy around with this, check out the [`prelude`].

If you want to write your own function types, check out the [`macros`]. 

**The Y combinator is left unimplemented, as Rust evaluates types greedily, making it unusable.**
*/


pub trait Function<Input> {
    type Output;
}


pub mod macros;
pub mod primitives;
pub mod boolean;
pub mod math;
pub mod datatypes;

pub mod prelude {
    pub use crate::{define, call};
    pub use crate::primitives::{Identity, FirstOf, SecondOf, Compose, Composed, Apply, Curry, Curried, Constant};
    pub use crate::boolean::{True, False, And, Or, Xor, Not, If};
    pub use crate::math::{Add, Subtract, Multiply, Exponent, Successor, Predecessor, IsZero, Eq, Leq, ToNumber, ConstNumber, Zero};
    pub use crate::datatypes::{Nil, First, Second, Pair, Null};
}
