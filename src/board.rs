pub mod operation;
pub mod rule;
pub mod status;

use crate::block::Block;
use crate::board::operation::Operation;
use crate::board::status::Status;

use rand::Rng;

pub struct Board {
    pub blocks: Vec<Block>,
    pub width: i32,
    pub height: i32,
    pub num_mines: i32,
    pub num_blocks: i32,
    pub status: Status,
}

impl Board {
    pub fn new(width: i32, height: i32, num_mines: i32) -> Board {
        let mut blocks = Vec::new();
        let num_blocks = width * height;
        let mut index = 0;
        for _ in 0..num_blocks {
            blocks.push(Block::new());
            blocks[index as usize].index = index;
            index += 1;
        }

        Board {
            blocks,
            width,
            height,
            num_mines,
            num_blocks,
            status: Status::new(),
        }
    }

    pub fn init(&mut self) {
        self.status.num_safe_blocks = self.num_blocks - self.num_mines;
        // generate mines
        for _ in 0..self.num_mines {
            let mut rng = rand::thread_rng();
            let mut index = rng.gen_range(0..self.num_blocks);
            while self.blocks[index as usize].is_mine() {
                index = rng.gen_range(0..self.num_blocks);
            }
            self.blocks[index as usize].set_mine();
        }

        // calculate adjacent mines
    }

    pub fn print(&self) {
        // print board
        for i in 0..self.height {
            for j in 0..self.width {
                let index = i * self.width + j;
                if self.blocks[index as usize].is_revealed {
                    if self.blocks[index as usize].is_mine() {
                        print!("M ");
                    } else {
                        print!("{} ", self.blocks[index as usize].adjacent_mines);
                    }
                } else {
                    if self.blocks[index as usize].is_flagged {
                        print!("F ");
                    } else {
                        print!(". ");
                    }
                }
            }
            println!();
        }
    }

    pub fn play(&mut self) {
        let mut operate: Operation = Operation::new();
        while !self.status.game_over && !self.status.game_won && !operate.exit_game {
            // get user input
            operate = Operation::get();

            if operate.is_flip {
                self.blocks[operate.index as usize].flip_flag();
            } else if operate.is_open {
                self.blocks[operate.index as usize].reveal(&mut self.status.num_revealed);
            } else if operate.is_invalid {
                println!("Invalid operation");
            }

            // check game over
            if self.status.num_revealed == self.status.num_safe_blocks {
                self.status.game_won = true;
            }

            if self.status.game_over {
                // reveal all blocks
            }

            self.print();
        }

        if self.status.game_won {
            println!("You win!");
        } else {
            println!("You lose!");
        }
    }
}
