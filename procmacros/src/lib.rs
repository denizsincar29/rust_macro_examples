// lets make a procmacro that enters a code from url and inserts it into the code

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use reqwest::blocking::get;
use std::fs;
use std::path::PathBuf;

#[proc_macro]
pub fn fetch_code(input: TokenStream) -> TokenStream {
    let url = parse_macro_input!(input as LitStr).value();
    let response = get(&url).unwrap_or_else(|e| panic!("Failed to fetch code from URL '{}': {}", url, e));
    let codestring: String = response.text().unwrap_or_else(|e| panic!("Failed to read response text from '{}': {}", url, e));
    let code: proc_macro2::TokenStream = codestring.parse().unwrap_or_else(|e| {
        panic!("Failed to parse code from '{}' as valid Rust code: {}. Check that the URL contains valid Rust syntax.", url, e)
    });


    let tokens = quote! {
        #code
    };

    tokens.into()
}

/// A proc-macro that counts how many times the binary has been compiled.
/// If the count exceeds 3, it produces a compile error.
/// 
/// Usage: `compile_counter!()`
#[proc_macro]
pub fn compile_counter(_input: TokenStream) -> TokenStream {
    // Get the path to store the counter file
    let counter_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".compile_count");
    
    // Read the current count or start at 0
    let count = if counter_file.exists() {
        fs::read_to_string(&counter_file)
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok())
            .unwrap_or(0)
    } else {
        0
    };
    
    // Increment the count
    let new_count = count + 1;
    
    // Write the new count back to the file
    if let Err(e) = fs::write(&counter_file, new_count.to_string()) {
        eprintln!("Warning: Failed to write compile count to {:?}: {}", counter_file, e);
    }
    
    // Check if we've exceeded the limit
    if new_count > 3 {
        // Create a compile error
        return quote! {
            compile_error!("this program can't be compiled more than 3 times!");
        }.into();
    }
    
    // Return an empty token stream (or you could return some info about the count)
    quote! {
        // Compilation count: #new_count
    }.into()
}

/// A proc-macro that fetches code from a Rust Playground URL and inserts it.
/// 
/// Usage:
/// ```rust
/// playground!("https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=...");
/// ```
#[proc_macro]
pub fn playground(input: TokenStream) -> TokenStream {
    let url = parse_macro_input!(input as LitStr).value();
    
    // Fetch the code from the playground URL
    let response = get(&url).unwrap_or_else(|e| panic!("Failed to fetch code from playground URL '{}': {}", url, e));
    let codestring: String = response.text().unwrap_or_else(|e| panic!("Failed to read response text from '{}': {}", url, e));
    let code: proc_macro2::TokenStream = codestring.parse().unwrap_or_else(|e| {
        panic!("Failed to parse code from '{}' as valid Rust code: {}. Check that the URL contains valid Rust syntax.", url, e)
    });

    let tokens = quote! {
        #code
    };

    tokens.into()
}

/// A proc-macro that fetches raw Rust code from a URL and inserts it.
/// 
/// Usage:
/// ```rust
/// raw_code!("https://gist.githubusercontent.com/.../file.rs");
/// ```
#[proc_macro]
pub fn raw_code(input: TokenStream) -> TokenStream {
    let url = parse_macro_input!(input as LitStr).value();
    
    // Fetch the raw code from the URL
    let response = get(&url).unwrap_or_else(|e| panic!("Failed to fetch raw code from URL '{}': {}", url, e));
    let codestring: String = response.text().unwrap_or_else(|e| panic!("Failed to read response text from '{}': {}", url, e));
    let code: proc_macro2::TokenStream = codestring.parse().unwrap_or_else(|e| {
        panic!("Failed to parse code from '{}' as valid Rust code: {}. Check that the URL contains valid Rust syntax.", url, e)
    });

    let tokens = quote! {
        #code
    };

    tokens.into()
}

// Question: Can we make a proc-macro that defines dependencies for the rust project to not ship cargo.toml?
// 
// Answer: Technically, it's theoretically possible but highly impractical and not recommended:
// 
// 1. Proc-macros execute during compilation, which happens AFTER dependency resolution
// 2. Cargo needs Cargo.toml before it even starts compiling to know which crates to download
// 3. You could theoretically:
//    - Create a build.rs script that generates Cargo.toml before compilation
//    - Use cargo-script or similar tools that embed dependencies in source files
//    - Create a custom cargo wrapper that reads dependency info from source annotations
// 
// 4. However, this would break the entire Cargo ecosystem:
//    - No IDE support for dependencies
//    - No cargo.lock for reproducible builds
//    - Can't use standard cargo commands
//    - Build tools wouldn't work properly
//    - Publishing to crates.io would be impossible
// 
// 5. There's a reason cargo.toml exists - it's the standard way to declare project metadata
//    and dependencies in Rust. Fighting against this would create more problems than it solves.
// 
// So while you *might* be able to hack something together, it's definitely not recommended
// and would go against Rust's philosophy of explicit, declarative dependency management.

