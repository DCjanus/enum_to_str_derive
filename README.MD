[![Latest Version](https://img.shields.io/crates/v/enum_to_str_derive.svg)](https://crates.io/crates/enum_to_str_derive)

I was going to write a simple HTTP client library.

There is an enum, which called `HTTPMethod`, for building a HTTP request message, I have to convert the enum to str.

Without macro, I have to write a lot of pattern match expressions, I believe there will be many typos.

I think it sucks! So I learned the `ProcMacro` and write this lib.

# warning

Because of `proc-macro`, this lib is **nightly** only(1.22).

# example

`cargo.toml`

```toml
[dependencies]
enum_to_str_derive = "0.1.0"
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