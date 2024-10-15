pub struct Block {
    pub index: i32,
    pub is_mine: bool,
    pub is_flagged: bool,
    pub is_revealed: bool,
    pub adjacent_mines: i32,
}

impl Block {
    pub fn new() -> Block {
        Block {
            index: 0,
            is_mine: false,
            is_flagged: false,
            is_revealed: false,
            adjacent_mines: 0,
        }
    }

    pub fn reveal(&mut self, n_revealed: &mut i32) -> bool {
        // the return boolean means if the block is a mine
        if self.is_revealed == false {
            self.is_revealed = true;
            *n_revealed += 1;
        }

        self.is_mine
    }

    pub fn flip_flag(&mut self) {
        self.is_flagged = !self.is_flagged;
    }

    pub fn set_mine(&mut self) {
        self.is_mine = true;
    }

    pub fn set_adjacent_mines(&mut self, num: i32) {
        self.adjacent_mines = num;
    }

    pub fn is_mine(&self) -> bool {
        self.is_mine
    }
}
