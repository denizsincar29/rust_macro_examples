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

## shebang_code

A proc-macro that fetches code from a URL and expands it into your code. This macro uses the `!` syntax (function-like macro) for a shebang-like appearance, as in `shebang_code!("url")`. This is designed to work with playground URLs or raw code URLs.

Usage:
```rust
use procmacros::shebang_code;

shebang_code!("https://gist.githubusercontent.com/rust-play/5e3665f22343b85ec791e13b1f56c367/raw/playground.rs");
```

The macro will expand to the code from the URL. The `!` syntax gives it a shebang-like appearance.

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

