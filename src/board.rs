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
        for _ in 0..num_blocks {
            blocks.push(Block::new());
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

        // 隨機放置地雷
        let mut rng = rand::thread_rng();
        for _ in 0..self.num_mines {
            let mut index = rng.gen_range(0..self.num_blocks);
            while self.blocks[index as usize].is_mine() {
                index = rng.gen_range(0..self.num_blocks);
            }
            self.blocks[index as usize].set_mine();
        }

        // 計算每個格子的相鄰地雷數量
        for i in 0..self.height {
            for j in 0..self.width {
                let index = i * self.width + j;
                if !self.blocks[index as usize].is_mine {
                    let adjacent_mines = self.count_adjacent_mines(i, j);
                    self.blocks[index as usize].set_adjacent_mines(adjacent_mines);
                }
            }
        }
    }

    fn count_adjacent_mines(&self, row: i32, col: i32) -> i32 {
        let offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut count = 0;

        for (dy, dx) in &offsets {
            let new_row = row + dy;
            let new_col = col + dx;
            if new_row >= 0 && new_row < self.height && new_col >= 0 && new_col < self.width {
                let index = new_row * self.width + new_col;
                if self.blocks[index as usize].is_mine() {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn reveal_block(&mut self, index: i32) {
        if self.blocks[index as usize].is_revealed {
            return;
        }

        // 揭露當前格子
        self.blocks[index as usize].reveal(&mut self.status.num_revealed);

        // 如果是地雷，遊戲結束
        if self.blocks[index as usize].is_mine {
            self.status.game_over = true;
            return;
        }

        // 如果相鄰地雷數為 0，遞迴揭露相鄰格子
        if self.blocks[index as usize].adjacent_mines == 0 {
            let adjacent_offsets: [(i32, i32); 8] = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];

            let width = self.width;
            let height = self.height;

            for (dy, dx) in adjacent_offsets.iter() {
                let new_y = index / width + dy;
                let new_x = index % width + dx;
                let new_index = new_y * width + new_x;

                if new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
                    self.reveal_block(new_index); // 遞迴揭露
                }
            }
        }
    }

    pub fn print(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let index = i * self.width + j;
                if self.blocks[index as usize].is_revealed {
                    if self.blocks[index as usize].is_mine() {
                        print!("M ");
                    } else {
                        print!("{} ", self.blocks[index as usize].adjacent_mines);
                    }
                } else if self.blocks[index as usize].is_flagged {
                    print!("F ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }

    pub fn play(&mut self) {
        let mut operate: Operation = Operation::new();
        while !self.status.game_over && !self.status.game_won && !operate.exit_game {
            self.print();
            operate = Operation::get_cli_input();

            if operate.is_flip {
                self.blocks[operate.index as usize].flip_flag();
            } else if operate.is_open {
                self.reveal_block(operate.index);
            } else if operate.is_invalid {
                println!("Invalid operation");
            }

            if self.status.num_revealed == self.status.num_safe_blocks {
                self.status.game_won = true;
            }

            if self.status.game_over {
                println!("Game Over!");
            }

            if self.status.game_won {
                println!("You Win!");
            }
        }
    }
}
