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
}
