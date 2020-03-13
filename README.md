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

## Configuration

`kvlogger` uses [env_logger](https://github.com/sebasmagri/env_logger) under the hood for filter selection. You have two ways to configure the desired level for your logs:

 * You can register the logger at a specific level with `init_at`:

```rust
kvlogger::init_at(Level::Debug)?;
```

 * You can use the `RUST_LOG` environment variable to specify which logs should be considered. See the [env_logger documentation](https://docs.rs/env_logger/0.7.1/env_logger/) for more information.

```shell
$ RUST_LOG=rocket=error,main=info cargo run
```