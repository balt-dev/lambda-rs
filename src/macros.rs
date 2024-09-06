

#[macro_export]
/**
    Helper macro for easily defining types that implement [`crate::Function`].

    ## Syntax
    The body of the macro should contain an arbitrary amount of "function type" items, ending in semicolons.

    These "function type" items may have an arbitrary amount of attributes.

    Due to how the macro is implemented, `cfg` attributes must be specified _with a @ instead of #_, like this:
    `@[cfg(feature = "abc")]`

    The body of one of these items is as follows:
    ```text
    pub fn Name<A, B> ::= { C. D. { A, B {C, D}}}
        where A: B, {A, B}: {C, D}, /* ... */;
    ```
    It's quite a bit to take in, but it's actually quite simple! Let's break it down.

    - The visibility specifier `pub` is optional, defaulting to private, as expected of any other item.
    
    - The `Name` is the name of the type that's publicly exported by the macro. 
      As you can see, type parameters are also supported, but const generics are unfortunately not.

    - Following the `::=` is the actual definition enclosed in `{}` braces.
      Everything within the brackets is implicitly passed to `crate::call`.

    - The `where` clauses are an unfortunate consequence of using a declarative macro instead of a procedural one,
      as the macro can't generate one _for_ us due to the inability to use macro calls as parameter guards.
      Both sides of each clause are implicitly passed to `crate::call`.

      In order for the compiler to accept your function, you must tell the compiler in these clauses
      that all arguments used in the definition are actually _able_ to be used in the way they're used.

      A good strategy is to write each call out in the where clause while you're writing, like this:
      ```text
      fn F ::= { A. B. C. { C, {B, A}, C }} where
           B: A, // B calls A
           C: {B, A}, // C calls {B, A}
           {C, {B, A}}: C; // {C, {B, A}} calls C
      ```
    
    Unfortunately, due to implementation complexity, anonymous functions like this:
    ```text
    // λn.λf.λx. n (λg.λh. h (g f)) (λu.x) (λu.u)
    pub fn Predecessor ::= { N. F. X. { N {G. H. H { G, F }}, Constant<X>, Identity } };
    ```
    are unsupported.
    Instead, you can break it up into smaller definitions, like this:
    ```text
    pub fn Predecessor ::= { N. F. X. { N, Pred_1<F>, Constant<X>, Identity } };
    fn Pred_1<F> ::= { G. H. { H { G, F } } };
    ```
    which is actually the definition used in [`crate::math::Predecessor`], sans the `where` clauses.

    Reading through how the crate defines function types is highly recommended to get a grasp on the syntax!

    This macro is **hygienic**.
    
 */
macro_rules! define {
    (
        $(
            $(#[$meta: meta])*
            $(@[$modmeta: meta])*
            $vis: vis fn $identifier: ident 
                $(< $($typearg: ident),+ >)? 
            ::= { $($definition: tt)+ }
            $(where $($lhs: tt : $rhs: tt),+ $(,)?)?  
        ;
        )*
    ) => {::paste::paste! {
        $(
        #[allow(non_snake_case)]
        $(#[$modmeta])*
        mod [< __$identifier >] {
            #![allow(unused_parens, non_camel_case_types, unused_imports)]
            use super::*;
            
            $crate::define!{ @item $(#[$meta])* $identifier ; $($($typearg)+)? }
            $crate::define!{ 
                @impl $identifier; $($($typearg)+)? ; 
                { $($definition)+ }
                $(where $($lhs: $rhs),+)? 
            }
        }
        $(#[$modmeta])*
        $vis use [< __$identifier >]::$identifier;
        )*
    }};
    (
        @item $(#[$meta: meta])* $name: ident ; $($($typearg: ident)+)?
    ) => { 
        $(#[$meta])*
        #[allow(unused_parens)]
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
        pub struct $name $( < $($typearg),+ > )? {
            _p: ::core::marker::PhantomData<($($($typearg,)+)?)>
        }
    };
    (
        @impl $ident: ident ; $($arg: ident)*; 
        { $name: ident . $name2: ident . $($definition: tt)+ } $(where $($lhs: tt:$rhs: tt),+)? 
    ) => { ::paste::paste! {
        #[allow(non_camel_case_types)]
        impl<$($arg, )* $name> $crate::Function<$name> for $ident < $($arg,)* > {
            type Output = [< $ident __ $name >] <$($arg,)* $name>;
        }
        $crate::define! {
            @item #[doc(hidden)] [< $ident __ $name >] ; $($arg)* $name
        }
        $crate::define! {
            @impl [<$ident __ $name>] ; $($arg)* $name; 
            { $name2 . $($definition)+ } $(where $($lhs : $rhs),+)?
        }
    } };
    (
        @impl $ident: ident ; $($args: ident)*; 
        { $name: ident . $definition: tt } $(where $($lhs: tt:$rhs: tt),+)?
    ) => {
        #[allow(non_camel_case_types)]
        impl<$($args, )* $name> $crate::Function<$name> for $ident <$($args, )*>
            $(
                where 
                $(
                    $crate::call!($lhs) : $crate::Function<$crate::call!($rhs)>
                ),+
            )?
        {
            type Output = $crate::call!{ $definition };
        }
    };
}

/**
    Ergonomic wrapper macro for calling a function.
    
    Calling syntax is modeled around the lambda calculus, where:
    - `fx` is translated to `f, x`
    - `f(...)` is translated to `f, {...}`

    So, for example, `ab(c(de)f)gh` translates to `a, b, { c, { d, e }, f }, g, h`.

    The whitespace isn't mandatory, but is recommended for readibility (see: `a,b{c,{d,e},f},g,h`).

    This macro is **hygienic**.
 */
#[macro_export]
macro_rules! call {
    ({$($wrapped: tt)+}) => { $crate::call!($($wrapped)+) };
    ($name: ty) => { $name };
    ($name: ty , $arg: ty) => {
        <$name as $crate::Function<$arg>>::Output
    };
    ($name: ty , { $($arg: tt)+ } $($arg2: tt)*) => {
        $crate::call!( $crate::call!($name , $crate::call!($($arg)+) ) $($arg2)* )
    };
    ($name: ty , $arg: ty , $($arg2: tt)+) => {
        $crate::call!( $crate::call!($name , $arg), $($arg2)+ )
    };
}

/**
    Chains applications of a function onto many arguments.
    
    For example, `chained!(Composed with A, B, C, D)` expands to `Composed<A, Composed<B, Composed<C, D>>>`.

    This macro is **hygienic**.
 */
#[macro_export]
macro_rules! chained {
    ($name: ident with $lhs: ty, $rhs: ty $(, $extra: ty)+) => {
        $name<$lhs, $crate::chained!($name with $rhs $(, $extra)+)>
    };
    ($name: ident with $lhs: ty, $rhs: ty) => {
        $name<$lhs, $rhs>
    }
}