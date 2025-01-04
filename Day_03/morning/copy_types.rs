/*
Copy Types
While move semantics are the default, certain types are copied by default.
*/

/*fn main() {
    let x = 42;
    let y = x;
    println!("x: {x}"); // would not be accessible if not Copy
    println!("y: {y}");
}*/

/*
These types implement the Copy trait.

You can opt-in your own types to use copy semantics
*/

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn main() {
    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}

/*
 * After the assignment, both p1 and p2 own their own data.
 * We can also use p1.clone() to explicitly copy the data.

Copying and cloning are not the same thing
* Copying refers to bitwise copies of memory regions and does not work on
 arbitrary objects.
* Copying does not allow for custom logic (unlike copy constructors in C++)
* Cloning is a more general operation and also allows for custom behaviour
 by implementing the Clone trait.
* Copying does not work on types that implement the Drop trait.
*/
