pub struct Operation {
    pub index: i32,
    pub is_flip: bool,
    pub is_open: bool,
    pub is_invalid: bool,
    pub exit_game: bool,
}

impl Operation {
    pub fn new() -> Operation {
        Operation {
            index: 0,
            is_flip: false,
            is_open: false,
            is_invalid: false,
            exit_game: false,
        }
    }
}
