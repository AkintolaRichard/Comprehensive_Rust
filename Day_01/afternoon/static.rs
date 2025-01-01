/*
static
Static variables will live during the whole execution of the program, and
therefore will not move.
*/

static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    println!("{BANNER}");
}
