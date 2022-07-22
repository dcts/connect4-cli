mod board;
use board::*;

mod network;
use network::server::GameSocket;

fn main() {
    println!("Connect 4!");

    // game logic demo
    let board = Board::random();
    board.print();
    let maybe_win_info = board.winner_exists();
    
    match maybe_win_info {
        Some(win_info) => {
            println!("\n\n\nWin Info: \n{:?}\n", win_info);
            // show isolated sequence in a new board
            let mut win_board = Board::new();
            win_info.win_path.iter().for_each(|pos| {
                let indx = Board::position_to_index(pos);
                win_board.slots[indx] = SlotState::Occupied(win_info.winner);
            });
            win_board.print();
        },
        None => println!("\n\n\nno win found!"),
    }
      
    // // BOARD DEMO OLD
    // let board = Board::random();
    // board.print();

    // // networking demo
    // let connection_string = String::from("0.0.0.0:3042");
    // let socket = GameSocket::new(connection_string);
}
