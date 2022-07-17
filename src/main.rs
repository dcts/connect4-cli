mod board;

use board::*;

fn main() {
    println!("Connect 4!");
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
}
