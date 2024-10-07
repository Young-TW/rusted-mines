pub struct Status {
    pub game_over: bool,
    pub game_won: bool,
    pub num_revealed: i32,
    pub num_flags: i32,
    pub num_safe_blocks: i32,
}

impl Status {
    pub fn new() -> Status {
        Status {
            game_over: false,
            game_won: false,
            num_revealed: 0,
            num_flags: 0,
            num_safe_blocks: 0,
        }
    }
}
