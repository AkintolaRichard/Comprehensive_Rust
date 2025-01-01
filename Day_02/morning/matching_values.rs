/*
Matching Values
The match keyword lets you match a value against one or more patterns. The
patterns can be simple values, similarly to switch in C and C++, but they
can also be used to express more complex conditions.
*/
#[rustfmt::skip]
fn main() {
    let input = 'x';
    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving round"),
        '0'..='9' => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _ => println!("Something else"),
    }
    // More to Explore
    let opt = Some(123);
    match opt {
        outer @ Some(inner) => {
            println!("outer: {outer:?}, inner: {inner}");
        }
        None => {}
    }
}
/*
Another piece of pattern syntax is the @ syntax which binds a part of a
pattern to a variable.
In this example inner has the value 123 which is pulled from the Option
via destructuring, outer captures the entire Some(inner) expression, so
it contains the full Option::Some(123). This is rarely used but it can be
useful in more complex patterns.
*/
