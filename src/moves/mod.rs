mod tests;
mod types;
mod castling;
mod make_move;
mod generation;

use self::types::Move;
use types::*;
use util::*;
use constants::*;
use board::Board;
use std::iter;
use std::cmp;
use self::castling::*;
use self::make_move::*;
use self::generation as gen;

pub fn is_color_in_check(board: &Board, color: Color) -> bool {
    let piece = to_color(W_KING, color == WHITE);
    let piece_pos = board.get_first_piece(piece);

    assert!(piece_pos != None);
    match piece_pos {
        Some(some_piece_pos) =>  is_pos_attacked_by(board, some_piece_pos.to_position(), opposite_color(color)),
        None => false
    }
}

pub fn would_move_cause_check(board: &Board, mv: &Move) -> bool { 
    let mut new_board: Board = board.clone();

    //println!("board.to_move: {}", board.to_move);
    new_board.print_board();
    make_move(&mut new_board, mv);
    //println!("board.to_move: {}", board.to_move);
    new_board.print_board();

    let res = is_color_in_check(&new_board, board.to_move);
    println!("Would move {:?} be leave {} in check? {}", mv, color_to_string(board.to_move), res);
    res
}


//query piece P on board B where P
pub fn does_piece_attack(piece: PiecePosition, target_position: Position, board: &Board) -> bool {  
    // start casting your moves list as iterators and attempting to use them only from that
    // interface.  particularly, try to cancel out of the iterator early if you can.
    gen::get_piece_attacks(piece, board).find(|m: &Move| { 
        let is_attacking = m.dest_pos == Position(target_position.0, target_position.1);
        //println!("is {:?}, {:?} attacking {:?}? {}", m.origin_pos, m.dest_pos, Position(target_position.0, target_position.1), is_attacking);
        is_attacking
    }).is_some()
}

pub fn is_pos_attacked_by(board: &Board, pos: Position, color: Color) -> bool {
    
    // for each piece, does it attack this square
    let is_attacked: bool = board.get_pieces_iter().find(|piece_pos| {
        let does_attack = color_of(piece_pos.0) == color && does_piece_attack(*piece_pos, pos, board);
        //println!("does piece {:?} attack: {:?}? {}", piece_pos, pos, does_attack);
        does_attack
    }).is_some();
    
    //println!("is_pos_attacked_by: {:?}, {}, {}", pos, color_to_string(color), is_attacked);

    is_attacked
}

pub fn get_attackers(board: &Board, pos: Position) -> Vec<PiecePosition> {
    // logically:  query pieces p where get_attacks(p) has_member sq(e4)
    //
    // or iterate through all pieces, and filter
    // or check rays for potential attackers
     
    // diags
    // horiz, vert
    // pawns
    // kings
    // knights
    // TODO: optimize this.  for example, we could use a special move 
    // generation function that is designed for only checking attack status
    // as opposed to generating *all* moves.
    board.get_pieces_iter().filter(|p| {
        gen::get_piece_attacks(p.clone(), board).find(|m| m.dest_pos == p.to_position()).is_some()
    }).collect::<Vec<_>>()
}


