[![Travis branch](https://img.shields.io/badge/rustc-1.15%20or%20later-brightgreen.svg)](https://www.rust-lang.org/)
[![Latest Version](https://img.shields.io/crates/v/enum_to_str_derive.svg)](https://crates.io/crates/enum_to_str_derive)
[![Travis branch](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

# example

`cargo.toml`

```toml
[dependencies]
enum_to_str_derive = "0.1"
``` 

`main.rs`

```rust
#[macro_use]
extern crate enum_to_str_derive;

#[derive(EnumToStr)]
enum HTTPMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE
}

fn main() {
    assert_eq!("GET", HTTPMethod::GET.enum_to_str());
    assert_eq!("HEAD", HTTPMethod::HEAD.enum_to_str());
    assert_eq!("POST", HTTPMethod::POST.enum_to_str());
}
```

# Other Way (which means I'm stupid)

```rust
#[derive(Debug)]
enum HTTPMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE
}

fn main() {
    assert_eq!("GET", format!("{:?}", HTTPMethod::GET));
}
```

# TODO

+ [x] add attribute: `value`
+ [ ] add attribute: `handler`