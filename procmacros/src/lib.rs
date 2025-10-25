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
/// Takes an optional limit (defaults to 3) and custom error message.
/// 
/// Usage: 
/// - `compile_counter!()` - defaults to 3 attempts
/// - `compile_counter!(5)` - allows 5 attempts
/// - `compile_counter!(3, "Custom error message!")`
#[proc_macro]
pub fn compile_counter(input: TokenStream) -> TokenStream {
    use syn::{parse::Parser, punctuated::Punctuated, Token, Lit, Expr};
    
    // Parse optional arguments: limit and error message
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let args = parser.parse(input).unwrap_or_else(|_| Punctuated::new());
    let args_vec: Vec<_> = args.into_iter().collect();
    
    // Extract limit (default 3) and error message
    let limit: u32 = if args_vec.is_empty() {
        3
    } else {
        match &args_vec[0] {
            Expr::Lit(expr_lit) => {
                if let Lit::Int(lit_int) = &expr_lit.lit {
                    lit_int.base10_parse().unwrap_or(3)
                } else {
                    3
                }
            }
            _ => 3,
        }
    };
    
    let error_msg = if args_vec.len() > 1 {
        match &args_vec[1] {
            Expr::Lit(expr_lit) => {
                if let Lit::Str(lit_str) = &expr_lit.lit {
                    lit_str.value()
                } else {
                    format!("this program can't be compiled more than {} times!", limit)
                }
            }
            _ => format!("this program can't be compiled more than {} times!", limit),
        }
    } else {
        format!("this program can't be compiled more than {} times!", limit)
    };
    
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
    if new_count > limit {
        // Create a compile error with custom message
        return quote! {
            compile_error!(#error_msg);
        }.into();
    }
    
    // Return an empty token stream (or you could return some info about the count)
    quote! {
        // Compilation count: #new_count
    }.into()
}

/// A proc-macro that allows compilation only at specific times with a custom error message.
/// 
/// Usage:
/// ```rust
/// // Only allow compilation between 9 AM and 5 PM
/// compile_time!("09:00", "17:00", "Can only compile during business hours (9 AM - 5 PM)!");
/// 
/// // With date range (YYYY-MM-DD HH:MM format)
/// compile_time!("2024-01-01 00:00", "2024-12-31 23:59", "This code expires at end of 2024!");
/// ```
#[proc_macro]
pub fn compile_time(input: TokenStream) -> TokenStream {
    use syn::{parse::Parser, punctuated::Punctuated, Token};
    use std::time::SystemTime;
    
    // Parse arguments: start_time, end_time, error_message
    let parser = Punctuated::<LitStr, Token![,]>::parse_separated_nonempty;
    let args = parser.parse(input).expect("Expected: start_time, end_time, error_message");
    let args_vec: Vec<_> = args.into_iter().collect();
    
    if args_vec.len() != 3 {
        panic!("compile_time! requires 3 arguments: start_time, end_time, error_message");
    }
    
    let start_str = args_vec[0].value();
    let end_str = args_vec[1].value();
    let error_msg = args_vec[2].value();
    
    // Parse time strings - support both HH:MM and YYYY-MM-DD HH:MM formats
    let parse_time = |time_str: &str| -> Result<(u32, u32, u32, u32, u32), String> {
        if time_str.contains('-') {
            // Full datetime format: YYYY-MM-DD HH:MM
            let parts: Vec<&str> = time_str.split(' ').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid datetime format: {}", time_str));
            }
            let date_parts: Vec<&str> = parts[0].split('-').collect();
            let time_parts: Vec<&str> = parts[1].split(':').collect();
            if date_parts.len() != 3 || time_parts.len() != 2 {
                return Err(format!("Invalid datetime format: {}", time_str));
            }
            Ok((
                date_parts[0].parse().map_err(|_| "Invalid year")?,
                date_parts[1].parse().map_err(|_| "Invalid month")?,
                date_parts[2].parse().map_err(|_| "Invalid day")?,
                time_parts[0].parse().map_err(|_| "Invalid hour")?,
                time_parts[1].parse().map_err(|_| "Invalid minute")?,
            ))
        } else {
            // Time only format: HH:MM (use today's date)
            let time_parts: Vec<&str> = time_str.split(':').collect();
            if time_parts.len() != 2 {
                return Err(format!("Invalid time format: {}", time_str));
            }
            Ok((
                0, 0, 0,
                time_parts[0].parse().map_err(|_| "Invalid hour")?,
                time_parts[1].parse().map_err(|_| "Invalid minute")?,
            ))
        }
    };
    
    let start_time = parse_time(&start_str).expect("Failed to parse start time");
    let end_time = parse_time(&end_str).expect("Failed to parse end time");
    
    // Get current time
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("System time error");
    let secs = now.as_secs();
    
    // Simple time check (this is approximate - a real implementation would use chrono)
    let (_, _, _, start_hour, start_min) = start_time;
    let (_, _, _, end_hour, end_min) = end_time;
    
    // Get current hour and minute (rough approximation)
    let current_hour = ((secs / 3600) % 24) as u32;
    let current_min = ((secs / 60) % 60) as u32;
    let current_time_mins = current_hour * 60 + current_min;
    let start_time_mins = start_hour * 60 + start_min;
    let end_time_mins = end_hour * 60 + end_min;
    
    // Check if current time is within allowed range
    let is_allowed = if start_time_mins <= end_time_mins {
        current_time_mins >= start_time_mins && current_time_mins <= end_time_mins
    } else {
        // Handle overnight range
        current_time_mins >= start_time_mins || current_time_mins <= end_time_mins
    };
    
    if !is_allowed {
        return quote! {
            compile_error!(#error_msg);
        }.into();
    }
    
    quote! {
        // Compilation allowed at current time
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

/// A proc-macro attribute that compiles Brainfuck code into Rust code at compile time.
/// 
/// The macro takes one string literal: the Brainfuck code to compile
/// The generated function accepts an input string parameter for ',' commands.
/// 
/// Usage:
/// ```rust
/// #[brainfuck("+++++++++[>++++++++<-]>.")]
/// fn hello() {}
/// 
/// println!("{}", hello(""));  // No input needed
/// ```
/// 
/// For BF code with input:
/// ```rust
/// #[brainfuck(",.,.")]
/// fn echo() {}
/// 
/// println!("{}", echo("Hi"));  // Outputs: Hi
/// ```
#[proc_macro_attribute]
pub fn brainfuck(attr: TokenStream, item: TokenStream) -> TokenStream {
    use syn::ItemFn;
    
    // Parse the attribute argument as a single string literal
    let bf_code = parse_macro_input!(attr as LitStr).value();
    
    // Parse the item to get the function name
    let func = syn::parse::<ItemFn>(item).expect("brainfuck attribute can only be applied to functions");
    let func_name = func.sig.ident;
    
    // Compile the brainfuck code into Rust code
    let mut rust_code = format!("fn {}(input: &str) -> String {{\n", func_name);
    rust_code.push_str("    let mut memory = vec![0u8; 30000];\n");
    rust_code.push_str("    let mut ptr = 0usize;\n");
    rust_code.push_str("    let mut output = String::new();\n");
    rust_code.push_str("    let input_bytes = input.as_bytes();\n");
    rust_code.push_str("    let mut input_ptr = 0usize;\n");
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
                rust_code.push_str(&format!("{}if input_ptr < input_bytes.len() {{\n", spaces));
                rust_code.push_str(&format!("{}    memory[ptr] = input_bytes[input_ptr];\n", spaces));
                rust_code.push_str(&format!("{}    input_ptr += 1;\n", spaces));
                rust_code.push_str(&format!("{}}}\n", spaces));
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


