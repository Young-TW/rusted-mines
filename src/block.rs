pub struct block {
    pub index: i32,
    pub is_mine: bool,
    pub is_flagged: bool,
    pub is_revealed: bool,
    pub adjacent_mines: i32,
}

impl block {
    pub fn new() -> block {
        block {
            index: 0,
            is_mine: false,
            is_flagged: false,
            is_revealed: false,
            adjacent_mines: 0,
        }
    }
}
