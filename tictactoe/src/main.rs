use std::io;

fn read_user_input() -> char {
    let mut user_input = String::new();

    let _ = io::stdin().read_line(&mut user_input);
    user_input.chars().nth(0).unwrap()
}

fn parse_cmd_to_pos(c: char) -> Option<(usize, usize)> {
    if c < '0'|| c > '8' {
        return None;
    }

    let c_digit : usize = c as usize - '0' as usize;
    Some((c_digit / 3, c_digit % 3))
}

fn is_input_correct(board: &[[char; 3]; 3], c: char) -> bool {
    parse_cmd_to_pos(c).map_or(false, |(x, y)| board[x][y] == '_')
}

fn is_game_draw(board: &[[char; 3]; 3]) -> bool {
    for row in *board {
        for c in row {
            if c == '_' {
                return false;
            }
        }
    }

    true
}

fn is_row_completed(board: &[[char; 3]; 3]) -> bool {
    for x in 0..3 {
        if board[x][0] == board[x][1] && board[x][1] == board[x][2] && board[x][0] == board[x][2] && board[x][0] != '_' {
            return true;
        }
    }
    false
}

fn is_col_completed(board: &[[char; 3]; 3]) -> bool {
    for y in 0..3 {
        if board[0][y] == board[1][y] && board[1][y] == board[2][y] && board[0][y] == board[2][y] && board[0][y] != '_' {
            return true;
        }
    }
    false
}

fn is_diag_completed(board: &[[char; 3]; 3]) -> bool {
    if board[0][0] != '_' && board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] == board[2][2] {
        return true;
    }

    if board[0][2] != '_' && board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] == board[2][0] {
        return true;
    }
    false
}

fn is_game_over(board: &[[char; 3]; 3]) -> bool {
    is_game_draw(board) || 
        is_row_completed(board) ||
        is_col_completed(board) ||
        is_diag_completed(board)
}

fn player_move(board: &mut[[char; 3]; 3], player: i32, cmd: char) {
    let (x, y) = parse_cmd_to_pos(cmd).unwrap();

    board[x][y] = if player == 0 { 'o' } else { 'x' };
}

fn print_board(board: &[[char; 3]; 3]) {
    println!("--------");

    for x in 0..3 {
        println!("{} {} {}", board[x][0], board[x][1], board[x][2]);
        println!("{} {} {}", 3 * x, 3 * x + 1, 3 * x + 2);
    }

    println!("--------");
}

fn main() {
    let mut board = [['_'; 3]; 3];
    let mut current_player = 0;
    print_board(&board);

    while !is_game_over(&board) {
        println!("move player {}", current_player);
        let mut cmd = read_user_input();
        while !is_input_correct(&board, cmd) {
            println!("again move player {}", current_player);
            cmd = read_user_input();
        }

        player_move(&mut board, current_player, cmd);
        print_board(&board);
        current_player = 1 - current_player;
    }
    println!("game over");
}
