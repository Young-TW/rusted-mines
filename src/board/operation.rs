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

        // 如果輸入為空
        if input.is_empty() {
            operation.is_invalid = true;
            return operation;
        }

        // 如果使用者輸入 exit 則退出遊戲
        if input == "exit" {
            operation.exit_game = true;
            return operation;
        }

        // 將輸入切割為兩部分
        let input: Vec<&str> = input.split_whitespace().collect();
        if input.len() != 2 {
            operation.is_invalid = true;
            return operation;
        }

        // 解析第一部分為整數，處理解析錯誤
        let index = match input[0].parse::<i32>() {
            Ok(i) => i,
            Err(_) => {
                operation.is_invalid = true;
                return operation;
            }
        };

        // 解析動作部分
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
