use types::*;
use util::*;
use constants::*;
use board::Board;
use std::fmt;
use std::collections::hash_map::RandomState;
use std::collections;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub origin_piece: PieceType,
    pub dest_piece: PieceType,
    pub origin_pos: Position,
    pub dest_pos: Position,
    pub meta_info: MetaInfo
}

impl Move {
    pub fn new() -> Self {
        Move {
            origin_piece: NO_PIECE,
            dest_piece: NO_PIECE,
            origin_pos: Position(0, 0),
            dest_pos: Position(0, 0),
            meta_info: QUIET_MOVE
        }
    }
    
    pub fn is_valid(&self) -> bool {
        let is_piece = self.origin_piece != NO_PIECE;
        let valid_capture = self.meta_info != CAPTURE || self.dest_piece != NO_PIECE;
        let valid_ep_capture = (
            self.meta_info != EP_CAPTURE
            || (is_white(self.origin_piece) && self.dest_pos.1 == WHITE_EP_CAP_RANK)
            || (!is_white(self.origin_piece) && self.dest_pos.1 == BLACK_EP_CAP_RANK)
        );

        let valid_castling = true;
        let valid_promotion = true;

        is_piece && valid_capture && valid_ep_capture && valid_castling && valid_promotion
    }

    pub fn color(&self) -> u8 {
       if self.origin_piece >= B_PAWN && self.origin_piece <= B_KING { BLACK } else { WHITE }
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        match self.meta_info {
            QUIET_MOVE =>  {
                //if self.dest_piece != NO_PIECE {
                //    println!("error!  {:?}, {:?}, {:?}, {:?}, {}", self.origin_piece, self.origin_pos, self.dest_piece, self.dest_pos, self.meta_info);
                //}

                //Na3-d3 
                write!(f, "{}{}{}-{}{}", 
                    piece_type_to_char(self.origin_piece), 
                    file_to_char(self.origin_pos.0), 
                    self.origin_pos.1 + 1,
                    
                    file_to_char(self.dest_pos.0), 
                    self.dest_pos.1 + 1,
                )    
            },

            CAPTURE => {
                write!(f, "{}{}{}*{}{}{}", 
                    piece_type_to_char(self.origin_piece), 
                    file_to_char(self.origin_pos.0), 
                    self.origin_pos.1 + 1,
                    
                    piece_type_to_char(self.dest_piece), 
                    file_to_char(self.dest_pos.0), 
                    self.dest_pos.1 + 1,
                ) 
            }, 

            EP_CAPTURE => {
                write!(f, "{}{}{}^{}{}", 
                    piece_type_to_char(self.origin_piece), 
                    file_to_char(self.origin_pos.0), 
                    self.origin_pos.1 + 1,
                    file_to_char(self.dest_pos.0), 
                    self.dest_pos.1 + 1,
                )    
            },

            KING_CASTLE => {
                write!(f, "O-O")    
            },
            
            QUEEN_CASTLE => {
                write!(f, "O-O-O")    
            },

            _ => {
                write!(f, "|{}{}{}-{}{}", 
                    piece_type_to_char(self.origin_piece), 
                    file_to_char(self.origin_pos.0), 
                    self.origin_pos.1 + 1,
                    
                    file_to_char(self.dest_pos.0), 
                    self.dest_pos.1 + 1,
                )    
            }
        }
    }
}

pub fn move_list_diff(positions1: &Vec<Move>, positions2: &Vec<Move>) -> Vec<Move> {
    let set1 = positions1.iter().map(|p| { *p }).collect::<collections::HashSet<Move, RandomState>>();
    let set2 = positions2.iter().map(|p| { *p }).collect::<collections::HashSet<Move, RandomState>>();

    //if positions1.iter().any(|m| !m.is_valid()) {
    //    println!("Move is invalid: {:?}", m);
    //    assert!(false);
    //}

    println!("set1: {:?}", set1);
    println!("set2: {:?}", set2);
    set1.symmetric_difference(&set2).map(|p| { *p }).collect::<Vec<Move>>()
}

pub type MetaInfo = u8;
pub type MoveList = Vec<Move>;

#[derive(Clone, PartialEq, Eq)]
pub struct AttackingRay {
    pub squares: Vec<Position>,
    pub attacker_index: u8
}

impl AttackingRay {
    pub fn new() -> Self {
        AttackingRay {
            squares: Vec::with_capacity(8),
            attacker_index: 0
        }
    }    
}

//TODO: is there a way to combine MovesIters so that that we can chain them?
pub struct MovesIter {
    pub moves: Vec<Move>,
    pub index: u8
}

impl MovesIter {
    pub fn from_vec(moves: Vec<Move>) -> Self {
        MovesIter {
            moves: moves,
            index: 0
        }
    }
}

impl Iterator for MovesIter {
    type Item = Move;
    
    fn next(&mut self) -> Option<Self::Item> {
        let r: Option<Self::Item>;

        if (self.index as usize) < self.moves.len() {
            r = Some(self.moves[self.index as usize]);
            self.index = self.index + 1;
        } else {
            r = None
        }
        
        r 
    }
}
