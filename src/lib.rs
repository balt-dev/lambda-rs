#![feature(generic_const_exprs)]

pub trait Function<Input> {
    type Output;
}

#[macro_export]
macro_rules! define {
    ($(
        $(#[$attrs: meta])* 
        $vis: vis fn $name: ident $(< $($typeargs: ident),+ >)? ( $($argnames: ident: $args: ty),* ) 
        $(where $($guardty: ty : $guardpath: path),+)? 
        => $output: ty;
    )*) => {$(
        $(#[$attrs])*
        #[allow(non_camel_case_types, unused_parens)]
        $vis struct $name $(< $($typeargs),+ >)? {
            _phantom: ::core::marker::PhantomData<($($($typeargs),+)?)>
        }
        $crate::define!(@inner $vis $name $(< $($typeargs),+ >)? ($($argnames: $args),* ;) $(where $($guardty: $guardpath),+)? => $output);
    )*};
    (@inner 
        $vis: vis $name: ident $(< $($typeargs: ident),+ >)? ( 
            $argname: ident: $arg: ty $(, $argnames: ident: $args: ty)+ ; 
            $($handledname: ident : $handledarg: ty),*
        ) $(where $($guardty: ty : $guardpath: path),+)? => $output: ty
    ) => {
        ::paste::paste! {
            #[allow(non_camel_case_types, unused_parens)]
            impl<$argname $(, $handledname)* $(, $($typeargs),+)?> $crate::Function<$argname> for $crate::define!(@inner_args $(< $($typeargs),+ >)? $name $($handledname)* ) {
                type Output = [< $name _ $argname >]<$argname $(, $handledname)*>;
            }
            #[doc(hidden)]
            #[allow(non_camel_case_types, unused_parens)]
            $vis struct [< $name _ $argname >]<$arg $(, $handledarg)*> {
                _phantom: ::core::marker::PhantomData<($arg $(, $handledarg)*)>
            }
            $crate::define! {
                @inner $vis [< $name _ $argname >] ($($argnames : $args),* ; $argname: $arg $(, $handledname : $handledarg)*) $(where $($guardty: $guardpath),+)? => $output
            }
        }
    };
    (@inner $vis: vis $name: ident $(< $($typeargs: ident),+ >)? ( $inputname: ident : $input: ty ; $($paramname: ident : $param: ty),* ) $(where $($guardty: ty : $guardpath: path),+)? => $output: ty) => { ::paste::paste! {
        impl<$inputname $(, $paramname)* $($(, $typeargs)+)?> Function<$inputname> for $crate::define!(@inner_args $(< $($typeargs),+ >)? $name $($paramname)* )
            $(where $(
                $guardty: $guardpath
            ),+)?
        {
            type Output = $output;
        }
    }};
    (@inner $name: ident $(< $($typeargs: ident),+ >)? ( ; ) => $output: ty) => { ::paste::paste! {
        impl<Input> Function<Input> for $name $(< $($typeargs),+ >)? {
            type Output = $output;
        }
    }};
    (@inner_args $(< $($typeargs: ident),+ >)? $name: ident) => { $name $(< $($typeargs),+ >)? };
    (@inner_args $name: ident $($handledname: ident)+) => { $name < $($handledname),+ >};
    (@inner_args < $($typeargs: ident),+ > $name: ident $($handledname: ident)+) => { $name < $($handledname, )+ $($typeargs), +>}
}

#[macro_export]
macro_rules! call {
    ($name: ty { $name2: ty { $($arg: tt)+ } } $({ $($arg2: tt)+ })+) => {
        call!( call!($name { $name2 { $($arg)+ } }) $({ $($arg2)+ })+ )
    };
    ($name: ty { $arg: ty } $({ $($arg2: tt)+ })+) => {
        call!( call!($name { $arg }) $({ $($arg2)+ })+ )
    };
    ($name: ty { $name2: ty { $($arg: tt)+ } }) => {
        call!( $name { call!( $name2 { $($arg)+ } )})
    };
    ($name: ty { $arg: ty }) => {
        <$name as $crate::Function<$arg>>::Output
    }
}

pub mod primitives;
pub mod boolean_algebra;
pub mod math;

#[cfg(test)]
mod test {
    use crate::math::*;

    static _TEST: call!(
        ToNumber { Successor { Successor { Successor { Successor { Zero } } } } }
    ) = ConstNumber::<4>;
}