extern crate kvlogger;
extern crate log;

use kvlogger::*;
use log::Level;
use log::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  kvlogger::init()?;

  info!("simple message");
  warn!("this is a warning");

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