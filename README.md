# lambda-rs

An implementation of the Lambda Calculus in Rust, only using ZSTs, traits, and generics.

```rust
use lambda_rs::{call, math::*};

type Two = call! { Successor { Successor { Zero } } };
type Three = call! { Successor { Two } };
type Six = call! { Multiply { Three } { Two } };
type Seven = call! { Successor { Six } };

static THE_MEANING_OF_LIFE: call! {
    ToNumber { Multiply { Six } { Seven } }
} = ConstNumber::<{b'*' as u64}>; // 42

```