use std::io;

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


fn get_player_input(board: &Board, current_player: &Player, available_pieces: &[Piece]) -> (Piece, Position) {
    loop {
        println!("Enter the index of the piece you want to play (0-based):");
        let piece_index: usize = read_input().trim().parse().expect("Please enter a valid number");

        if piece_index >= available_pieces.len() {
            println!("Invalid piece index, please try again.");
            continue;
        }

        let selected_piece = available_pieces[piece_index].clone();

        println!("Enter the row and column where you want to place the piece (separated by a space):");
        let position_input: Vec<usize> = read_input()
            .split_whitespace()
            .map(|n| n.parse().expect("Please enter valid numbers"))
            .collect();

        if position_input.len() != 2 {
            println!("Invalid input, please enter row and column separated by a space.");
            continue;
        }

        let position = (position_input[0], position_input[1]);

        if is_valid_move(board, &selected_piece, &position, current_player) {
            return (selected_piece, position);
        } else {
            println!("Invalid move, please try again.");
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn is_valid_move(board: &Board, piece: &Piece, position: &Position, player: &Player) -> bool {
    // Additional checks for valid placement according to Blokus rules should be implemented here.
    can_place_piece(board, piece, position, player)
}

fn can_place_piece(board: &Board, piece: &Piece, position: &Position, player: &Player) -> bool {
    for (row_offset, piece_row) in piece.iter().enumerate() {
        let board_row = position.0 + row_offset;
        if board_row >= board.len() {
            return false;
        }

        for (col_offset, &piece_cell) in piece_row.iter().enumerate() {
            let board_col = position.1 + col_offset;
            if board_col >= board[0].len() {
                return false;
            }

            if piece_cell && board[board_row][board_col].is_some() {
                return false;
            }
        }
    }

    // Check if the piece touches another piece of the same color only at the corners
    let adjacents = [
        (0, 1),
        (1, 0),
        (0, isize::MAX),
        (isize::MAX, 0),
    ];
    let diagonals = [
        (isize::MAX, isize::MAX),
        (1, isize::MAX),
        (isize::MAX, 1),
        (1, 1),
    ];

    let mut diagonal_touch = false;
    for (row_offset, piece_row) in piece.iter().enumerate() {
        let board_row = position.0 + row_offset;
        for (col_offset, &piece_cell) in piece_row.iter().enumerate() {
            if !piece_cell {
                continue;
            }

            let board_col = position.1 + col_offset;

            for &(dr, dc) in adjacents.iter() {
                let neighbor_row = (board_row as isize).wrapping_add(dr) as usize;
                let neighbor_col = (board_col as isize).wrapping_add(dc) as usize;

                if neighbor_row < board.len() && neighbor_col < board[0].len() && board[neighbor_row][neighbor_col] == Some(*player) {
                    return false;
                }
            }

            for &(dr, dc) in diagonals.iter() {
                let neighbor_row = (board_row as isize).wrapping_add(dr) as usize;
                let neighbor_col = (board_col as isize).wrapping_add(dc) as usize;

                if neighbor_row < board.len() && neighbor_col < board[0].len() && board[neighbor_row][neighbor_col] == Some(*player) {
                    diagonal_touch = true;
                }
            }
        }
    }

    diagonal_touch
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

        if is_valid_move(&board, &selected_piece, &position, &current_player) {
            make_move(&mut board, &mut players, &selected_piece, &position, &current_player);

            if check_game_over(&players) {
                declare_winner(&players);
                break;
            }
            current_player = switch_player(current_player);
        } else {
            println!("Invalid move, please try again.");
        }
    }
}

