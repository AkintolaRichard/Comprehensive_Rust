/*
Rust provides type safety via static typing. Variable bindings are made
with let:
*/
fn main() {
    let x: i32 = 10;
    println!("x: {x}");
    // x = 20;
    // println!("x: {x}");
}
/*
The types have widths as follows:
* iN, uN, fN are N bits wide,
* isize and usize are the width of a pointer,
* char is 32 bits wide,
* bool is 8 bits wide.
*/
