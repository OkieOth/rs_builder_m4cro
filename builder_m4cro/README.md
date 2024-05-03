[![ci](https://github.com/OkieOth/rs_observable/actions/workflows/rust.yml/badge.svg)](https://github.com/OkieOth/rs_builder_m4cro/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/builder_m4cro.svg)](https://crates.io/crates/builder_m4cro)

This is a crate provides two simple macros:
* Builder - an implementation of the factory pattern
* BuilderFromDefault - factory pattern implementation that relies on the
    implementation of the Default trait, to provide customized init values


For more examples look at:
* Builder: `examples/builder/src/main.rs`
* BuilderFromDefault: `examples/builder_from_default/src/main.rs`

```rust
use builder_m4cro::Builder;

/// Custom type that's annotated with the Builder macro
#[derive(Builder, Debug)]
pub struct TestType {
    pub a_unsigned: u32,
    pub a_opt_unsigned: Option<u32>,
    pub a_string: String,
    pub a_opt_string: Option<String>,
}

/// The macro expandes to the following code
impl TestType {
    pub fn builder() -> TestTypeBuilder {
        TestTypeBuilder::new()
    }
}
pub struct TestTypeBuilder {
    a_unsigned: Option<u32>,
    a_string: Option<String>,
    a_opt_unsigned: Option<u32>,
    a_opt_string: Option<String>,
}
// ...

impl TestTypeBuilder {
    pub fn new() -> TestTypeBuilder {
        TestTypeBuilder::default()
    }
    pub fn a_unsigned(&mut self, a_unsigned: u32) -> &mut Self {
        self.a_unsigned = Some(a_unsigned);
        self
    }
    pub fn a_string(&mut self, a_string: &str) -> &mut Self {
        self.a_string = Some(a_string.to_string());
        self
    }
    pub fn a_opt_unsigned(&mut self, a_opt_unsigned: u32) -> &mut Self {
        self.a_opt_unsigned = Some(a_opt_unsigned);
        self
    }
    pub fn a_opt_string(&mut self, a_opt_string: &str) -> &mut Self {
        self.a_opt_string = Some(a_opt_string.to_string());
        self
    }
    pub fn build(&self) -> Result<TestType, String> {
        Ok(TestType {
            a_unsigned: self
                .a_unsigned
                .clone()
                .ok_or_else(|| {
                    let res = ::alloc::fmt::format(
                        format_args!("{0} is not set", "a_unsigned"),
                    );
                    res
                })?,
            a_string: self
                .a_string
                .clone()
                .ok_or_else(|| {
                    let res = ::alloc::fmt::format(
                        format_args!("{0} is not set", "a_string"),
                    );
                    res
                })?,
            a_opt_unsigned: self.a_opt_unsigned.clone(),
            a_opt_string: self.a_opt_string.clone(),
        })
    }
}
```


# Helper to support marco dev

```bash
# Install cargo expand
# https://github.com/dtolnay/cargo-expand
cargo install cargo-expand

# e. g. ...
cd examples/result_builder
cargo expand
```