mod board;
use board::*;

mod network;
use network::server::GameSocket;

fn main() {
    println!("Connect 4!");

    // game logic demo
    let mut win_path: Vec<Position> = Vec::new();
    let mut board = Board::random();
    while win_path.len() < 3 {
        board = Board::random();
        win_path = find_win(&board);
    }
    board.print();

    println!();
    for p in win_path {
        println!("row {} col {}", p.row, p.col)
    }
      
    // // BOARD DEMO OLD
    // let board = Board::random();
    // board.print();

    // // networking demo
    // let connection_string = String::from("0.0.0.0:3042");
    // let socket = GameSocket::new(connection_string);
}
