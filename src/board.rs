use rand::Rng;

// BOARD.rs
// @TODO
// static methods
// [ ] Board::position_to_index(pos) -> i8
// [ ] Board::index_to_position(index) -> position
// [ ] Board::get_neighbor_position(pos, directioin) -> position
// instance methods
// [x] board.print()
// [ ] board.check_endgame_condition() -> Option<Player>
// [ ] board.drop(Column, Player) -> bool // bool indicates whether drop was successfull, if impossible => return false
// [ ] board.get_col(Column) -> Vec<GameSlots>
// [ ] board.get_open_slot(Column) -> Option<i8> //index of the open slot in that column
pub struct Board {
    slots: [SlotState; 7 * 6], // 7 cols * 6 rows
}

impl Board {
    fn new() -> Board {
        Board {
            slots: [SlotState::Empty; 7 * 6],
        }
    }

    // just for testing
    pub fn random() -> Board {
        let mut slots = [SlotState::Empty; 7 * 6];
        let mut c = 0;
        while c < 7 * 6 {
            slots[c] = random_slot();
            c += 1;
        }
        Board { slots: slots }
    }

    pub fn print(&self) {
        self.slots.iter().enumerate().for_each(|(indx, slot)| {
            if indx == 0 {
                // opening pipe
                print!("|");
            }
            match slot {
                SlotState::Empty => print!("  |"),
                SlotState::Occupied(p) => match p {
                    Player::One => print!("🔴|"),
                    Player::Two => print!("🟡|"),
                },
            }
            if (indx + 1) % 7 == 0 && indx != 41 {
                // breaks line after 7 items, must be omitted for the 42nd element
                print!("\n|");
            }
        })
    }

    // should panic for positions that are out of bound
    fn position_to_index(position: Position) -> i8 {
        position.row*7 + position.col
    }

    // should panic! if index out of bounds (< 0 || >= 42)
    fn index_to_position(index: i8) -> Position {
        Position { col: index%7 , row: index/6 - 1 }
    }
}

#[derive(Copy, Clone, Debug)]
enum SlotState {
    Empty,
    Occupied(Player),
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
    Two,
}


// player action description
enum DropInColumn {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

struct Position {
    col: i8,
    row: i8,
}

// all directions needed to calculate endgame condition
enum Direction {
    LeftDown,
    Down,
    DownRight,
    Right,
}



























/*
 * UNIT TESTS
 */
#[cfg(test)]
mod tests {
    use crate::board::*;

    // helper
    fn position_to_index_test_helper(row: i8, col: i8, target_index: i8) {
        let actual_index = Board::position_to_index(Position { row: row, col: col });
        assert_eq!(actual_index, target_index);
    }

    // helper
    fn index_to_position_test_helper(start_index: i8, target_pos: Position) {
        let actual_position = Board::index_to_position(start_index);
        assert_eq!(actual_position.row, target_pos.row);
        assert_eq!(actual_position.col, target_pos.col);
    }

    #[test]
    fn position_to_index_test() {
        // test all 42 valid positions
        let mut target_index = 0;
        for col in 0..7 {
            for row in 0..6 {
                position_to_index_test_helper(row, col, target_index);
                target_index += 1;
            }
        }
    }

    // OUT OF BOUND SHOULD ALWAYS PANIC!
    #[test]
    #[should_panic]
    fn position_to_index_left_panic_test() {
        Board::position_to_index(Position { col: -1, row: 0 });
    }

    #[test]
    #[should_panic]
    fn position_to_index_right_panic_test() {
        Board::position_to_index(Position { col: 7, row: 0 });
    }

    #[test]
    #[should_panic]
    fn position_to_index_top_panic_test() {
        Board::position_to_index(Position { col: 0, row: -1 });
    }

    #[test]
    #[should_panic]
    fn position_to_index_bottom_panic_test() {
        Board::position_to_index(Position { col: 0, row: 6 });
    }

    // INDEX_TO_POSITION
    #[test]
    fn index_to_position_test() {
        index_to_position_test_helper(0, Position { col: 0, row: 0});
        index_to_position_test_helper(0, Position { col: 0, row: 0});
        index_to_position_test_helper(0, Position { col: 0, row: 0});
        index_to_position_test_helper(0, Position { col: 0, row: 0});
        index_to_position_test_helper(0, Position { col: 0, row: 0});
    }
    
    #[test]
    #[should_panic]
    fn index_to_position_panic_test_1() {
        Board::index_to_position(-1); // out of bound should panic so we can catch bugs
    }
    
    #[test]
    #[should_panic]
    fn index_to_position_panic_test_2() {
        Board::index_to_position(42); // out of bound should panic so we can catch bugs
    }
}
