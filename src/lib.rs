//! `kvlogger` is a standard log facility for Rust that allows its user to add
//! key/value pairs to their log lines.
//!
//! # Example
//!
//! ```
//! use std::error::Error;
//! use log::*;
//! use kvlogger::{KvLoggerBuilder, *};
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!   KvLoggerBuilder::default()
//!     .set_level(Level::Debug)
//!     .set_datetime_format("%Y-%m-%d")
//!     .init()?;
//!
//!   info!("a simple message");
//!
//!   kvlog!(Info, "user tried to log in", {
//!     "username" => "apognu",
//!     "status" => 200
//!   });
//!
//!   Ok(())
//! }
//! ```

mod builder;
mod kvlogger;
mod macros;
mod utils;

pub use crate::builder::*;
pub use crate::macros::*;
pub use crate::utils::{get_decoration, get_line};
