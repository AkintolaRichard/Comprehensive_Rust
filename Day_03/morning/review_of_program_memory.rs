/*
Review of Program Memory
Programs allocate memory in two ways:
* Stacks: Continuous area of memory for local variables.
    * Values have fixed sizes known at compile time.
    * Extremely fast: just move a stack pointer.
    * Easy to manage: follows function calls.
    * Great memory locality.
* Heaps: Storaage of values outside of function calls.
    * Values have dynamic sizes determined at runtime.
    * Slightly slower than the stack: some book-keeping needed.
    * No guarantee of memory locality.
*/

/*
Example
Creating a String puts fixed-sized metadata on the stack and dynamically sized data, the actual string, on the heap:

fn main() {
    let s1 = String::from("Hello");
}

*/

/*
We can inspect the memory layout with unsafe Rust.
*/

fn main() {
    let mut s1 = String::from("Hello");
    s1.push(' ');
    s1.push_str("world");
    // DON'T DO THIS AT HOME! For educational purposes only.
    // String provides no guaranteed about its layout, so this could lead to
    // undefined behaviour.
    unsafe {
        let (capacity, ptr, len): (usize, usize, usize) = std::mem::transmute(s1);
        println!("capacity = {capacity}, ptr = {ptr:#x}, len = {len}");
    }
}
