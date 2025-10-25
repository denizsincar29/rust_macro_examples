use procmacros::brainfuck;

// Classic "Hello World!" in Brainfuck
#[brainfuck("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.", "")]
fn hello_world() {}

// Simple example that echoes input
#[brainfuck(",.", "A")]
fn echo_a() {}

// Example that reads two bytes and outputs both
#[brainfuck(",.,.", "Hi")]
fn echo_two() {}

// Example that increments input by 1
#[brainfuck(",+.", "A")]
fn increment() {}

fn main() {
    println!("Hello World output: {}", hello_world());
    println!("Echo A output: {}", echo_a());
    println!("Echo two output: {}", echo_two());
    println!("Increment A output: {}", increment());
}
