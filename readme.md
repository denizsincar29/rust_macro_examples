# rust_macro_examples

this is my experiment collection of very stupid rust macros

## fetch_code

this macro fetches the code from the given url and includes it in the code

Usage:
```rust
use procmacros::fetch_code;

fetch_code!("https://gist.githubusercontent.com/rust-play/5e3665f22343b85ec791e13b1f56c367/raw/playground.rs");
```

## compile_counter

A proc-macro that counts how many times the binary has been compiled. It tracks compilation attempts in a `.compile_count` file, and if the count exceeds the limit, it produces a compile error with a custom message.

Usage:
```rust
use procmacros::compile_counter;

// Default: allows 3 compilations
compile_counter!();

// Custom limit: allows 5 compilations
compile_counter!(5);

// Custom limit with custom error message
compile_counter!(3, "You've reached the compilation limit!");
```

After the specified number of clean builds, attempting another build will result in a compile error.

**Force rebuild**: To reset the counter or force macro re-expansion, use:
```bash
# Remove the counter file and rebuild
rm .compile_count && cargo clean && cargo build

# Or just force a clean rebuild
cargo clean && cargo build
```

## compile_time

A proc-macro that allows compilation only at specific times with a custom error message. Useful for time-limited code, demo versions, or enforcing work hours.

Usage:
```rust
use procmacros::compile_time;

// Only allow compilation between 9 AM and 5 PM
compile_time!("09:00", "17:00", "Can only compile during business hours (9 AM - 5 PM)!");

// With date range (YYYY-MM-DD HH:MM format)
compile_time!("2024-01-01 00:00", "2024-12-31 23:59", "This code expires at end of 2024!");
```

**Force rebuild**: Macros are expanded during compilation. To re-run this macro:
```bash
# Force a clean rebuild to re-expand all macros
cargo clean && cargo build
```

## playground

A proc-macro that fetches code from a Rust Playground URL and expands it into your code.

Usage:
```rust
use procmacros::playground;

playground!("https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=...");
```

The macro will expand to the code from the playground URL.

## raw_code

A proc-macro that fetches raw Rust code from a URL and expands it into your code.

Usage:
```rust
use procmacros::raw_code;

raw_code!("https://gist.githubusercontent.com/rust-play/5e3665f22343b85ec791e13b1f56c367/raw/playground.rs");
```

The macro will expand to the code from the URL.

## brainfuck

A proc-macro attribute that compiles Brainfuck code into Rust code at compile time. The macro takes one string literal: the Brainfuck code to execute. The generated function accepts an `&str` parameter for input (one byte per `,` command in the BF code).

Usage:
```rust
use procmacros::brainfuck;

// Classic "Hello World!" in Brainfuck
#[brainfuck("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.")]
fn hello_world() {}

// Echo input
#[brainfuck(",.")]
fn echo() {}

// Use the generated functions
fn main() {
    println!("{}", hello_world(""));  // Output: Hello World!
    println!("{}", echo("A"));        // Output: A
}
```

The macro compiles the Brainfuck code at compile time and generates a Rust function that accepts an input string parameter and returns the output as a `String`. The `,` command in Brainfuck reads bytes from the input string sequentially.

## Examples

All examples are located in the `examples/` directory:
- `compile_counter_example.rs` - Demonstrates the compile counter and fetch_code macros
- `compile_time_example.rs` - Demonstrates time-based compilation restrictions
- `url_fetching_example.rs` - Demonstrates the raw_code macro
- `brainfuck_example.rs` - Demonstrates the brainfuck compiler macro

Run examples with: `cargo run --example <example_name>`
