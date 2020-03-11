extern crate kvlogger;
extern crate log;

use kvlogger::*;
use log::Level;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  kvlogger::init()?;

  kvlog!(Info, "GET /favicon.ico", {
    "method" => "GET",
    "scheme" => "https",
    "host" => "example.com",
    "path" => "/favicon.ico",
    "duration" => "0.23"
  });

  kvlog!(Error, "could not retrieve content", {
    "error" => "timeout"
  });

  Ok(())
}
