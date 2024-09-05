#![feature(generic_const_exprs)]

pub trait Function<Input> {
    type Output;
}

pub mod macros;
pub mod primitives;
pub mod boolean_algebra;
pub mod math;