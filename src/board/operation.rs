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

    pub fn get() -> Operation {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let mut operation = Operation::new();
        if input.len() == 0 {
            operation.is_invalid = true;
            return operation;
        }
        if input == "exit" {
            operation.exit_game = true;
            return operation;
        }
        let input: Vec<&str> = input.split_whitespace().collect();
        if input.len() != 2 {
            operation.is_invalid = true;
            return operation;
        }
        let index = input[0].parse::<i32>().unwrap();
        let action = input[1];
        if action == "f" {
            operation.is_flip = true;
        } else if action == "o" {
            operation.is_open = true;
        } else {
            operation.is_invalid = true;
        }
        operation.index = index;
        operation
    }
}
