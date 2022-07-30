use std::{io::{self, Write}};
use console::{Term, Key};
use crate::board::{Board, Player, COLS, ROWS, SlotState};

pub fn print_new_screen(newlines_after_title: usize) {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("=== CONNECT-4 ===");
    for _ in 0..newlines_after_title {
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

pub fn prompt_for_name(player_num: usize) -> String {
    print!("enter player {} name\n> ", player_num);

    io::stdout().flush().unwrap();
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    choice.trim().to_string()
}

pub fn print_board(board: &Board, pointer_position: i8) {
    match pointer_position {
        1 => println!(" 👇️"),
        2 => println!("    👇️"),
        3 => println!("       👇️"),
        4 => println!("          👇️"),
        5 => println!("             👇️"),
        6 => println!("                👇️"),
        7 => println!("                   👇️"),
        _ => println!(""),
    }
    board.slots.iter().enumerate().for_each(|(indx, slot)| {
        if indx == 0 {
            // opening pipe
            print!("|");
        }
        match slot {
            SlotState::Empty => print!("  |"),
            SlotState::Occupied(p) => match p {
                Player::One => print!("🔴|"),
                Player::Two => print!("🟡|"),
            },
        }
        if (indx + 1) % COLS == 0 && indx != COLS * ROWS - 1 {
            // breaks line after 7 items, must be omitted for the 42nd element
            print!("\n|");
        }
    });
    println!("");
}

pub fn wait_for_player_move(board: &Board, player_1_name: String, player_2_name: String, current_player_num: usize) -> usize {
    
    let print_board_state_player_1 = |current_selection: i8| {
        print_new_screen(1);
        println!("🔴 {} (⏳️ waiting for your move...)", player_1_name);
        println!("🟡 {}\n", player_2_name);
        print_board(&board, current_selection);
        println!("\nIt's your turn, {}! 👀 Please place your piece!", player_1_name);
        println!("\n🎮️ CONSTROLS:\n- move left (a)\n- move right (d)\n- drop piece (s)");
        println!("\nwaiting for piece drop...\n");
    };
    
    let print_board_state_player_2 = |current_selection: i8| {
        print_new_screen(1);
        println!("🔴 {}", player_1_name);
        println!("🟡 {} (⏳️ waiting for your move...)\n", player_2_name);
        print_board(&board, current_selection);
        println!("\nIt's your turn, {}! 👀 Please place your piece!", player_2_name);
        println!("\n🎮️ CONSTROLS:\n- move left (j)\n- move right (l)\n- drop piece (k)");
        println!("\nwaiting for piece drop...\n");
    };

    match current_player_num {
        1 => {
            let mut current_selection = 4;
            loop {
                print_board_state_player_1(current_selection);
                if let Ok(key) = Term::buffered_stdout().read_key() {
                    match key {
                        Key::Char(c) if c == 'a' => {
                            current_selection -= 1;
                            current_selection = std::cmp::max(std::cmp::min(7, current_selection), 1);
                        },
                        Key::Char(c) if c == 'd' => {
                            current_selection += 1;
                            current_selection = std::cmp::max(std::cmp::min(7, current_selection), 1);
                        },
                        Key::Char(c) if c == 's' => {
                            return current_selection as usize;
                        },
                        _ => println!("key: {:?}", key),
                    }
                }
            }
        },
        2 => {
            let mut current_selection = 4;
            loop {
                print_board_state_player_2(current_selection);
                if let Ok(key) = Term::buffered_stdout().read_key() {
                    match key {
                        Key::Char(c) if c == 'j' => {
                            current_selection -= 1;
                            current_selection = std::cmp::max(std::cmp::min(7, current_selection), 1);
                        },
                        Key::Char(c) if c == 'l' => {
                            current_selection += 1;
                            current_selection = std::cmp::max(std::cmp::min(7, current_selection), 1);
                        },
                        Key::Char(c) if c == 'k' => {
                            return current_selection as usize;
                        },
                        _ => println!("key: {:?}", key),
                    }
                }
            }
        }
        _=> panic!("INVALID current_player_num. Allowerd 1,2. Got: {}", current_player_num),
    }
}

