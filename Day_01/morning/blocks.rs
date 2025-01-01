/*
Blocks:
A block in Rust contains a sequence of expressions, enclosed by braces {}.
Each block has a value and a type, which are those of the last expression
of the block.
*/
fn main() {
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");
}
/*
if the last expression ends with ;, then the resulting value and type is ()
*/
