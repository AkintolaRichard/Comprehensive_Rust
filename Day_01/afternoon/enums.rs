/*
Enums
The enum keyword allows the creation of a type which has few different variants
*/

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                           // Simple variant
    Run(Direction),                 // Tuple variant
    Teleport { x: u32, y: u32 },    // Struct variant
}

fn main() {
    let player_move: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("On this turn: {player_move:?}");
}