/// A proc-macro attribute that compiles Brainfuck code into Rust code at compile time.
/// 
/// The macro takes two string literals:
/// - The Brainfuck code to compile
/// - The input string (one byte per ',' command in the BF code)
/// 
/// Usage:
/// ```rust
/// #[brainfuck("+++++++++[>++++++++<-]>.", "")]
/// fn hello() {}
/// ```
/// 
/// The function will be replaced with code that returns the BF output as a String.
#[proc_macro_attribute]
pub fn brainfuck(attr: TokenStream, item: TokenStream) -> TokenStream {
    use syn::{parse::Parser, punctuated::Punctuated, Token, ItemFn};
    
    // Parse the attribute arguments as two comma-separated string literals
    let parser = Punctuated::<LitStr, Token![,]>::parse_separated_nonempty;
    let args = parser.parse(attr).expect("Expected two string literals: brainfuck code and input");
    
    let args_vec: Vec<_> = args.into_iter().collect();
    if args_vec.len() != 2 {
        panic!("brainfuck macro requires exactly 2 arguments: BF code and input string");
    }
    
    let bf_code = args_vec[0].value();
    let input = args_vec[1].value();
    
    // Parse the item to get the function name
    let func = syn::parse::<ItemFn>(item).expect("brainfuck attribute can only be applied to functions");
    let func_name = func.sig.ident;
    
    // Compile the brainfuck code into Rust code
    let mut rust_code = format!("fn {}() -> String {{\n", func_name);
    rust_code.push_str("    let mut memory = vec![0u8; 30000];\n");
    rust_code.push_str("    let mut ptr = 0usize;\n");
    rust_code.push_str("    let mut output = String::new();\n");
    
    if !input.is_empty() {
        rust_code.push_str(&format!("    let input = {:?}.as_bytes();\n", input));
        rust_code.push_str("    let mut input_ptr = 0usize;\n");
    }
    
    rust_code.push_str("\n");
    
    // Track loop depth for proper indentation
    let mut indent = 1;
    
    for ch in bf_code.chars() {
        let spaces = "    ".repeat(indent);
        match ch {
            '>' => rust_code.push_str(&format!("{}ptr = (ptr + 1) % 30000;\n", spaces)),
            '<' => rust_code.push_str(&format!("{}ptr = if ptr == 0 {{ 29999 }} else {{ ptr - 1 }};\n", spaces)),
            '+' => rust_code.push_str(&format!("{}memory[ptr] = memory[ptr].wrapping_add(1);\n", spaces)),
            '-' => rust_code.push_str(&format!("{}memory[ptr] = memory[ptr].wrapping_sub(1);\n", spaces)),
            '.' => rust_code.push_str(&format!("{}output.push(memory[ptr] as char);\n", spaces)),
            ',' => {
                if !input.is_empty() {
                    rust_code.push_str(&format!("{}if input_ptr < input.len() {{\n", spaces));
                    rust_code.push_str(&format!("{}    memory[ptr] = input[input_ptr];\n", spaces));
                    rust_code.push_str(&format!("{}    input_ptr += 1;\n", spaces));
                    rust_code.push_str(&format!("{}}}\n", spaces));
                } else {
                    rust_code.push_str(&format!("{}memory[ptr] = 0;\n", spaces));
                }
            },
            '[' => {
                rust_code.push_str(&format!("{}while memory[ptr] != 0 {{\n", spaces));
                indent += 1;
            },
            ']' => {
                indent = indent.saturating_sub(1);
                let spaces = "    ".repeat(indent);
                rust_code.push_str(&format!("{}}}\n", spaces));
            },
            _ => {} // Ignore other characters (comments)
        }
    }
    
    rust_code.push_str("    output\n");
    rust_code.push_str("}\n");
    
    // Parse the generated Rust code into a TokenStream
    let code: proc_macro2::TokenStream = rust_code.parse().unwrap_or_else(|e| {
        panic!("Failed to parse generated Rust code: {}. Generated code:\n{}", e, rust_code)
    });
    
    code.into()
}


