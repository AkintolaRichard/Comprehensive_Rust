/*
Borrow Checking
Rust's borrow checker puts constraints on the ways you can borrow values.
We've already seen that a reference cannot outlive the value it borrows.
*/

/*fn main() {
    let x_ref = {
        let x = 10;
        &x
    };
    println!("x: {x_ref}");
}*/

/*
There's also a second main rule that the borrow checker enforces: The
aliasing rule. For a given value, at any time:
 * You can have one or more shared references to the value, or
 * You can have exactly one exclusive reference to the value.
*/
fn main() {
    let mut a: i32 = 10;
    let b: &i32 = &a;
    println!("b: {b}");

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
    //println!("b: {b}");
}
