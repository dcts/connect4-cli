use rand::Rng;

// BOARD.rs
// @TODO
// static methods
// [ ] Board::position_to_index(pos) -> i8
// [ ] Board::index_to_position(index) -> position
// [ ] Board::get_neighbor_position(pos, directioin) -> position
// instance methods
// [x] board.print()
// [ ] board.has_valid_neighbors(Position, Direction) -> bool
// [ ] board.get_slot_state(Position) -> Option<SlotState>
// [ ] board.check_endgame_condition() -> Option<Player>
// [ ] board.check_endgame_condition_for_slot(Position, Direction) -> Option<Player>
// [ ] board.drop(Column, Player) -> bool // bool indicates whether drop was successfull, if impossible => return false
// [ ] board.get_col(Column) -> Vec<GameSlots>
// [ ] board.get_open_slot(Column) -> Option<i8> //index of the open slot in that column

const ROWS: i8 = 6;
const COLUMNS: i8 = 7;
const WIN_SEQUENCE: i8 = 4;

pub struct Board {
    slots: [SlotState; (COLUMNS * ROWS) as usize], // 7 cols * 6 rows
}

impl Board {
    fn new() -> Board {
        Board {
            slots: [SlotState::Empty; (COLUMNS * ROWS) as usize],
        }
    }

    pub fn get_slot_state(&self, position: Position) -> Option<SlotState> {
        let index_option = Board::position_to_index(position).ok();
        match index_option {
            Some(index) => Some(self.slots[index as usize]),
            _ => None,
        }
    }

    pub fn check_endgame_condition_for_position(&self, position: Position, direction: Direction) -> Option<Player> {
        // dont bother checking when SlotState is empty
        let index = Board::position_to_index(position);
        let slot_state = self.slots[index as usize];
        if let SlotState::Empty = slot_state {
            return None;
        }

        match direction {
            Direction::Right => (),
            Direction::DownRight => (),
            Direction::Down => (),
            Direction::LeftDown => (),
        }
        
        // match over all directions
        //     => for every direction
        //         => check if it has valid neighbors? => NO => exit out of this direction
        //         => get neighbors
        //         => all neighbors same as player

        None


        


        // fn same_neighbors(starting_position: Position, direction: Direction) -> () {
        //     let starting_player_index = Board::position_to_index(starting_position);
        //     // let starting_player = self.slots[]
        // }
        // match direction {
        //     Direction::Right => (),
        //     Direction::DownRight => (),
        //     Direction::Down => (),
        //     Direction::LeftDown => (),
        // }
    }

    // just for testing
    pub fn random() -> Board {
        let mut slots = [SlotState::Empty; (COLUMNS * ROWS) as usize];
        let mut c = 0;
        while c < COLUMNS * ROWS {
            slots[c as usize] = random_slot();
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
            if (indx + 1) as i8 % COLUMNS == 0 && indx as i8 != COLUMNS * ROWS - 1 {
                // breaks line after 7 items, must be omitted for the 42nd element
                print!("\n|");
            }
        })
    }
    
    // should panic for positions that are out of bound
    fn position_to_index(position: Position) -> i8 {
        match position.col >= 0 && position.col < COLUMNS && position.row >= 0 && position.row < ROWS {
            true => position.row*COLUMNS + position.col,
            false => panic!("Position out of bounds!"),
        }
    }
    // should panic! if index out of bounds (< 0 || >= 42)
    fn index_to_position(index: i8) -> Result<Position, String> {
        match index >= 0 && index < COLUMNS*ROWS {
            true => Ok(Position { col: index%COLUMNS , row: index/COLUMNS }),
            false => Err("Index out of bounds!".to_string())
        }
    }
    
    fn has_valid_neighbors(position: Position, direction: Direction) -> bool {
        for offset in 1..WIN_SEQUENCE {
            match direction {
                // Direction::DownLeft => {
                //     if !is_valid_position() {
                //         return false;
                //     }
                // },
                // Direction::Down => {

                // },
                // Direction::DownRight => {

                // },
                // Direction::Right => {

                // },
            }
            // get_neighbor(starting_position, direction: Direction, offset: i8);
        }

        true
    }

    // check if position is valid
    fn is_valid_position(position: Position) -> bool {
        let col = position.col;
        let row = position.row;
        let col_out_of_bounds = col < 0 || col >= COLUMNS;
        let row_out_of_bounds = row < 0 || row >= ROWS;

        !row_out_of_bounds && !col_out_of_bounds
    }

    // UNDO THE RESULT RETURN
    // fn position_to_index(position: Position) -> Result<i8, String> {
    //     match position.col >= 0 && position.col < COLUMNS && position.row >= 0 && position.row < ROWS {
    //         true => Ok(position.row*COLUMNS + position.col),
    //         false => Err("Position out of bounds!".to_string()),
    //     }
    // }
    
    // UNDO THE RESULT RETURN
    // fn index_to_position(index: i8) -> Result<Position, String> {
    //     match index >= 0 && index < COLUMNS*ROWS {
    //         true => Ok(Position { col: index%COLUMNS , row: index/COLUMNS }),
    //         false => Err("Index out of bounds!".to_string())
    //     }
    // }
}

