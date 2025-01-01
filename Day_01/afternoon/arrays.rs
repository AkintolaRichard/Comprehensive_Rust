/*
 Arrays
A value of the array type [T; N] holds N (a compile-time constant) elements
of the same type T.
The println! macro asks for the debug implementation with the ? format
parameter: {} gives the default output, {:?} gives the debug output.
Types such as integers and strings implement the default output, but arrays
only implement the debug output.
Adding #, e.g {a:#?}, invokes a "pretty printing" format.
*/
fn main() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}");
}
