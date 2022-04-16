# uptimer

A multiplatform library to get the uptime of the current process.

* [x] Windows
* [x] Linux
* [ ] macOS

## Example

```toml
uptimer = { git = "https://github.com/nerixyz/uptimer", tag = "v0.2.0" }
```

```rust
use std::thread::sleep;
use std::time::Duration;

fn main() {
    sleep(Duration::from_secs(2));
    println!("{:?}", uptimer::get());
}
```
