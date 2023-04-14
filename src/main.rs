use std::io;
use std::cmp;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Player {
    Player1,
    Player2,
}

type Board = Vec<Vec<Option<Player>>>;
type Piece = Vec<Vec<bool>>;
type Position = (usize, usize);

fn initialize_board() -> Board {
    vec![vec![None; 14]; 14]
}

fn initialize_players() -> Vec<Vec<Piece>> {
    // Define and initialize the players' pieces here.
    // For simplicity, this example uses only two sample pieces per player.
    let piece1 = vec![
        vec![true, true],
        vec![true, false],
    ];

    let piece2 = vec![
        vec![true, true, true],
    ];

    vec![vec![piece1.clone(), piece2.clone()], vec![piece1, piece2]]
}

fn print_board(board: &Board) {
    for row in board {
        for cell in row {
            match cell {
                Some(Player::Player1) => print!("1"),
                Some(Player::Player2) => print!("2"),
                None => print!("."),
            }
        }
        println!();
    }
}

fn get_available_pieces<'a>(players: &'a [Vec<Piece>], current_player: &Player) -> &'a [Piece] {
    match current_player {
        Player::Player1 => &players[0],
        Player::Player2 => &players[1],
    }
}

fn get_input_from_user(piece_count: usize) -> (usize, Position) {
    loop {
        println!("Enter the piece number and position (row, col):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input_values: Vec<usize> = input
            .split_whitespace()
            .filter_map(|word| word.parse().ok())
            .collect();

        if input_values.len() == 3 {
            let piece_index = input_values[0];
            let row = input_values[1];
            let col = input_values[2];

            if piece_index < piece_count {
                return (piece_index, (row, col));
            }
        }

        println!("Invalid input, please try again.");
    }
}

fn get_player_input(board: &Board, current_player: &Player, available_pieces: &[Piece]) -> (Piece, Position) {
    loop {
        let (piece_index, position) = get_input_from_user(available_pieces.len());

        let selected_piece = &available_pieces[piece_index];
        let validation_result = is_valid_move(board, &selected_piece, &position, current_player);

        match validation_result {
            Ok(_) => return (selected_piece.clone(), position),
            Err(reason) => println!("Invalid move: {}", reason),
        }
    }
}

fn is_first_move(board: &Board, player: &Player) -> bool {
    board.iter().flatten().all(|cell| cell.is_none() || cell.as_ref().unwrap() != player)
}

fn is_corner_move(board: &Board, piece: &Piece, position: &Position, player: &Player) -> bool {
    let corner_positions = match player {
        Player::Player1 => vec![(0, 0)],
        Player::Player2 => vec![(board.len() - 1, board[0].len() - 1)],
    };

    for &(corner_row, corner_col) in corner_positions.iter() {
        for (row_offset, piece_row) in piece.iter().enumerate() {
            for (col_offset, &piece_cell) in piece_row.iter().enumerate() {
                if piece_cell && (position.0 + row_offset) == corner_row && (position.1 + col_offset) == corner_col {
                    return true;
                }
            }
        }
    }

    false
}

fn check_touching_corner(board: &Board, row: usize, col: usize, player: &Player) -> bool {
    let row = row as isize;
    let col = col as isize;
    let offsets = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for (row_offset, col_offset) in offsets.iter() {
        let new_row = row + row_offset;
        let new_col = col + col_offset;

        if new_row >= 0 && new_row < board.len() as isize && new_col >= 0 && new_col < board[0].len() as isize {
            if board[new_row as usize][new_col as usize] == Some(*player) {
                return true;
            }
        }
    }

    false
}

fn check_touching_side(board: &Board, row: usize, col: usize, player: &Player) -> bool {
    let row = row as isize;
    let col = col as isize;
    let offsets = [(-1, 0), (0, -1), (0, 1), (1, 0)];

    for (row_offset, col_offset) in offsets.iter() {
        let new_row = row + row_offset;
        let new_col = col + col_offset;

        if new_row >= 0 && new_row < board.len() as isize && new_col >= 0 && new_col < board[0].len() as isize {
            if board[new_row as usize][new_col as usize] == Some(*player) {
                return true;
            }
        }
    }

    false
}

fn can_place_piece(board: &Board, piece: &Piece, position: &Position, player: &Player) -> bool {
    let mut min_row = board.len();
    let mut min_col = board[0].len();
    let mut touching_corner = false;
    let mut touching_side = false;

    for (row_offset, piece_row) in piece.iter().enumerate() {
        for (col_offset, &piece_cell) in piece_row.iter().enumerate() {
            if piece_cell {
                let row = position.0 + row_offset;
                let col = position.1 + col_offset;

                if row >= board.len() || col >= board[0].len() || board[row][col].is_some() {
                    return false;
                }

                touching_corner |= check_touching_corner(board, row, col, player);
                touching_side |= check_touching_side(board, row, col, player);

                min_row = cmp::min(min_row, row);
                min_col = cmp::min(min_col, col);
            }
        }
    }

    if min_row > 0 && min_col > 0 && !touching_corner {
        return false;
    }

    if touching_side {
        return false;
    }

    true
}


fn make_move(board: &mut Board, players: &mut Vec<Vec<Piece>>, selected_piece: &Piece, position: &Position, current_player: &Player) {
    for (row_offset, piece_row) in selected_piece.iter().enumerate() {
        for (col_offset, &piece_cell) in piece_row.iter().enumerate() {
            if piece_cell {
                board[position.0 + row_offset][position.1 + col_offset] = Some(*current_player);
            }
        }
    }

    let player_index = match current_player {
        Player::Player1 => 0,
        Player::Player2 => 1,
    };

    players[player_index].retain(|piece| piece != selected_piece);
}

fn switch_player(current_player: Player) -> Player {
    match current_player {
        Player::Player1 => Player::Player2,
        Player::Player2 => Player::Player1,
    }
}

fn check_game_over(players: &[Vec<Piece>]) -> bool {
    players.iter().all(|player_pieces| player_pieces.is_empty())
}

fn declare_winner(players: &[Vec<Piece>]) {
    let player1_remaining_pieces = players[0].len();
    let player2_remaining_pieces = players[1].len();

    if player1_remaining_pieces < player2_remaining_pieces {
        println!("Player 1 wins!");
    } else if player1_remaining_pieces > player2_remaining_pieces {
        println!("Player 2 wins!");
    } else {
        println!("It's a draw!");
    }
}

fn is_valid_move(board: &Board, piece: &Piece, position: &Position, player: &Player) -> Result<(), &'static str> {
    if is_first_move(board, player) {
        if !is_corner_move(board, piece, position, player) {
            return Err("First move must be placed in the corner.");
        }
    } else if !can_place_piece(board, piece, position, player) {
        return Err("Invalid move: Cannot place piece at the given position.");
    }

    Ok(())
}

fn main() {
    let mut board = initialize_board();
    let mut players = initialize_players();
    let mut current_player = Player::Player1;

    loop {
        print_board(&board);
        let available_pieces = get_available_pieces(&players, &current_player);
        println!("Available pieces for {:?}:", current_player);
        for (index, piece) in available_pieces.iter().enumerate() {
            println!("{}: {:?}", index, piece);
        }

        let (selected_piece, position) = get_player_input(&board, &current_player, &available_pieces);

        match is_valid_move(&board, &selected_piece, &position, &current_player) {
            Ok(_) => {
                make_move(&mut board, &mut players, &selected_piece, &position, &current_player);

                if check_game_over(&players) {
                    declare_winner(&players);
                    break;
                }
                current_player = switch_player(current_player);
            }
            Err(reason) => println!("Invalid move: {}", reason),
        }
    }
}
