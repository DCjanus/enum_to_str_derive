[![Rustc version](https://img.shields.io/badge/rustc-1.26%20or%20later-brightgreen.svg)](https://www.rust-lang.org/)
[![Latest Version](https://img.shields.io/crates/v/enum_to_str_derive.svg)](https://crates.io/crates/enum_to_str_derive)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![Build state](https://travis-ci.org/DCjanus/enum_to_str_derive.svg?branch=master)](https://travis-ci.org/DCjanus/enum_to_str_derive)

# example

`cargo.toml`

```toml
[dependencies]
enum_to_str_derive = "0.2"
``` 

`main.rs`

```rust
#[macro_use]
extern crate enum_to_str_derive;

#[derive(EnumToStr)]
enum Foo {
    Foo,
    #[ETS(value = r#"Can you please tell me what does "foo" mean"#)]
    Question,
}

fn main() {
    debug_assert_eq!("Foo", Foo::Foo.enum_to_str());
    debug_assert_eq!(
        r#"Can you please tell me what does "foo" mean"#,
        Foo::Question.enum_to_str()
    );
}
```

# TODO

+ [x] add attribute: `value`
+ [ ] add attribute: `handler`