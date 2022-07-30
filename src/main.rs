
mod board;
use board::*;

mod network;
use network::server::GameSocket;

mod graphics;
use graphics::*;


fn main() {
    // init board
    let mut board = Board::new();

    // screen0: welcome
    print_new_screen(0);
    print_intro_message();

    // screen1: get user names
    print_new_screen(1);
    println!("🔴 (player 1)");
    println!("🟡 (player 2)\n");
    let player_1_name = prompt_for_name(1);

    print_new_screen(1);
    println!("🔴 {}", player_1_name);
    println!("🟡 (player 2)\n");
    let player_2_name = prompt_for_name(2);
    
    // screen2: wait for player 1 move
    let mut current_player = Player::One;
    let mut current_player_id = get_player_id(current_player);
    while !board.winner_exists_bool() {
        let player_move = wait_for_player_move(
            &board, 
            player_1_name.clone(), 
            player_2_name.clone(), 
            current_player_id
        );
        board.drop_piece(
            player_move as i8, 
            current_player
        );

        current_player = switch_player(current_player);
        current_player_id = get_player_id(current_player);
    }
}


fn switch_player(player: Player) -> Player {
    match player {
        Player::One => Player::Two,
        Player::Two => Player::One,
    }
}

fn get_player_id(player: Player) -> usize {
    match player {
        Player::One => 1,
        Player::Two => 2,
    }
}