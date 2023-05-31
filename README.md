# call_input
Get user input in rust

## Installation and usage

`$ cargo add call_input`

```rust
mod call_input;

fn main() {
    let n = call_input::get_i32("Enter your number:");
    let s = call_input::get_string("Enter your string:");
    let f = call_input::get_float("Enter your float:");
}

```
its that simple
