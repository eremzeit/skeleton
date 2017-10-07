use constants::*;
use types::Position;
use types::PiecePosition;
use super::generation::*;
use super::make_move::*;
use board::Board;

const QUEEN_TEST_FEN: &'static str = "7k/8/8/1p3P2/8/8/5Q1p/K7 w - - 0 1";

pub fn test_move_making() {
    let mut board = Board::from_fen("1k6/8/1K6/8/8/8/8/R7 b - - 0 1"); 

    // just make 3 random moves and try not to error
    let moves = generate_all_moves_for_color(&board, board.to_move);
    make_move(&mut board, moves[0]);
    println!("moves[0]: {:?}", moves[0]);
        
    let moves2 = generate_all_moves_for_color(&board, board.to_move);
    make_move(&mut board, moves2[0]);
        
    let moves3 = generate_all_moves_for_color(&board, board.to_move);
    make_move(&mut board, moves3[0]);
}
