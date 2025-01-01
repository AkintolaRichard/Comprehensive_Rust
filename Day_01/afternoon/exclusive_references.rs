/*
Exclusive References:
Exclusive references, also known as mutable references, allow changing the
value they refer to. They have type &mut T.
*/
fn main() {
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("point: {point:?}");
}
