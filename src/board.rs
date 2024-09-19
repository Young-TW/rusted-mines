use crate::block::block;

pub struct board {
    pub blocks: Vec<block>,
    pub width: i32,
    pub height: i32,
    pub num_mines: i32,
    pub num_flags: i32,
    pub num_revealed: i32,
    pub num_safe_blocks: i32,
    pub num_blocks: i32,
    pub game_over: bool,
    pub game_won: bool,
}

impl board {
    pub fn new(width: i32, height: i32, num_mines: i32) -> board {
        let mut blocks = Vec::new();
        let num_blocks = width * height;
        let num_safe_blocks = num_blocks - num_mines;
        let mut index = 0;
        for _ in 0..num_blocks {
            blocks.push(block::new());
            blocks[index as usize].index = index;
            index += 1;
        }
        board {
            blocks: blocks,
            width: width,
            height: height,
            num_mines: num_mines,
            num_flags: 0,
            num_revealed: 0,
            num_safe_blocks: num_safe_blocks,
            num_blocks: num_blocks,
            game_over: false,
            game_won: false,
        }
    }

    pub fn reveal_block(&mut self, index: i32) {
        if self.blocks[index as usize].is_revealed {
            return;
        }
        self.blocks[index as usize].is_revealed = true;
        self.num_revealed += 1;
        if self.blocks[index as usize].is_mine {
            self.game_over = true;
            return;
        }
        if self.blocks[index as usize].adjacent_mines == 0 {
            // self.reveal_adjacent_blocks(index);
        }
        if self.num_revealed == self.num_safe_blocks {
            self.game_won = true;
        }
    }

    pub fn flip_flag(&mut self, index: i32) {
        if self.blocks[index as usize].is_revealed {
            return;
        }
        if self.blocks[index as usize].is_flagged {
            self.blocks[index as usize].is_flagged = false;
            self.num_flags -= 1;
        } else {
            self.blocks[index as usize].is_flagged = true;
            self.num_flags += 1;
        }
    }
}