#[derive(Copy, Clone, Debug)]
pub enum SlotState {
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

#[derive(Debug)]
pub struct Position {
    pub col: i8,
    pub row: i8,
}

// all directions needed to calculate endgame condition
enum Direction {
    LeftDown,
    Down,
    DownRight,
    Right,
}






impl PartialEq for Position {
	fn eq(&self, other: &Self) -> bool {
		self.col == other.col && self.row == other.row
	}
}





















/*
 * UNIT TESTS
 */
#[cfg(test)]
mod tests {
    use crate::board::*;

    // helper
    fn position_to_index_test_helper(col: i8, row: i8, target_index: i8) {
        let actual_index = Board::position_to_index(Position { col: col, row: row });
        assert_eq!(actual_index, Ok(target_index));
    }

    // helper
    fn index_to_position_test_helper(start_index: i8, target_pos: Position) {
        let actual_position = Board::index_to_position(start_index);
        assert_eq!(actual_position, Ok(target_pos));
    }

    #[test]
    fn position_to_index_test() {
        // test all 42 valid positions
        let mut target_index = 0;
        for row in 0..6 {
            for col in 0..7 {
                position_to_index_test_helper(col, row, target_index);
                target_index += 1;
            }
        }
    }

    // // OUT OF BOUND SHOULD ALWAYS PANIC!
    // #[test]
    // #[should_panic]
    // fn position_to_index_left_panic_test() {
    //     Board::position_to_index(Position { col: -1, row: 0 });
    // }

    // #[test]
    // #[should_panic]
    // fn position_to_index_right_panic_test() {
    //     Board::position_to_index(Position { col: 7, row: 0 });
    // }

    // #[test]
    // #[should_panic]
    // fn position_to_index_top_panic_test() {
    //     Board::position_to_index(Position { col: 0, row: -1 });
    // }

    // #[test]
    // #[should_panic]
    // fn position_to_index_bottom_panic_test() {
    //     Board::position_to_index(Position { col: 0, row: 6 });
    // }

    // INDEX_TO_POSITION
    #[test]
    fn index_to_position_test() {
        index_to_position_test_helper(0, Position { col: 0, row: 0});
        index_to_position_test_helper(0, Position { col: 0, row: 0});
        index_to_position_test_helper(0, Position { col: 0, row: 0});
        index_to_position_test_helper(0, Position { col: 0, row: 0});
        index_to_position_test_helper(0, Position { col: 0, row: 0});
    }

    // #[test]
    // #[should_panic]
    // fn index_to_position_panic_test_1() {
    //     Board::index_to_position(-1); // out of bound should panic so we can catch bugs
    // }

    // #[test]
    // #[should_panic]
    // fn index_to_position_panic_test_2() {
    //     Board::index_to_position(42); // out of bound should panic so we can catch bugs
    // }
}
