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

  // Generated public getters
  println!("test_var: {}", config.test_var());
  println!("database_url: {}", config.database_url());
}
