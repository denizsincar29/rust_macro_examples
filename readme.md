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

A proc-macro that counts how many times the binary has been compiled. It tracks compilation attempts in a `.compile_count` file, and if the count exceeds 3, it produces a compile error with the message "this program can't be compiled more than 3 times!"

Usage:
```rust
use procmacros::compile_counter;

compile_counter!();
```

After 3 clean builds, attempting a 4th build will result in:
```
error: this program can't be compiled more than 3 times!
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
- `url_fetching_example.rs` - Demonstrates the raw_code macro
- `brainfuck_example.rs` - Demonstrates the brainfuck compiler macro

Run examples with: `cargo run --example <example_name>`

## About dependency definition macros

**Question**: Can we make a proc-macro that defines dependencies for the rust project to not ship cargo.toml?

**Answer**: While theoretically possible, it's highly impractical and not recommended. The main issues are:

1. **Proc-macros execute during compilation**, which happens AFTER dependency resolution
2. **Cargo needs Cargo.toml before it even starts compiling** to know which crates to download
3. You could theoretically:
   - Create a build.rs script that generates Cargo.toml before compilation
   - Use cargo-script or similar tools that embed dependencies in source files
   - Create a custom cargo wrapper that reads dependency info from source annotations

4. However, this would break the entire Cargo ecosystem:
   - No IDE support for dependencies
   - No cargo.lock for reproducible builds
   - Can't use standard cargo commands
   - Build tools wouldn't work properly
   - Publishing to crates.io would be impossible

5. **There's a reason cargo.toml exists** - it's the standard way to declare project metadata and dependencies in Rust. Fighting against this would create more problems than it solves.

So while you *might* be able to hack something together, it's definitely not recommended and would go against Rust's philosophy of explicit, declarative dependency management.

