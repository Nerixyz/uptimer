# uptimer

A multiplatform library to get the uptime of the current process.

 # Example
 
```rust
use std::thread::sleep;
use std::time::Duration;

fn main() {
    sleep(Duration::from_secs(2));
    println!("{:?}", uptimer::get());
}
```

## Features

* `async` enables the `get_async` function.
