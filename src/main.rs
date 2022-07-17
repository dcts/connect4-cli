fn main() {
    println!("Connect 4!");
}

// BOARD.rs
struct Board {
    matrix: Vec<i32>
}

enum SlotState {
    Empty,
    Occupied(Player)
}

enum Player {
    One,
    Two
}

enum Column {
    One, 
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven
}