# duct.rs [![Travis build](https://travis-ci.org/oconnor663/duct.rs.svg?branch=master)](https://travis-ci.org/oconnor663/duct.rs) [![AppVeyor build](https://ci.appveyor.com/api/projects/status/w3g0fplnx234bxji/branch/master?svg=true)](https://ci.appveyor.com/project/oconnor663/duct-rs/branch/master) [![crates.io](https://img.shields.io/crates/v/duct.svg)](https://crates.io/crates/duct) [![docs.rs](https://docs.rs/duct/badge.svg)](https://docs.rs/duct)

Duct is a library for running child processes. Duct makes it easy to build
pipelines and redirect IO like a shell. At the same time, Duct helps you
write correct, portable code: whitespace is never significant, errors from
child processes get reported by default, and a variety of [gotchas, bugs,
and platform
inconsistencies](https://github.com/oconnor663/duct.py/blob/master/gotchas.md)
are handled for you the Right Way™.

- [Documentation](https://docs.rs/duct)
- [Crate](https://crates.io/crates/duct)
- [GitHub repo](https://github.com/oconnor663/duct.rs)
- [the same library, in Python](https://github.com/oconnor663/duct.py)

Changelog
---------

- v0.13
  - Removed the `then` method.
  - Added `ReaderHandle` and `Expression::reader`.
  - Added `Expression::stdout_stderr_swap`.
  - Renamed `stdin`/`stdout`/`stderr` to
    `stdin_path`/`stdout_path`/`stderr_path`.
  - Renamed `stdin_handle`/`stdout_handle`/`stderr_handle` to
    `stdin_file`/`stdout_file`/`stderr_file`.
  - Renamed `input` to `stdin_bytes`.
  - Renamed `Handle::output` to `Handle::into_output`.

Examples
--------

Run a command without capturing any output. Here "hi" is printed directly
to the terminal:

```rust
use duct::cmd;
cmd!("echo", "hi").run()?;
```

Capture the standard output of a command. Here "hi" is returned as a
`String`:

```rust
let stdout = cmd!("echo", "hi").read()?;
assert_eq!(stdout, "hi");
```

Capture the standard output of a pipeline:

```rust
let stdout = cmd!("echo", "hi").pipe(cmd!("sed", "s/i/o/")).read()?;
assert_eq!(stdout, "ho");
```

Merge standard error into standard output and read both incrementally:

```rust
use duct::cmd;
use std::io::prelude::*;
use std::io::BufReader;

let big_cmd = cmd!("bash", "-c", "echo out && echo err 1>&2");
let reader = big_cmd.stderr_to_stdout().reader()?;
let mut lines = BufReader::new(reader).lines();
assert_eq!(lines.next().unwrap()?, "out");
assert_eq!(lines.next().unwrap()?, "err");
```

Children that exit with a non-zero status return an error by default:

```rust
cmd!("false").run()?; // error
```
