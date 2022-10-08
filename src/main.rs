use chess_game::Board;
use chess_game::{Rook, Bishop, Knight, King, Queen, Pawn};

fn main() {
    let mut board = Board::new();
    board.init_board();

    println!("{}", board);
}
