/*
Reference Validity:
Rust enforces a number of rules for references that make them always safe
to use. One rule is that references can never be null, making them safe to
use without null checks. The other rule we'll look at for now is that
references can't outlive the data they point to.
*/
fn main() {
    let x_ref = {
        let x = 10;
        &x
    };
    println!("x: {x_ref}");
}
