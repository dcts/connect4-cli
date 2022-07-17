mod board;

use board::Board;

fn main() {
    println!("Connect 4!");
    let board = Board::random();
    board.print();
}




















