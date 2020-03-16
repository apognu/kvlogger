use kvlogger::{KvLoggerBuilder, *};
use log::*;
use std::error::Error;

struct Person {
  username: String,
}

impl std::fmt::Display for Person {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Person({})", self.username)
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  KvLoggerBuilder::default().init()?;

  info!("simple message");
  warn!("this is a warning");

  kvlog!(Info, "coucou");

  kvlog!(Info, "GET /favicon.ico", {
    "method" => "GET",
    "scheme" => "https",
    "host" => "example.com",
    "path" => "/favicon.ico",
    "duration" => 0.23
  });

  kvlog!(Error, "could not retrieve content", {
    "error" => "timeout"
  });

  let user = Person {
    username: "apognu".to_string(),
  };

  kvlog!(Trace, "my message", {
    "boolean" => true,
    "integer" => 12,
    "float" => 103.45,
    "struct" => user
  });

  Ok(())
}
