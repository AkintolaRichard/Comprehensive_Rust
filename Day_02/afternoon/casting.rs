/*
Casting
Rust has no implicit type conversions, but it does support explicit casts
with as. These generally follow C semantics where those are defined.
*/
fn main() {
    let value: i16 = 1000;
    println!("as u16: {}", value as u16);
    println!("as i16: {}", value as i16);
    println!("as u8: {}", value as u8);
}

