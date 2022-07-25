
mod board;
use std::thread::current;

use board::*;

use console::Term;
use console::Key;

mod network;
use network::server::GameSocket;

use std::{thread, time};

fn main() {

    println!("Connect 4!");
    
    // DROP PIECE DEMO
    let mut board = Board::new();
    board.drop_piece(1, Player::One);
    board.drop_piece(1, Player::Two);
    board.drop_piece(1, Player::One);
    board.drop_piece(1, Player::Two);
    board.print(0);

    // GAME LOGIC DEMO
    // let board = Board::random();
    // board.print(100);
    // let maybe_win_info = board.winner_exists();
    
    // match maybe_win_info {
    //     Some(win_info) => {
    //         println!("\n\n\nWin Info: \n{:?}\n", win_info);
    //         // show isolated sequence in a new board
    //         let mut win_board = Board::new();
    //         win_info.win_path.iter().for_each(|pos| {
    //             let indx = Board::position_to_index(pos);
    //             win_board.slots[indx] = SlotState::Occupied(win_info.winner);
    //         });
    //         win_board.print(100);
    //     },
    //     None => println!("\n\n\nno win found!"),
    // }
}


// // GRAPHICS DEMO => move cursor MVP
// let board = Board::new();
// let stdout = Term::buffered_stdout();
// let mut current_selection = 0;
// board.print(current_selection);
// 'game_loop: loop {
//     if let Ok(key) = stdout.read_key() {
//         match key {
//             Key::ArrowLeft => {
//                 current_selection -= 1;
//                 current_selection = std::cmp::max(std::cmp::min(7, current_selection), 0);
//             },
//             Key::ArrowRight => {
//                 current_selection += 1;
//                 current_selection = std::cmp::max(std::cmp::min(7, current_selection), 0);
//             },
//             _ => println!("key: {:?}", key),
//         }
//         board.print(current_selection);
//     }
// }
      
//     // // BOARD DEMO OLD
//     // let board = Board::random();
//     // board.print();

//     // // networking demo
//     // let connection_string = String::from("0.0.0.0:3042");
//     // let socket = GameSocket::new(connection_string);


//     // change col graphics
//     let mut board = Board::new();
//     board.set_slot_state(Position::new(2, 5), Player::One);
//     board.set_slot_state(Position::new(3, 5), Player::Two);
//     board.print(0);

//     // // DROP PIN ANIMATION DEMO
//     // let mut board = Board::new();
//     // board.print();
//     // sleep(2000);
//     // board.slots[Board::position_to_index(&Position::new(1,0))] = SlotState::Occupied(Player::One);
//     // board.print();
//     // sleep(50);
//     // board.slots[Board::position_to_index(&Position::new(1,0))] = SlotState::Empty;
//     // board.slots[Board::position_to_index(&Position::new(1,1))] = SlotState::Occupied(Player::One);
//     // board.print();
//     // sleep(50);
//     // board.slots[Board::position_to_index(&Position::new(1,1))] = SlotState::Empty;
//     // board.slots[Board::position_to_index(&Position::new(1,2))] = SlotState::Occupied(Player::One);
//     // board.print();
//     // sleep(50);
//     // board.slots[Board::position_to_index(&Position::new(1,2))] = SlotState::Empty;
//     // board.slots[Board::position_to_index(&Position::new(1,3))] = SlotState::Occupied(Player::One);
//     // board.print();
//     // sleep(50);
//     // board.slots[Board::position_to_index(&Position::new(1,3))] = SlotState::Empty;
//     // board.slots[Board::position_to_index(&Position::new(1,4))] = SlotState::Occupied(Player::One);
//     // board.print();
//     // sleep(50);
//     // board.slots[Board::position_to_index(&Position::new(1,4))] = SlotState::Empty;
//     // board.slots[Board::position_to_index(&Position::new(1,5))] = SlotState::Occupied(Player::One);
//     // board.print();

//     // sleep(2000);
//     // board.slots[Board::position_to_index(&Position::new(2,0))] = SlotState::Occupied(Player::Two);
//     // board.print();
//     // sleep(50);
//     // board.slots[Board::position_to_index(&Position::new(2,0))] = SlotState::Empty;
//     // board.slots[Board::position_to_index(&Position::new(2,1))] = SlotState::Occupied(Player::Two);
//     // board.print();
//     // sleep(50);
//     // board.slots[Board::position_to_index(&Position::new(2,1))] = SlotState::Empty;
//     // board.slots[Board::position_to_index(&Position::new(2,2))] = SlotState::Occupied(Player::Two);
//     // board.print();
//     // sleep(50);
//     // board.slots[Board::position_to_index(&Position::new(2,2))] = SlotState::Empty;
//     // board.slots[Board::position_to_index(&Position::new(2,3))] = SlotState::Occupied(Player::Two);
//     // board.print();
//     // sleep(50);
//     // board.slots[Board::position_to_index(&Position::new(2,3))] = SlotState::Empty;
//     // board.slots[Board::position_to_index(&Position::new(2,4))] = SlotState::Occupied(Player::Two);
//     // board.print();
//     // sleep(50);
//     // board.slots[Board::position_to_index(&Position::new(2,4))] = SlotState::Empty;
//     // board.slots[Board::position_to_index(&Position::new(2,5))] = SlotState::Occupied(Player::Two);
//     // board.print();


//     // sleep(2000);
//     // board.slots[Board::position_to_index(&Position::new(2,0))] = SlotState::Occupied(Player::One);
//     // board.print();
//     // sleep(50);
//     // board.slots[Board::position_to_index(&Position::new(2,0))] = SlotState::Empty;
//     // board.slots[Board::position_to_index(&Position::new(2,1))] = SlotState::Occupied(Player::One);
//     // board.print();
//     // sleep(50);
//     // board.slots[Board::position_to_index(&Position::new(2,1))] = SlotState::Empty;
//     // board.slots[Board::position_to_index(&Position::new(2,2))] = SlotState::Occupied(Player::One);
//     // board.print();
//     // sleep(50);
//     // board.slots[Board::position_to_index(&Position::new(2,2))] = SlotState::Empty;
//     // board.slots[Board::position_to_index(&Position::new(2,3))] = SlotState::Occupied(Player::One);
//     // board.print();
//     // sleep(50);
//     // board.slots[Board::position_to_index(&Position::new(2,3))] = SlotState::Empty;
//     // board.slots[Board::position_to_index(&Position::new(2,4))] = SlotState::Occupied(Player::One);
//     // board.print();
// }

// // GRAPHICS "ENGINE"
// pub fn sleep(sleep_time_in_ms: u64) {
//     println!("");
//     thread::sleep(time::Duration::from_millis(sleep_time_in_ms));
// }