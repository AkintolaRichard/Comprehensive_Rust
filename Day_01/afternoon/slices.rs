/*
Slices:
A slice gives you a view into a larger collection:
* Slices borrow data from the sliced type.
*/
fn main() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];

    println!("s: {s:?}");
}

/*
 * We create a slice by borrowing a and specifying the starting and ending indexes in brackets.
 * s i a reference to a slice of i32s. Notice that the type of s (&[i32])
 no longer mention the array length. This allows us to perform computation
 on slices of different sizes.
 * Slices always borrow from another object. In this example, a has to
 remain 'alive' (in scope) for at least as long as our slice.
*/
