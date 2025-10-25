use procmacros::raw_code;

// Example of raw_code macro
// This macro fetches raw Rust code from a URL and inserts it

raw_code!("https://gist.githubusercontent.com/rust-play/5e3665f22343b85ec791e13b1f56c367/raw/playground.rs");

// The above will expand to the code from the URL
// In this case, it will create a main() function from the gist
