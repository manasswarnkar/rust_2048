use rand::Rng;
struct GameBoard {
     board : [[usize;4]; 4],
}

impl GameBoard {
    fn new() -> GameBoard{
        return GameBoard { board : [[0; 4]; 4]};
    }

    fn display(&self) {
        for i in self.board.iter() {
            for j in i.iter() {
                print!("{} ", j);
            }
            println!();
        }
        println!("----------");
    }

    fn generate_new_tile(&mut self) {
        let mut empty:Vec<[usize;2]> = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                if self.board[i][j] == 0 {
                    empty.push([i, j]);
                } 
            }
        }
        
        if empty.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        let n = empty.len();

        let rand_index = rng.gen::<usize>() % n;
        let row = empty[rand_index][0];
        let col = empty[rand_index][1];

        let new_tile = if (rng.gen::<u32>() % 10) < 9 {2} else {4};

        self.board[row][col] = new_tile;

    }

    fn is_win(&self) -> bool {
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
    fn _merge_tiles() {
    }
    fn _handle_input() {
    }
    fn _is_game_over(&self) -> bool {
        return false;
    }

}

fn main() {
    let mut b = GameBoard::new();  
    loop {
        b.display();
        if b.is_win() { 
            println!("You win!");
            break; 
        }
        b.generate_new_tile();
        b.board[0][0] = 2048;
    }
}