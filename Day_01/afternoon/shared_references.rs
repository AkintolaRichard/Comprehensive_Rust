/*
Shared References
A reference provides a way to access another value without taking ownership
of the value, and is also called "borrowing". Shared references are
read-only, and the referenced data cannot change.
*/
fn main() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);
}

/*
A shared reference to a type T has type &T. A reference value is made with
the & operator. The * operator "dereferences" a reference, yielding its
value.
* References can never be null in Rust, so null checking is not necessary
* Rust does not automatically create references for you - the & is always required.
*/
