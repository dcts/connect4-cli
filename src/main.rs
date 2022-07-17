// mod board;
mod network;
use network::server::GameSocket;

mod board;
use board::Board;

// fn game_callback(s: String) -> String {

// }
// sudo lsof -i -P | grep LISTEN | grep :3042
fn main() {
    // game logic demo
    println!("Connect 4!");
    let board = Board::random();
    board.print();

    // networking demo
    let connection_string = String::from("0.0.0.0:3042");
    // let socket = GameSocket::new(connection_string);
}




















