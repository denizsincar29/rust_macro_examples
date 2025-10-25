use procmacros::compile_time;

// This example can only be compiled during specific hours
// Comment out or adjust the times to test
compile_time!("18:00", "09:00", "Compilation allowed only between 9 AM and 6 PM");

fn main() {
    println!("Successfully compiled! The time restriction allowed this build.");
}
