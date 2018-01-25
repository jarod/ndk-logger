A library bridging [log](https://crates.io/crates/log) message to [Android NDK log](https://developer.android.com/ndk/reference/log_8h.html).

### Usage

Cargo.toml
```toml
[target.'cfg(target_os="android")'.dependencies]
ndk-logger = "*"
```

Rust entry
```rust
#[macro_use]
extern crate log;
#[cfg(target_os="android")]
extern crate ndk_logger;

#[cfg(target_os="android")]
fn init_logger() {
    ndk_logger::init().unwrap();
}
#[cfg(not(target_os="android"))]
fn init_logger() {
    // init logger for other platform
}

fn main() {
    init_logger();
}
```