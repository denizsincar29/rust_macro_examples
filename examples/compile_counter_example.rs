use procmacros::{fetch_code, compile_counter};

// Test the compile counter macro with custom limit and message
compile_counter!(5, "This example can only be compiled 5 times! Reset with: rm .compile_count && cargo clean");

// Test the original fetch_code macro - this includes a main function
fetch_code!("https://gist.githubusercontent.com/rust-play/5e3665f22343b85ec791e13b1f56c367/raw/playground.rs");