/*!

# Note

As there is no runtime functionality of this crate,
no actual _runtime_ tests run here.
Instead, if any of the statics fail, then this will fail to compile.

*/

use lambda_rs::prelude::*;

type One = call!{ Successor, Zero };
type Two = call!{ Successor, One };
type Three = call!{ Successor, Two };
type Four = call!{ Successor, Three };

static _MATH_TEST: call! {
    If, { Eq, { Multiply, Two, Four }, { Exponent, Two, Three } }, u8, ()
} = 0;


fn main() {}