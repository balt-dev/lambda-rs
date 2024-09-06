# lambda-rs

!["Lambda calculus? In *my* type system?" (It's more likely than you think.)](assets/docs/banner.png)

An implementation of the Lambda Calculus in Rust, only using ZSTs, traits, and generics.

```rust
use lambda_types::prelude::*;

type Two = call! { Successor, Successor, Zero  };
type Three = call! { Successor, Two  };
type Six = call! { Multiply, Three, Two };
type Seven = call! { Successor, Six };

static THE_MEANING_OF_LIFE: call! {
    ToNumber { Multiply, Six, Seven }
} = ConstNumber::<{b'*' as u64}>; // 42

```
