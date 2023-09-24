/*

author: San_J (heissajay)
email: heissanjay@gmail.com

:multiverse:

*/

struct GameBoard {
    game_board: [[char; 3]; 3],
    is_game_over: bool,
}

impl GameBoard {
    fn print_board(&self) {
        println!();
        for row in self.game_board.iter() {
            for &cell in row.iter() {
                print!(" {} |", cell);
            }
            println!();
            println!("------------");
        }
        println!();
    }

    fn _move(&mut self, player: &mut Player, row: usize, col: usize) {
        if self.game_board[row][col] != ' ' {
            println!("** ILLEGAL MOVE **");
        } else {
            self.game_board[row][col] = player.avatar;
        }
    }

    fn has_player_won(&mut self, player: &Player) -> bool {
        // Check rows
        for row in &self.game_board {
            if row.iter().all(|&cell| cell == player.avatar) {
                self.game_over(player);
                return true;
            }
        }
        // Check columns
        for col in 0..3 {
            if (0..3).all(|row| self.game_board[row][col] == player.avatar) {
                self.game_over(player);
                return true;
            }
        }
        // Check diagonals
        if (0..3).all(|i| self.game_board[i][i] == player.avatar)
            || (0..3).all(|i| self.game_board[i][2 - i] == player.avatar)
        {
            self.game_over(player);
            return true;
        }
        false
    }

    fn game_over(&mut self, player: &Player) {
        println!("The Winner is Player {}", player.player_id);
        self.is_game_over = true;
    }
}

struct Player {
    player_id: i8,
    avatar: char,
}

fn init_game() -> (GameBoard, Player, Player) {
    let player1 = Player {
        player_id: 1,
        avatar: 'X',
    };

    let player2 = Player {
        player_id: 2,
        avatar: 'O',
    };

    let board = GameBoard {
        game_board: [[' '; 3]; 3],
        is_game_over: false,
    };

    println!("- Start Playing -");
    board.print_board();

    (board, player1, player2)
}

fn game_loop(mut board: GameBoard, mut player1: Player, mut player2: Player) {
    let mut current_player = &mut player1;
    let mut x_pos;
    let mut y_pos;
    let mut total_moves = 0; 

    while !board.is_game_over {
        let mut input = String::new();

        loop {
            println!("Player {}, enter the position you want to place (row, col): e.g. 0, 1", current_player.player_id);
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            input = input.trim().to_string();

            let parts: Vec<&str> = input.split(',').map(|s| s.trim()).collect();

            if parts.len() != 2 {
                println!("Invalid input format. Please use the format 'row,col'.");
                continue;
            }

            if let (Ok(x), Ok(y)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                if x < 3 && y < 3 && board.game_board[x][y] == ' ' {
                    x_pos = x;
                    y_pos = y;
                    break;
                } else {
                    println!("Invalid move. Please provide values between (0-2, 0-2) for an empty cell.");
                }
            } else {
                println!("Invalid input. Please enter numeric values.");
            }
            input.clear();
        }

        board._move(current_player, x_pos, y_pos);
        board.print_board();
        total_moves += 1; 

        if board.has_player_won(current_player) {
            break;
        }

        if total_moves == 9 {
            println!("It's a draw! The game is over.");
            break;
        }

        if current_player.player_id == 1 {
            current_player = &mut player2;
        } else {
            current_player = &mut player1;
        }
    }
}

fn main() {
    let (board, player1, player2) = init_game();
    game_loop(board, player1, player2);
}
