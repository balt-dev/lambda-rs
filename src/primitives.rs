crate::define! {
    /// Identity function. Returns its input.
    /// ```text
    /// λx.x
    /// ```
    pub fn Identity ::= { Input. Input };
    /// Takes two values and returns the first.
    /// ```text
    /// λa.λb.a
    /// ```
    pub fn FirstOf ::= { Input. (Constant<Input>) };
    /// Returns its type parameter, ignoring input.
    /// ```text
    /// λv.I
    /// ```
    pub fn Constant<I> ::= { _Ignored. I };
    /// Takes two values and returns the second.
    /// ```text
    /// λa.λb.b
    /// ```
    pub fn SecondOf ::= { _Ignored. Identity };
    
    /// Composes a function with another.
    /// Note that this returns a [`Composed`], for more ergonomic point-free form.
    /// ```text
    /// λf.λg.(λx.f(gx))
    /// ```
    pub fn Compose ::= { A. B. (Composed<A, B>) };
    
    /// The composition of two functions. See [`Compose`].
    /// ```text
    /// λx.FGx
    /// ```
    pub fn Composed<F, G>
        ::= { Input. { F, { G, Input }}}
    where
        G: Input,
        F: {G, Input};
    
    /// Takes a function and an input, and returns the function applied to said input.
    /// ```text
    /// λf.λx.fx
    /// ```
    pub fn Apply
        ::= { Fn. Input. { Fn, Input }}
    where
        Fn: Input;

    /// S combinator. Takes three inputs, and applies the first to the second and third.
    /// ```text
    /// λx.λy.λz.xz(yz)
    /// ```
    pub fn Sheinfinkel ::= { X. Y. Z. {X, Z, { Y, Z }}} where
        X: Z, Y: Z, {X, Z}: {Y, Z};

    /// Takes a function that takes two arguments and the first argument to said function,
    /// and returns a function that takes the second argument and runs the function with both.
    /// See [`Curried`].
    /// ```text
    /// λf.λx.(λy.fxy)
    /// ```
    pub fn Curry
        ::= { Fn. Input. (Curried<Fn, Input>) };
    
    /// Function that curries the first type argument with the second and the input. See [`Curry`].
    /// ```text
    /// λy.FXy
    /// ```
    pub fn Curried<Fn, X> ::= { Y. { Fn, X, Y }} where
        Fn: X,
        {Fn, X}: Y;
}
