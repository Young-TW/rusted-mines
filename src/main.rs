mod block;
mod board;

fn main() {
    let mut game_board = board::Board::new(10, 10, 10);
    loop {
        game_board.init();
        game_board.play();
    }
}
