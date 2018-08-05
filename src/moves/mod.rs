pub mod tests;
pub mod types;
pub mod castling;
pub mod make_move;
pub mod unmake_move;
pub mod generation;

use regex::Regex;

use self::types::Move;
use types::{Position, PiecePosition};
use util::{
    opposite_color, 
    color_of,
    to_color,
    color_to_string
};
use constants::*;
use board::Board;
use std::iter;
use std::cmp;
use self::castling::*;
use self::make_move::*;
use self::generation as gen;


// TODO: inefficient
pub fn would_move_cause_check(board: &Board, mv: Move) -> bool { 
    let mut new_board: Board = board.clone();
    make_move(&mut new_board, mv);
    let res = is_color_in_check(&new_board, board.to_move);
    res
}

pub fn is_pos_attacked_by(board: &Board, pos: Position, color: Color) -> bool {
    // for each piece, does it attack this square
    let is_attacked: bool = board.get_pieces_iter().find(|piece_pos| {
        let does_attack = color_of(piece_pos.0) == color && does_piece_attack(*piece_pos, pos, board);
        //println!("does piece {:?} attack: {:?}? {}", piece_pos, pos, does_attack);
        does_attack
    }).is_some();
    
    is_attacked
}

pub fn is_color_in_check(board: &Board, color: Color) -> bool {
    let piece = to_color(W_KING, color == WHITE);
    let piece_pos = board.get_first_piece(piece);

    assert!(piece_pos != None);
    match piece_pos {
        Some(some_piece_pos) =>  {
            let is_checked = is_pos_attacked_by(board, some_piece_pos.to_position(), opposite_color(color));
            //println!("some_piece_pos: {:?}, is_checked={}", piece_pos, is_checked);
            is_checked
        },
        None => false
    }
}

pub fn does_match_moves(target: &str,  moves: &Vec<Move>) -> bool {
    let regex = Regex::new(target).unwrap();

    //println!("moves: {}", format!("moves and target: {:?}, {}", moves, target));
    regex.is_match(&format!("{:?}", moves))
}


//query piece P on board B where P
fn does_piece_attack(piece: PiecePosition, target_position: Position, board: &Board) -> bool {  
    // start casting your moves list as iterators and attempting to use them only from that
    // interface.  particularly, try to cancel out of the iterator early if you can.
    gen::get_piece_attacks(piece, board).find(|m: &Move| { 
        let is_attacking = m.dest_pos == Position(target_position.0, target_position.1);
        //println!("is {:?}, {:?} attacking {:?}? {}", m.origin_pos, m.dest_pos, Position(target_position.0, target_position.1), is_attacking);
        is_attacking
    }).is_some()
}

