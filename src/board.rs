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

const ROWS: usize = 6;
const COLUMNS: usize = 7;
const WIN_SEQUENCE: usize = 4;

pub struct Board {
    slots: [SlotState; (COLUMNS * ROWS) as usize], // 7 cols * 6 rows
}

impl Board {
    fn new() -> Board {
        Board {
            slots: [SlotState::Empty; (COLUMNS * ROWS) as usize],
        }
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

    pub fn set_slot_state(&mut self, position: Position, player: Player) {
        let target_index = Board::position_to_index(&position);
        let target_slot_state = SlotState::Occupied(player); 
        self.slots[target_index] = target_slot_state;
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
            if (indx + 1) % COLUMNS == 0 && indx != COLUMNS * ROWS - 1 {
                // breaks line after 7 items, must be omitted for the 42nd element
                print!("\n|");
            }
        })
    }

    // should panic for positions that are out of bound
    fn position_to_index(position: &Position) -> usize {
        match position.col < COLUMNS && position.row < ROWS {
            true => position.row * COLUMNS + position.col,
            false => panic!("Position out of bounds!"),
        }
    }

    // should panic! if index out of bounds (< 0 || >= 42)
    fn index_to_position(index: usize) -> Position {
        match index < COLUMNS * ROWS {
            true => Position {
                col: index % COLUMNS,
                row: index / COLUMNS,
            },
            false => panic!("Index out of bounds!"),
        }
    }

    fn get_slot(&self, pos: &Position) -> SlotState {
        self.slots[Board::position_to_index(&pos)]
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

fn directional_neighbor(pos: &Position, direction: &WinPathDirection) -> Option<Position> {
    fn valid_option(pos: Position) -> Option<Position> {
        if pos.col < COLUMNS && pos.row < ROWS {
            Some(pos)
        } else {
            None
        }
    }

    match direction {
        WinPathDirection::Right => valid_option(Position {
            col: pos.col + 1,
            row: pos.row,
        }),
        WinPathDirection::Down => valid_option(Position {
            col: pos.col,
            row: pos.row + 1,
        }),
        WinPathDirection::LowerRight => valid_option(Position {
            col: pos.col + 1,
            row: pos.row + 1,
        }),
        WinPathDirection::UpperRight => {
            if pos.row > 0 {
                valid_option(Position {
                    col: pos.col + 1,
                    row: pos.row - 1,
                })
            } else {
                None
            }
        }
    }
}

#[derive(Debug)]
pub struct WinInfo {
    pub winner: Player,
    pub win_path: Vec<Position>,
}

pub fn find_win(board: &Board) -> Option<WinInfo> {
    for index in 0..board.slots.len() {
        let pos = Board::index_to_position(index);
        let mabye_win_info = find_win_for_position(board, &pos);
        match mabye_win_info {
            Some(win_info) => return Some(win_info),
            _ => (),
        }
    }
    None
}

fn find_win_for_position(board: &Board, from: &Position) -> Option<WinInfo> {
    println!("===== find_for_pos: c={} r={} =====", from.col, from.row);
    let mut queue: Vec<(Position, Vec<Position>, WinPathDirection)> = vec![];

    let player = match board.get_slot(&from) {
        SlotState::Empty => return None, // if slot is empty, no winner can be found => return None
        SlotState::Occupied(player) => player,
    };

    // init breadth first search by adding all elements to queue
    queue.push((from.clone(), vec![], WinPathDirection::Down));
    queue.push((from.clone(), vec![], WinPathDirection::Right));
    queue.push((from.clone(), vec![], WinPathDirection::LowerRight));
    queue.push((from.clone(), vec![], WinPathDirection::UpperRight));
    
    
    while !queue.is_empty() {
        // pop element from queue
        let queue_element = queue.remove(0);
        let current_pos = queue_element.0;
        let mut path = queue_element.1;
        let direction = queue_element.2;

        println!("      queue-len: {} path-len: {}", queue.len(), path.len());
        
        if path.len() >= WIN_SEQUENCE - 1 {
            println!("      path.len >= WIN_SEQUENCE");
            path.push(current_pos);
            return Some(WinInfo {
                winner: player,
                win_path: path
            });
        }
        
        let possible_pos = directional_neighbor(&current_pos, &direction);
        if !possible_pos.is_none() {
            let next_pos = possible_pos.unwrap();
            let player_at = match board.get_slot(&next_pos) {
                SlotState::Empty => break,
                SlotState::Occupied(player) => player,
            };
            if player_at == player_at {
                println!("      same-player found");
                let mut new_path = path.clone();
                new_path.push(current_pos.clone());
                queue.push((next_pos, new_path, direction));
            }
        }
    }
    println!("      END queue-len: {}", queue.len());
    None
}

// all directions needed to calculate endgame condition
enum WinPathDirection {
    Right,
    Down,
    LowerRight,
    UpperRight,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    One,
    Two,
}

#[derive(Clone, Debug)]
pub struct Position {
    pub col: usize,
    pub row: usize,
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
    fn position_to_index_test_helper(col: usize, row: usize, target_index: usize) {
        let actual_index = Board::position_to_index(&Position { col: col, row: row });
        assert_eq!(actual_index, target_index);
    }

    // helper
    fn index_to_position_test_helper(start_index: usize, target_pos: Position) {
        let actual_position = Board::index_to_position(start_index);
        assert_eq!(actual_position, target_pos);
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

    #[test]
    #[should_panic]
    fn position_to_index_right_panic_test() {
        Board::position_to_index(&Position { col: 7, row: 0 });
    }

    #[test]
    #[should_panic]
    fn position_to_index_bottom_panic_test() {
        Board::position_to_index(&Position { col: 0, row: 6 });
    }

    // INDEX_TO_POSITION
    #[test]
    fn index_to_position_test() {
        index_to_position_test_helper(0, Position { col: 0, row: 0 });
        index_to_position_test_helper(0, Position { col: 0, row: 0 });
        index_to_position_test_helper(0, Position { col: 0, row: 0 });
        index_to_position_test_helper(0, Position { col: 0, row: 0 });
        index_to_position_test_helper(0, Position { col: 0, row: 0 });
    }

    #[test]
    #[should_panic]
    fn index_to_position_panic_test_2() {
        Board::index_to_position(42); // out of bound should panic so we can catch bugs
    }
}
