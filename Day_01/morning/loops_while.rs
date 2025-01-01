/*
Loops:
There are three looping keywords in Rust: while, loop, and for.

while:
The while keyword works much like in other languages, executing the loop
body as loop as the condition is true.
*/
fn main() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("Final x: {x}");
}
