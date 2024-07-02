# EnvConf

Simple and small crate for configuration structs.

Crate generates [`impl`](https://doc.rust-lang.org/std/keyword.impl.html) with:

1. `new` public method that fetches values from environment
2. `#field_name` public getters for values

## Why this instead of **X**`-config`?

My crate panics if it can't found variable in environment and `default` is not specified and idk... it contains only 76 LOC.

Anyway, there is no big reason why to use my crate instead of cool and big **X**`-config`.

## Install

1. Using cli `cargo add envconfgen`
2. Manually adding to Cargo.toml

```
# Cargo.toml

[dependencies]
envconfgen = "1.0.0"
```

## Example

```rust
#[derive(envconfgen::EnvConfig)]
struct Config {
  // `var` specifies the environment variable name to fetch the value from.
  // If not provided, the uppercase field name (`TEST_VAR`) will be used.
  #[var("MY_TEST_VAR")]
  test_var: String,

  // `default` provides a fallback value if the environment variable is not set.
  // If not provided and the variable is missing, it will panic.
  //
  // This field does not use `var`, so it defaults to searching for `DATABASE_URL`.
  #[default("nothing")]
  database_url: String,
}

fn main() {
  let config = Config::new(); // Generated factory method

  // Crate also generates a public getters for fields
  // I don't think it's a good idea to use public fields for config struct
  println!("test_var: {}", config.test_var());
  println!("database_url: {}", config.database_url());
}
```

### License - [`MIT`](./LICENSE)

### Contribution

Feel free to open issue or send PR.
