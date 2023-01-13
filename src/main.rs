use std::usize;

use rand::Rng;
struct Game {
    start: bool,
    die: bool,
    board: Board,
}
impl Game {
    fn new() -> Self {
        Self {
            start: (true),
            die: (false),
            board: Board::new(),
        }
    }
}
struct Board {
    matrix: [[Square; 4]; 4],
}
impl Board {
    fn new() -> Self {
        Board {
            matrix: [[Square::new(); 4]; 4],
        }
    }
    fn prepare_board(&mut self) {
        for i in 0..self.matrix.len() {
            for z in 0..self.matrix[i].len() {
                self.matrix[i][z].value = 0;
            }
        }

        let mut rng = rand::thread_rng();
        let mut i: i32 = rng.gen_range(0..4);
        let mut z: i32 = rng.gen_range(0..4);
    }

    fn is_board_full(&self) -> bool {
        for i in 0..self.matrix.len() {
            for z in 0..self.matrix[i].len() {
                if self.matrix[i][z].value == 0 {
                    return false;
                }
            }
        }
        true
    }
    fn is_square_avaible(&self, x: usize, y: usize) -> bool {
        if self.matrix[x][y].value == 0 {
            true
        } else {
            false
        }
    }
    fn get_rnd_avaible_square(&self) -> Option<[usize; 2]> {
        if self.is_board_full() {
            None
        } else {
            let mut rng = rand::thread_rng();
            let mut y: usize = rng.gen_range(0..4);
            let mut x: usize = rng.gen_range(0..4);
            while !self.is_square_avaible(x, y) {
                println!("{},{}", x, y);
                x = rng.gen_range(0..4);
                y = rng.gen_range(0..4);
            }
            let coordinates: [usize; 2] = [x, y];
            Some(coordinates)
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Square {
    value: i32,
}

impl Square {
    fn new() -> Self {
        Self { value: (0) }
    }
    fn decide_number(&mut self) {
        let mut rng = rand::thread_rng();
        let n: f32 = rng.gen();

        if n > 0.9 {
            self.set_four();
        } else {
            self.set_two();
        }
    }

    fn set_two(&mut self) {
        self.value = 2;
    }
    fn set_four(&mut self) {
        self.value = 4;
    }
}

fn main() {
    let mut game = Game::new();

    println!("{}", game.board.is_board_full());
    println!("{:?}", game.board.get_rnd_avaible_square());

    for i in 0..game.board.matrix.len() {
        for z in 0..game.board.matrix[i].len() {
            game.board.matrix[i][z].decide_number();

            print!("{:?}  ", game.board.matrix[i][z]);
            println!("{:?}", game.board.get_rnd_avaible_square());
        }
        println!()
    }
    println!("{}", game.board.is_board_full());

    println!("{:?}", game.board.get_rnd_avaible_square());
}
