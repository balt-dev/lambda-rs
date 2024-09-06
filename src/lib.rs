#![cfg_attr(any(doc, feature = "const-numeral"), feature(generic_const_exprs))]

/*!
lambda-rs
===
*/
#![doc = concat!("![
    Lambda calculus? In *my* type system? _(It's more likely than you think.)_
](", include_str!("../assets/image_data_url.txt"), ")")]
/*!
---

Implements [the Lambda Calculus](https://en.wikipedia.org/wiki/Lambda_calculus) in Rust's type system.

There is **zero** runtime functionality _or_ procedural macros in this crate - it's all done using generics, traits, and associated types.

If you want to toy around with this, check out the [`prelude`].

If you want to write your own function types, check out the [Macros](#macros).

**The Y combinator is left unimplemented, as Rust evaluates types greedily, making it unusable.**
*/


pub trait Function<Input> {
    type Output;
}


/// Helper macros for easier definition and usage of function types.
pub mod macros;
/// Primitive constructs that are often useful.
pub mod primitives;
/// Function types relating to boolean algebra.
pub mod boolean;
/// Function types relating to church numerals and mathematics.
/// 
/// The `const-numeral` feature flag, off by default, adds a wrapper to allow converting any church numeral to a number.
/// This flag requires `#![feature(generic_const_exprs)]`.
pub mod math;
/// Function types relating to the "pair" datatype, and singly linked lists made out of them.
pub mod datatypes;

/// A module you can glob import to get many useful things in scope.
pub mod prelude {
    pub use crate::{define, call, chained};
    pub use crate::primitives::*;
    pub use crate::boolean::*;
    pub use crate::math::*;
    pub use crate::datatypes::*;
}
