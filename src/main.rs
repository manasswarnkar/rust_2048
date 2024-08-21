use std::io;
use rand::Rng;
struct GameBoard {
    board: [[usize; 4]; 4],
}

impl GameBoard {
    fn new() -> GameBoard {
        return GameBoard { board: [[0; 4]; 4] };
    }

    fn display(&self) {
        println!("----------");
        for i in self.board.iter() {
            for j in i.iter() {
                let len = j.clone().to_string().len();
                let space = " ".repeat(6 - len);
                print!("{}{}", j, space);
            }
            println!();
        }
        println!("----------");
    }

    fn generate_new_tile(&mut self) -> Option<i32> {
        let mut empty: Vec<[usize; 2]> = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                if self.board[i][j] == 0 {
                    empty.push([i, j]);
                }
            }
        }

        if empty.is_empty() {
            if self.is_game_over() {
                return Some(-1);
            } else {
                return Some(0);
            }
        }

        let mut rng = rand::thread_rng();
        let n = empty.len();

        let rand_index = rng.gen::<usize>() % n;
        let row = empty[rand_index][0];
        let col = empty[rand_index][1];

        let new_tile = if (rng.gen::<u32>() % 10) < 9 { 2 } else { 4 };

        self.board[row][col] = new_tile;
        None
    }

    pub fn is_win(&self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if self.board[i][j] == 2048 {
                    return true;
                }
            }
        }

        return false;
    }

    // yet to implement
    pub fn merge_tiles(&mut self, input: &str) {
        if input == "w" {
            for i in 1..4 {
                for j in 0..4 {
                    if self.board[i][j] != 0 {
                        let mut x = i - 1;
                        while self.board[x][j] == 0 {
                            self.board[x][j] = self.board[x + 1][j];
                            self.board[x + 1][j] = 0;
                            if x != 0 {
                                x -= 1;
                            } else {
                                break;
                            }
                        }
                        
                        if self.board[x][j] == self.board[x + 1][j] {
                            self.board[x][j] *= 2;
                            self.board[x + 1][j] = 0;
                        }
                    }
                }
            }
        } else if input == "a" {
            for i in 0..4 {
                for j in 1..4 {
                    if self.board[i][j] != 0 {
                        let mut x = j - 1;
                        while self.board[i][x] == 0 {
                            self.board[i][x] = self.board[i][x+1];
                            self.board[i][x+1] = 0;
                            if x != 0 {
                                x -= 1;
                            } else {
                                break;
                            }
                        }
                        if self.board[i][x] == self.board[i][x+1] {
                            self.board[i][x] *= 2;
                            self.board[i][x+1] = 0;
                        }
                    }
                }
            }
        } else if input == "s" {
            for i in (0..=2).rev()  {
                for j in 0..4 {
                    if self.board[i][j] != 0 {
                        let mut x = i;
                        while self.board[x+1][j] == 0 {
                            self.board[x+1][j] = self.board[x][j];
                            self.board[x][j] = 0;
                            if x != 2 {
                                x += 1;
                            } else {
                                break;
                            }
                        }
                        if self.board[x+1][j] == self.board[x][j] {
                            self.board[x+1][j] *= 2;
                            self.board[x][j] = 0;
                        }
                    }
                }
            }
        } else if input == "d" {
            for i in 0..4 {
                for j in (0..=2).rev() {
                    if self.board[i][j] != 0 {
                        let mut x = j;
                        while self.board[i][x+1] == 0 {
                            self.board[i][x+1] = self.board[i][x];
                            self.board[i][x] = 0;
                            if x != 2 {
                                x += 1;
                            } else {
                                break;
                            }
                        }
                        if self.board[i][x+1] == self.board[i][x] {
                            self.board[i][x+1] *= 2;
                            self.board[i][x] = 0;
                        }
                    }
                }
            }
        }
    }
    
    fn is_game_over(&self) -> bool {
        for i in 1..3 {
            for j in 1..3 {
                if self.board[i][j] == self.board[i][j - 1]
                    || self.board[i][j] == self.board[i][j + 1]
                    || self.board[i][j] == self.board[i + 1][j]
                    || self.board[i][j] == self.board[i - 1][j]
                {
                    return false;
                }
            }
        }

        return true;
    }
}

fn main() {
    let mut b = GameBoard::new();
    b.generate_new_tile();
    b.generate_new_tile();
    loop {
        b.display();
        if b.is_win() {
            println!("You win!");
            break;
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to get input");
        let input = input.trim();
        b.merge_tiles(input);
        let a = b.generate_new_tile();
        if a != None && a.unwrap() == -1 {
            println!("You lose!");
            break;
        }
    }
}
