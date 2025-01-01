/*
Deriving
Derivation is implemented with macros, and many crates provide useful
derive macros to add useful functionality. For example, serde can derive
serialization support for a struct using #[derive(Serialize)]
*/

#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn main() {
    let p1 = Player::default(); // Default trait adds `default` constructor
    let mut p2 = p1.clone();    // Clone trait adds `close` method.
    p2.name = String::from("EldurScroll");
    // Debug trait adds support for printing with `{:?}`.
    println!("{p1:?} vs. {p2:?}");
}
