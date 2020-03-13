# Key-value logger for Rust

![Banner](https://github.com/apognu/kvlogger/blob/master/resources/kvlogger.png)

## Usage

Like other `log`gers, `kvlogger` must be initialized and registered as the default log handler in your program. You can then use either that usual log macros (that do not handle key/value pairs), or use the `kvlog!` macro and add your data.

Any type that implements ```Display``` can be used as a value.

```rust
use std::error::Error;
use log::*;
use kvlogger::*;

fn main() -> Result<(), Box<dyn Error>> {
  kvlogger::init()?;

  info!("a simple message");

  kvlog!(Info, "user tried to log in", {
    "username" => "apognu",
    "status" => 200
  });

  Ok(())
}
```