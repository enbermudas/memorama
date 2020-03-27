use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::io::Write;

fn ask_for_wildcards() -> usize {
    let mut wildcards = String::new();

    print!("\nSelect the amount of wildcards [2..8]: ");

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

fn display_board(board: &Vec<char>, found_cards: &mut Vec<char>, selected_card: &usize) {
    let default_selected_card: usize = 0;
    println!("\n");

    for (i, _cell) in board.iter().enumerate() {
        if (i) % 4 == 0 && i != 0 {
            print!("\n\n");
        }

        let current_iteration_card: char = found_cards[i];
        if current_iteration_card == ' ' {
            match i + 1 > 9 {
                false => print!("  [0{}]  ", i + 1),
                true => print!("  [{}]  ", i + 1),
            }
        } else {
            print!("  [{}]  ", current_iteration_card)
        }
    }

    let len: usize = found_cards.iter().filter(|&&c| c == ' ').count();

    if selected_card != &default_selected_card
        && (found_cards[selected_card - 1] != ' ')
        && (found_cards[selected_card - 1] != found_cards[*selected_card])
        && len % 2 == 0
    {
        found_cards[selected_card - 1] = ' ';
        found_cards[*selected_card] = ' ';
    }

    print!("\n\n");
}

// fn check_selected_card(board: &mut Vec<char>, found_cards: &mut Vec<char>, &card_index: &usize) {}

fn start_game(board: Vec<char>) {
    let board = board;
    let len: usize = board.len();
    let default_selected_card: usize = 0;
    let mut found_cards: Vec<char> = vec![' '; len];

    display_board(&board, &mut found_cards, &default_selected_card);

    loop {
        let mut selected_card = String::new();

        print!("\n  Pick a card: ");

        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut selected_card)
            .expect("Error while reading the line");

        let selected_card: usize = match selected_card.trim().parse() {
            Ok(num) => match num {
                1..=16 => num - 1,
                _ => {
                    panic!("That card is not valid, try again");
                }
            },
            Err(_) => {
                panic!("That's not a valid number");
            }
        };

        if selected_card > len {
            panic!("\n  That card is not valid, try again");
        }

        found_cards[selected_card] = board[selected_card];

        display_board(&board, &mut found_cards, &selected_card);

        // check_selected_card(&mut board, &mut found_cards, &selected_card);
    }
}

fn main() {
    let wildcards: usize = ask_for_wildcards();
    let board = setup_board(wildcards);

    start_game(board);
}
