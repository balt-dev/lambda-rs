use crate::{call, define, Function};

define! {
    /// Identity function. Returns its input.
    /// ```ignore
    /// λx.x
    /// ```
    pub fn Identity ( Input: I ) => Input;
    /// Takes two values and returns the first.
    /// ```ignore
    /// λa.λb.a
    /// ```
    pub fn First ( Input: I, _Ignored: _ ) => Input;
    /// Takes two values and returns the second.
    /// ```ignore
    /// λa.λb.b
    /// ```
    pub fn Second ( _Ignored: _ ) => Identity;
    /// Composes a function with another.
    /// Note that this returns a [`Composed`], for more ergonomic point-free form.
    /// ```ignore
    /// λf.λg.λx.fgx
    /// ```
    pub fn Compose ( Fn: A, OtherFn: B ) => Composed<Fn, OtherFn>;
    /// The composition of two functions. See [`Compose`].
    pub fn Composed<Fn, OtherFn> ( Input: T ) 
        where
            OtherFn: Function<Input>,
            Fn: Function<call!(OtherFn { Input })>
        => call!(Fn { OtherFn { Input } });
    /// Takes a function and an input, and returns the function applied to said input.
    /// ```ignore
    /// λf.λx.fx
    /// ```
    pub fn Apply ( Fn: F, Input: T )
        where Fn: Function<Input>
        => call!{ Fn { Input } };
}