use std::io::{self, Write, stdout};


use console::Term;
use console::Key;

use crate::board::{Board, Player};

pub fn print_new_screen(newlines_after_title: usize) {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("=== CONNECT-4 ===");
    for i in 0..newlines_after_title {
        println!("");
    }
}

pub fn print_intro_message() {
    println!("\nWelcome to connect4 CLI. You can play a casual");
    print!("game of connect4 in the terminal.\n\nPress any key to START...\n> ");

    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    println!("starting game...");
}

pub fn print_waiting_for_move(player_num: usize) {
    println!("waiti");
}

pub fn prompt_for_name(player_num: usize) -> String {
    print!("enter player {} name\n> ", player_num);

    io::stdout().flush().unwrap();
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    choice.trim().to_string()
}

pub fn wait_for_player_move(board: &Board, player: Player) -> usize {
    let mut current_selection = 1;
    loop {
        print_new_screen(0);
        if let Ok(key) = Term::buffered_stdout().read_key() {
            match key {
                Key::ArrowLeft => {
                    current_selection -= 1;
                    current_selection = std::cmp::max(std::cmp::min(7, current_selection), 0);
                },
                Key::ArrowRight => {
                    current_selection += 1;
                    current_selection = std::cmp::max(std::cmp::min(7, current_selection), 0);
                },
                _ => println!("key: {:?}", key),
            }
            board.print(current_selection);
        }
    }
}
