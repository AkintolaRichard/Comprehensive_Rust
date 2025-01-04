/*
Move Semantics
An assignment will transfer ownership between variables
*/

fn main() {
    let s1: String = String::from("Hello");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");
}

/*
 * The assignment of s1 to s2 transfers ownership.
 * When s1 goes out of scope, nothing happens: it does not own anything.
 * When s2 goes out of scope, the string data is freed.
*/

/*
When you pass a value to a function, the value is assigned to the function
parameter. This transfers ownership.
*/

/*fn say_hello(name: String) {
    println!("Hello {name}");
}

fn main() {
    let name = String::from("Alice");
    say_hello(name);
    say_hello(name);
}*/
