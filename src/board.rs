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

/**
EXAMPLE BOARD
|  |🔴|🔴|🟡|  |  |🟡|
|🟡|🟡|  |  |  |  |🟡|
|🟡|  |  |🟡|🔴|🟡|🟡|
|🔴|🟡|🟡|  |🟡|🔴|🔴|
|  |🟡|🔴|🟡|  |🟡|🟡|
|🟡|  |  |  |🟡|  |🟡|
 */

// RULESET IS HARDCODED AS STANDARD VERSION FOR NOW
const ROWS: usize = 6; 
const COLS: usize = 7; 
const WIN_SEQUENCE: usize = 4;

pub struct Board {
    pub slots: [SlotState; (COLS * ROWS) as usize], // 7 cols * 6 rows
}

impl Board {
    pub fn new() -> Board {
        Board {
            slots: [SlotState::Empty; (COLS * ROWS) as usize],
        }
    }

    // just for testing
    pub fn random() -> Board {
        let mut slots = [SlotState::Empty; (COLS * ROWS) as usize];
        let mut c = 0;
        while c < COLS * ROWS {
            slots[c as usize] = random_slot();
            c += 1;
        }
        Board { slots: slots }
    }

    pub fn is_valid_position(col: i8, row: i8) -> bool {
        let col_is_out_of_bound = col < 0 || col >= (COLS as i8);
        let row_is_out_of_bound = row < 0 || row >= (ROWS as i8);
        !col_is_out_of_bound && !row_is_out_of_bound
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
            if (indx + 1) % COLS == 0 && indx != COLS * ROWS - 1 {
                // breaks line after 7 items, must be omitted for the 42nd element
                print!("\n|");
            }
        })
    }

    // should panic for positions that are out of bound
    pub fn position_to_index(position: &Position) -> usize {
        match position.col < COLS && position.row < ROWS {
            true => position.row * COLS + position.col,
            false => {
                panic!("Position out of bounds: Position: {:?} is_valid_position={:?}", position, Board::is_valid_position(position.col as i8, position.row as i8))
            }
        }
    }

    // should panic! if index out of bounds (< 0 || >= 42)
    pub fn index_to_position(index: usize) -> Position {
        match index < COLS * ROWS {
            true => Position {
                col: index % COLS,
                row: index / COLS,
            },
            false => panic!("Index out of bounds!"),
        }
    }

    fn get_slot(&self, pos: &Position) -> SlotState {
        self.slots[Board::position_to_index(&pos)]
    }

    pub fn winner_exists(&self) -> Option<WinInfo> {
        for index in 0..self.slots.len() {
            let pos = Board::index_to_position(index);
            let mabye_win_info = self.winner_exists_for_position(pos);
            match mabye_win_info {
                Some(win_info) => return Some(win_info),
                _ => (),
            }
        }
        None
    }

    // returns all neighbors in a given direction
    // => including the starting position
    // => skips neighbors if out of bounds
    fn get_directional_neighbors(&self, pos: &Position, direction: WinPathDirection, n_neighbors: usize) -> Vec<Position> {
        let offset: (i8, i8) = match direction {
            WinPathDirection::Right => (1,0),
            WinPathDirection::RightDown => (1,1),
            WinPathDirection::Down => (0,1),
            WinPathDirection::DownLeft => (-1,1),
        };
        let (col_offset, row_offset) = offset;

        let mut neighbors: Vec<Position> = vec![];
        for i in 1..=n_neighbors {
            // get new position by transforming (multiply with i)
            let new_col = (pos.col as i8) + col_offset * i as i8;
            let new_row = (pos.row as i8) + row_offset * i as i8;
            // check for out of bounds
            if Board::is_valid_position(new_col, new_row) {
                // transform to index
                let target_position = Position { 
                    col: new_col as usize, 
                    row: new_row as usize,
                };
                neighbors.push(target_position);
            }
        }
        neighbors
    }

    fn check_win_path(&self, neighbors: &Vec<Position>) -> Option<Player> {
        // cannot be empty
        if neighbors.len() == 0 {
            return None;
        }
        // cannot be smaller than win sequence
        if neighbors.len() < WIN_SEQUENCE {
            return None;
        }
        // check if all neighbors are the same Player
        // exclude SlotState::Empty
        let first_neighbor_index = Board::position_to_index(&neighbors[0]);
        let target_slot_state = self.slots[first_neighbor_index];
        match target_slot_state {
            SlotState::Empty => return None,
            SlotState::Occupied(player) => {
                let all_slots_same = neighbors
                    .iter()
                    .map(|position| self.slots[Board::position_to_index(&position)])
                    .all(|slot_state| slot_state == target_slot_state);
                return match all_slots_same {
                    true => Some(player),
                    false => None,
                }
            }
        }
    }

    fn winner_exists_for_position(&self, pos: Position) -> Option<WinInfo> {
        let all_possible_directions = [
            WinPathDirection::Right,
            WinPathDirection::RightDown,
            WinPathDirection::Down,
            WinPathDirection::DownLeft
        ];
        for win_path_direction in all_possible_directions {
            let neighbors = self.get_directional_neighbors(
                &pos, 
                win_path_direction, 
                WIN_SEQUENCE
            );
            // return WinInfo if found, otherwise continue search
            let maybe_player: Option<Player> = self.check_win_path(&neighbors);
            match maybe_player {
                Some(player) => return Some(WinInfo::new(player, neighbors)),
                _ => (),
            }
        }
        // extended search done, no winner found!
        None
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Debug)]
pub struct WinInfo {
    pub winner: Player,
    pub win_path: Vec<Position>,
}

impl WinInfo {
    fn new(winner: Player, win_path: Vec<Position>) -> WinInfo {
        WinInfo {
            winner: winner,
            win_path: win_path,
        }
    }
}

// all directions needed to calculate endgame condition
enum WinPathDirection {
    Right,
    RightDown,
    Down,
    DownLeft,
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
