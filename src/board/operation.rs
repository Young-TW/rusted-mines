pub struct Operation {
    pub index: i32,
    pub is_flip: bool,
    pub is_open: bool,
}

impl Operation {
    pub fn new() -> Operation {
        Operation {
            index: 0,
            is_flip: false,
            is_open: false,
        }
    }
}
