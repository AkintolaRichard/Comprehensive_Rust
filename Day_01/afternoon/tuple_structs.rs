/*
Tuple Structs
If the field names are unimportant, you can use a tuple struct:
*/
struct Point(i32, i32);

fn main() {
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);
}

/*
This is often ised for single-field wrappers (called newtypes)
*/
struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    todo!("Ask a rocket scientist at NASA")
}

fn set_thruster_force(force: Newtons) {
    // ..
}

fn main() {
    let force = compute_thruster_force();
    set_thruster_force(force);
}
