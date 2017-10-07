use constants::{ File };
use moves::types::{Move};

#[derive(Clone)]
pub struct MoveContext {
    pub pending_move: Move,
    pub zhash: u64,
    pub castling: u8,
    pub en_passant: File,
    pub halfmove_counter: u8,
    pub fullmove_counter: u8,
}
