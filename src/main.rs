use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::io::Write;

fn ask_for_wildcards() -> usize {
    let mut wildcards = String::new();

    print!("Select the amount of wildcards [2..8]: ");

    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut wildcards)
        .expect("Error while reading the line");

    let wildcards: usize = match wildcards.trim().parse() {
        Ok(num) => match num {
            2..=8 => num,
            _ => {
                panic!("The amount of wildcards must be [2..8]");
            }
        },
        Err(_) => {
            panic!("That's not a valid number");
        }
    };

    wildcards
}

fn setup_board(wildcards: usize) -> Vec<char> {
    let emojis = vec!['😸', '🐶', '🐵', '🐻', '🐼', '🐰', '🐭', '🐯'];

    let mut board: Vec<_> = emojis[0..wildcards]
        .iter()
        .cycle()
        .take(emojis[0..wildcards].len() * 2)
        .collect();

    board.shuffle(&mut thread_rng());

    let mut shuffled_board: Vec<char> = vec![];

    for c in board {
        shuffled_board.push(*c);
    }

    shuffled_board
}

fn display_board(board: &Vec<char>) {
    println!("");

    for (i, _cell) in board.iter().enumerate() {
        if (i) % 4 == 0 && i != 0 {
            print!("\n\n");
        }

        match i + 1 > 9 {
            false => print!("  [0{}]  ", i + 1),
            true => print!("  [{}]  ", i + 1),
        }
    }

    print!("\n\n");
}

fn start_game(board: Vec<char>) {
    display_board(&board);
    // Game logic here...
}

fn main() {
    let wildcards: usize = ask_for_wildcards();
    let board = setup_board(wildcards);

    start_game(board);
}
