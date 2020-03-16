# Key-value logger for Rust

![Banner](https://github.com/apognu/kvlogger/blob/master/resources/kvlogger.png)

## Usage

Like other `log`gers, `kvlogger` must be initialized and registered as the default log handler in your program. You can then use either that usual log macros (that do not handle key/value pairs), or use the `kvlog!` macro and add your data.

Any type that implements ```Display``` can be used as a value.

```rust
use std::error::Error;
use log::*;
use kvlogger::{KvLoggerBuilder, *};

fn main() -> Result<(), Box<dyn Error>> {
  KvLoggerBuilder::default()
    .set_level(Level::Debug)
    .set_datetime_format("%Y-%m-%d")
    .init()?;

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

 * You can force the logger at a specific level with the `set_level(mut self, log::Level)` method of `KvLoggerBuilder`:

```rust
KvLoggerBuilder::default()
  .set_level(Level::Debug)
  .init()?;
```

 * Otherwise, you can use the `RUST_LOG` environment variable to specify which logs should be considered. See the [env_logger documentation](https://docs.rs/env_logger/0.7.1/env_logger/) for more information.

```shell
$ RUST_LOG=rocket=error,main=info cargo run
```

 * If using the `datetime` feature (see below), you can specify another format from the default, with the `set_datetime_format(mut self, Into<String>)` method:

```rust
KvLoggerBuilder::default()
  .set_datetime_format("%Y-%m-%d")
  .init()?;
```

### Optional features

By default, the date used is printed as the number of seconds since UNIX epoch. You can opt in the use of more complex (and human readable) formats by enabling the `datetime` feature in `Cargo.toml`:

```toml
[dependencies]
kvlogger = { version = "*", features = ["datetime"] }
```

## Examples

```shell
$ cargo run --example simple
$ cargo run --example simple --features datetime
$ RUST_LOG=simple=trace cargo run --example simple --features datetime
```
