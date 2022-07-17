mod board;

use board::*;

fn main() {
    println!("Connect 4!");
    let board = Board::random();
    let test_pos_0 = Position { col: 0, row: 0};
    let test_pos_1 = Position { col: 1, row: 0};
    let test_pos_2 = Position { col: 2, row: 0};
    println!("{:?}", board.get_slot_state(test_pos_0));
    println!("{:?}", board.get_slot_state(test_pos_1));
    println!("{:?}", board.get_slot_state(test_pos_2));
    board.print();
}




















