mod board;
use board::*;

mod network;
use network::server::GameSocket;

fn main() {
    println!("Connect 4!");

    // game logic demo
    let board = Board::random();
    board.print();
    println!("");
    
    let maybe_win_info = find_win(&board);
    match maybe_win_info {
        Some(win_info) => println!("Win Info: {:?}", win_info),
        None => println!("no win found!"),
    }
      
    // // BOARD DEMO OLD
    // let board = Board::random();
    // board.print();

    // // networking demo
    // let connection_string = String::from("0.0.0.0:3042");
    // let socket = GameSocket::new(connection_string);
}
