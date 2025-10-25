use procmacros::compile_time;

// This example can only be compiled during specific hours
// Comment out or adjust the times to test
compile_time!("00:00", "23:59", "Compilation allowed at any time for demo purposes!");

fn main() {
    println!("Successfully compiled! The time restriction allowed this build.");
}
