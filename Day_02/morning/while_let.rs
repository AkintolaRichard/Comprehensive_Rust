fn main() {
    let mut name = String::from("Comprehensive Rust");
    while let Some(c) = name.pop() {
        println!("character: {c}");
    }
    // (There are more efficient ways to reverse a string!)
}
