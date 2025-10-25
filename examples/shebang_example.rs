use procmacros::shebang_code;

// Example of shebang_code macro using ! syntax (function-like macro)
// This demonstrates the "shebang-like" ! syntax as requested
// The macro fetches code from a URL and inserts it

shebang_code!("https://gist.githubusercontent.com/rust-play/5e3665f22343b85ec791e13b1f56c367/raw/playground.rs");

// The above will expand to the code from the URL
// In this case, it will create a main() function from the gist
