# shrimpstats

shrimpstats is a tool to collect and display statistics about TF2. It parses logs and stores them in a database. It also provides a web interface to display the statistics.

The backend is written in Rust and the frontend in Rust and WebAssembly. The backend is a binary that can be run as a daemon or as a one-shot command. It can be run in a Docker container.

## Examples

```sh
shrimpstats --help
```

## Usage

```sh
shrimpstats [FLAGS] [OPTIONS]
```

## Flags

```sh
-h, --help                   Prints help information
-V, --version                Prints version information
-a, --address <address>      Sets a custom address
-p, --port <port>            Sets a custom port
-d, --database <database>    Sets a custom database name
-u, --user <user>            Sets a custom user name
-w, --password <password>    Sets a custom password
-c, --config <config>        Sets a custom config file
-l, --log <log>              Sets a custom log file
-v, --verbose <level>        Sets the level of verbosity
```

We will use macros and attributes to make the code more readable and to avoid boilerplate code.

We will use the following crates:

- [clap](https://crates.io/crates/clap) for command line parsing
- [log](https://crates.io/crates/log) for logging
- [env_logger](https://crates.io/crates/env_logger) for logging to stdout
- [surrealdb](https://crates.io/crates/surrealdb) for the database
- [serde](https://crates.io/crates/serde) for serialization and deserialization
- [serde_json](https://crates.io/crates/serde_json) for JSON serialization and deserialization
- [serde_derive](https://crates.io/crates/serde_derive) for deriving Serialize and Deserialize
- [chrono](https://crates.io/crates/chrono) for date and time
- [regex](https://crates.io/crates/regex) for regular expressions
- [reqwest](https://crates.io/crates/reqwest) for HTTP requests
- [tokio](https://crates.io/crates/tokio) for asynchronous programming