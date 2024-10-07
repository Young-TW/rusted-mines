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
            while self.blocks[index as usize].is_mine {
                index = rng.gen_range(0..self.num_blocks);
            }
            self.blocks[index as usize].is_mine = true;
        }

        // calculate adjacent mines
    }

    pub fn reveal_block(&mut self, index: i32) {
        if self.blocks[index as usize].is_revealed {
            return;
        }
        self.blocks[index as usize].is_revealed = true;
        self.status.num_revealed += 1;
        if self.blocks[index as usize].is_mine {
            self.status.game_over = true;
            return;
        }
        if self.blocks[index as usize].adjacent_mines == 0 {
            // self.reveal_adjacent_blocks(index);
        }
        if self.status.num_revealed == self.status.num_safe_blocks {
            self.status.game_won = true;
        }
    }

    pub fn flip_flag(&mut self, index: i32) {
        if self.blocks[index as usize].is_revealed {
            return;
        }
        if self.blocks[index as usize].is_flagged {
            self.blocks[index as usize].is_flagged = false;
            self.status.num_flags -= 1;
        } else {
            self.blocks[index as usize].is_flagged = true;
            self.status.num_flags += 1;
        }
    }

    pub fn print(&self) {
        // print board
        for i in 0..self.height {
            for j in 0..self.width {
                let index = i * self.width + j;
                if self.blocks[index as usize].is_revealed {
                    if self.blocks[index as usize].is_mine {
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
        while !self.status.game_over && !self.status.game_won {
            // get user input
            let mut operate: Operation = Operation::new();
            operate = Operation::get();

            // match user input
            match operate {
                ref is_flip => {
                    self.flip_flag(operate.index);
                }
                ref is_open => {
                    self.reveal_block(operate.index);
                }
                ref is_invalid => {
                    println!("Invalid operation");
                }
                ref exit_game => {
                    println!("Exit game");
                    break;
                }
            }

            // check game over
            if self.status.num_revealed == self.status.num_safe_blocks - self.num_mines {
                self.status.game_won = true;
            }

            if self.status.game_over {
                // reveal all blocks
            }

            self.print();
        }
    }
}
