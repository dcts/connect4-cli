
use rand::Rng;

fn main() {
    println!("Connect 4!");
    let board = Board::random();
    board.print();
}

// BOARD.rs
// @TODO
// static methods
// [ ] Board::position_to_index(pos) -> index
// [ ] Board::index_to_position(index) -> position
// [ ] Board::get_neighbor_position(pos, directioin) -> position
// instance methods
// [x] board.print()
// [ ] board.check_endgame_condition() -> Option<Player>
// [ ] board.drop(Column, Player) -> bool // bool indicates whether drop was successfull, if impossible => return false
// [ ] board.get_col(Column) -> Vec<GameState>
// [ ] board.get_open_slot(Column) -> Option<i8> //index of the open slot in that column
struct Board {
    slots: [SlotState; 7*6] // 7 cols * 6 rows
}

impl Board {
    fn new() -> Board {
        Board {
            slots: [SlotState::Empty; 7*6]
        }
    }

    // just for testing
    fn random() -> Board {
        let mut slots = [SlotState::Empty; 7*6];
        let mut c = 0;
        while c < 7*6 {
            slots[c] = random_slot();
            c += 1;
        }
        Board { 
            slots: slots,
        }
    }

    fn print(&self) {
        let mut c = 0;
        for slot in self.slots {
            match slot {
                SlotState::Empty => print!("  |"),
                SlotState::Occupied(p) => match p {
                    Player::One => print!("🔴|"),
                    Player::Two => print!("🟡|"),
                }
            }
            if (c+1) % 7 == 0 {
                println!("");
            }
            c += 1;
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum SlotState {
    Empty,
    Occupied(Player)
}

fn random_slot() -> SlotState {
    match rand::thread_rng().gen_range(0..=2) {
        0 => SlotState::Empty,
        1 => SlotState::Occupied(Player::One),
        2 => SlotState::Occupied(Player::Two),
        _ => panic!("Randomly generated value is out of bound. Allowed 0-2."),
    }
}

#[derive(Copy, Clone, Debug)]
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

struct Position {
    row: i8,
    col: i8,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}