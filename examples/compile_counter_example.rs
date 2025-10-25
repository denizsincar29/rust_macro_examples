use procmacros::{fetch_code, compile_counter};

// Test the compile counter macro - this will count compilations and error after 3
compile_counter!();

// Test the original fetch_code macro - this includes a main function
fetch_code!("https://gist.githubusercontent.com/rust-play/5e3665f22343b85ec791e13b1f56c367/raw/playground.rs");