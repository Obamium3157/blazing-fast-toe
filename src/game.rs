pub mod input {
    use std::io;

    pub struct Input { }

    impl Input {
        pub fn input_usize() -> usize {
            loop {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                let input: usize = match input.trim().parse() {
                    Ok(a) => a,
                    Err(_) => continue,
                };
                return input;
            }
        }

        pub fn input_string() -> String {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input = String::from(input.trim());

            input
        }
    }
}

pub struct Game {
    board: [[usize ; 3] ; 3],
    player: i8,
    is_in_game: bool,
    cause_of_end: i8,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [
                [ 0, 0, 0 ],
                [ 0, 0, 0 ],
                [ 0, 0, 0 ],
            ],
            player: 1, // 1 -> FIRST, -1 -> SECOND
            is_in_game: true,
            cause_of_end: 0,
        }
    }

    pub fn paint(&self, col: usize, row: usize) {
        console::Term::stdout().clear_screen().expect("Failed to clear console");

        if col == 5 && row == 5 {
            for _c in 0..3 {
                for _r in 0..3 {
                    print!("[ ] ");
                }
                println!();
            }
        } else {
            for c in 0..3 {
                for r in 0..3 {
                    if self.board[c][r] != 0 {
                        if self.board[c][r] == 1{
                            print!("[X] ");
                        } else {
                            print!("[O] ");
                        }
                    } else {
                        print!("[ ] ");
                    }
                }
                println!();
            }
        }
    }

    pub fn turn(&mut self, col: usize, row: usize) {
        if (1 <= col && col <= 3) && (1 <= row && row <= 3) && (self.board[col - 1][row - 1] == 0) {
            if self.player == 1 {
                self.board[col - 1][row - 1] = 1;
            } else {
                self.board[col - 1][row - 1] = 2;
            }
            self.paint(col, row);
            self.check_in_game();

            if self.is_in_game {
                if self.player == 1 {
                    self.player += 1;
                } else {
                    self.player -= 1;
                }
            }
        }
    }

    fn check_in_game(&mut self) {
        self.is_in_game = !(
            (self.board[0][0] == self.board[1][0] && self.board[0][0] == self.board[2][0] && self.board[0][0] != 0)
                || (self.board[0][1] == self.board[1][1] && self.board[0][1] == self.board[2][1] && self.board[0][1] != 0)
                || (self.board[0][2] == self.board[1][2] && self.board[0][2] == self.board[2][2] && self.board[0][2] != 0)

                || (self.board[0][0] == self.board[0][1] && self.board[0][0] == self.board[0][2] && self.board[0][0] != 0)
                || (self.board[1][0] == self.board[1][1] && self.board[1][0] == self.board[1][2] && self.board[1][0] != 0)
                || (self.board[2][0] == self.board[2][1] && self.board[2][0] == self.board[2][2] && self.board[2][0] != 0)

                || (self.board[0][0] == self.board[1][1] && self.board[0][0] == self.board[2][2] && self.board[0][0] != 0)
                || (self.board[0][2] == self.board[1][1] && self.board[0][2] == self.board[2][0] && self.board[0][2] != 0)
            );

        if
            self.board[0][0] != 0 && self.board[0][1] != 0 && self.board[0][2] != 0
                && self.board[1][0] != 0 && self.board[1][1] != 0 && self.board[1][2] != 0
                && self.board[2][0] != 0 && self.board[2][1] != 0 && self.board[2][2] != 0
         {
            self.is_in_game = false;
            self.cause_of_end = 1;
        }
    }

    pub fn get_in_game(&self) -> bool {
        self.is_in_game
    }
    pub fn get_cause_of_end(&self) -> i8 {
        self.cause_of_end
    }
    pub fn get_player(&self) -> i8 {
        self.player
    }
}