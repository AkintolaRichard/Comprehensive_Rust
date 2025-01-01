/*
Strings
We can now understand the two string type in Rust:
 * &str is a slice of UTF-8 encoded bytes, similar to &[u8].
 * String is an owned buffer og UTF-8 encoded bytes, similar to Vec<T>.
*/
fn main() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[s2.len() - s1.len()..];
    println!("s3: {s3}");
}
