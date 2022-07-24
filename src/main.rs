mod board;
use board::*;

mod network;
use network::server::GameSocket;

use std::{thread, time};

fn main() {
    println!("Connect 4!");

    // // GAME LOGIC DEMO
    // let board = Board::random();
    // board.print();
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
    //         win_board.print();
    //     },
    //     None => println!("\n\n\nno win found!"),
    // }
      
    // // BOARD DEMO OLD
    // let board = Board::random();
    // board.print();

    // // networking demo
    // let connection_string = String::from("0.0.0.0:3042");
    // let socket = GameSocket::new(connection_string);

    // DROP PIN ANIMATION DEMO
    let mut board = Board::new();
    board.print();
    sleep(2000);
    board.slots[Board::position_to_index(&Position::new(1,0))] = SlotState::Occupied(Player::One);
    board.print();
    sleep(50);
    board.slots[Board::position_to_index(&Position::new(1,0))] = SlotState::Empty;
    board.slots[Board::position_to_index(&Position::new(1,1))] = SlotState::Occupied(Player::One);
    board.print();
    sleep(50);
    board.slots[Board::position_to_index(&Position::new(1,1))] = SlotState::Empty;
    board.slots[Board::position_to_index(&Position::new(1,2))] = SlotState::Occupied(Player::One);
    board.print();
    sleep(50);
    board.slots[Board::position_to_index(&Position::new(1,2))] = SlotState::Empty;
    board.slots[Board::position_to_index(&Position::new(1,3))] = SlotState::Occupied(Player::One);
    board.print();
    sleep(50);
    board.slots[Board::position_to_index(&Position::new(1,3))] = SlotState::Empty;
    board.slots[Board::position_to_index(&Position::new(1,4))] = SlotState::Occupied(Player::One);
    board.print();
    sleep(50);
    board.slots[Board::position_to_index(&Position::new(1,4))] = SlotState::Empty;
    board.slots[Board::position_to_index(&Position::new(1,5))] = SlotState::Occupied(Player::One);
    board.print();

}

// GRAPHICS "ENGINE"
pub fn sleep(sleep_time_in_ms: u64) {
    println!("");
    thread::sleep(time::Duration::from_millis(sleep_time_in_ms));
}