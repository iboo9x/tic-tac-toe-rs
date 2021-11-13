// Tic-tac-toe
use std::io::{self, Write};

const BASE_LEN: usize = 3;
fn main() {
    let mut board = Box::new([[' '; BASE_LEN]; BASE_LEN]);
    let mut player = 'X';
    let mut draw_result = true;

    println!("Tic-tac-toe - The Game");
    draw(&board);

    // Game starts
    for _ in 0..BASE_LEN * BASE_LEN {
        play(player, &mut board);
        draw(&board);
        if check(player, &board) {
            draw_result = false;
            break;
        }
        // Take turn
        player = match player {
            'X' => 'Y',
            'Y' => 'X',
            _ => panic!("unknown player"),
        };
    }

    // Show result
    if draw_result {
        println!("DRAW!");
    } else {
        println!("Player {} is the winner", player);
    }
}

fn draw(board: &Box<[[char; 3]; 3]>) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            print!("[{}]", board[i][j]);
        }
        println!();
    }
    println!();
}

fn play(player: char, board: &mut Box<[[char; BASE_LEN]; BASE_LEN]>) {
    println!("Player {}'s turn:", player);

    loop {
        let mut col_str = String::new();
        let mut row_str = String::new();

        // Get position
        print!("Enter row: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut row_str).unwrap();

        print!("Enter col: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut col_str).unwrap();

        let col: usize = col_str.trim().parse().unwrap();
        let row: usize = row_str.trim().parse().unwrap();

        // Verify and set
        if board[row][col] != ' ' {
            //panic!("duplicated spot");
            println!("Duplicated spot! Try again!");
        } else {
            board[row][col] = player;
            break;
        }
    }
}

fn check(player: char, board: &Box<[[char; BASE_LEN]; BASE_LEN]>) -> bool {
    let mut done;
    for i in 0..BASE_LEN {
        done = true;
        // Row check
        for j in 0..BASE_LEN {
            if board[i][j] != player {
                done = false;
                break;
            }
        }
        if done {
            return true;
        }
        done = true;
        // Column check
        for j in 0..BASE_LEN {
            if board[j][i] != player {
                done = false;
                break;
            }
        }
        if done {
            return true;
        }
    }
    // Cross check
    board[0][0] == player && board[1][1] == player && board[2][2] == player
        || board[0][2] == player && board[1][1] == player && board[2][0] == player
}
