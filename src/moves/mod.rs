mod tests;

use types::*;
use util::*;
use constants::*;
use board::Board;
use std::fmt;
use std::iter;
use std::cmp;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    origin_piece: PieceType,
    dest_piece: PieceType,
    origin_pos: Position,
    dest_pos: Position,
    meta_info: MetaInfo
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

            _ => {
                write!(f, "{}{}{}-{}{}", 
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

pub type MetaInfo = u8;

pub type MoveList = Vec<Move>;

#[derive(Clone, PartialEq, Eq)]
pub struct AttackingRay {
    pub squares: Vec<Position>,
    pub attacker_index: u8
}

impl AttackingRay {
    fn new() -> Self {
        AttackingRay {
            squares: Vec::with_capacity(8),
            attacker_index: 0
        }
    }    
}




// maybe in the future we could convert all these generator 
// functions to return boxed iterators.
//let x: Box<Iterator<Item=Move>> = Box::new(moves.into_iter());

//pub fn generate_moves(board: Board) -> MoveList {
//    generate_bishop_moves(board)
//        .chain(generate_rook_moves())
//        .chain(generate_knight_moves())
//        .chain(generate_pawn_moves())
//        .chain(generate_queen_moves())
//        .chain(generate_king_moves());
//}

pub fn diag_squares(pos: Position) -> AttackingRay { 
    let mut ray = AttackingRay::new(); 
    
    // [ -7, +7 ]
    let diag_series_index: i8 = pos.1 as i8 - pos.0 as i8;
    let diag_length: i8 = RANK_COUNT as i8 - diag_series_index.abs() as i8;
    let diag_piece_ind: i8 = (if diag_series_index > 0 { pos.0 } else { pos.1 }) as i8;

    ray.attacker_index = diag_piece_ind as u8;
 
    for i in 0..diag_length {
        let f: i8 = (pos.0 as i8) + (i - diag_piece_ind);
        let r: i8 = (pos.1 as i8) + (i - diag_piece_ind);
        
        let pos = Position(f as File, r as Rank);
        ray.squares.push(pos);
    }
     
    ray
}

pub fn anti_diag_squares(pos: Position) -> AttackingRay {
    let mut ray = AttackingRay::new(); 

    // [ -7, +7 ]
    let diag_series_index: i8 = pos.1 as i8 - (FILE_COUNT as i8 - pos.0 as i8 - 1);
    let diag_length: i8 = (RANK_COUNT as i8 - diag_series_index.abs() as i8) as i8;
    let diag_piece_ind: i8 = (if diag_series_index > 0 { FILE_COUNT - pos.0 - 1} else { pos.1 }) as i8;
    ray.attacker_index = diag_piece_ind as u8;
    
    for i in 0..diag_length {
        let f: i8 = (pos.0 as i8) - (i - diag_piece_ind);
        let r: i8 = (pos.1 as i8) + (i - diag_piece_ind);

        let pos = Position(f as File, r as Rank);
        
        ray.squares.push(pos);
    }
    
    ray
}

pub fn horizontal_squares(pos: Position) -> AttackingRay {
    let mut res = AttackingRay::new();
    
    for f in 0..FILE_COUNT {
        res.squares.push(Position(f, pos.1)); 

        if pos.0 == f {
            res.attacker_index = f         
        }
    }

    res
}

pub fn vertical_squares(pos: Position) -> AttackingRay {
    let mut res = AttackingRay::new();
    
    for r in 0..RANK_COUNT {
        res.squares.push(Position(pos.0, r)); 
        if pos.1 == r {
            res.attacker_index = r;
        }
    }

    res
}
    
fn moves_from_ray(piece: PiecePosition, squares: &[Position], board: &Board) -> Vec<Move> {
    let mut moves: MoveList = Vec::with_capacity(20);

    for dest_pos in squares.iter() {
        let dest_piece = board.mb.get(dest_pos.0, dest_pos.1);

        if &piece.to_position() == dest_pos {
            //do nothing.
            //TODO: modify the generate bishop moves function to not return the square with the
            //attacker itself
        } else if dest_piece == NO_PIECE {
            println!("empty: {:?}", dest_pos);
            let mut mv = Move::new();
            mv.origin_piece = piece.0;
            mv.dest_piece = dest_piece;                
            mv.origin_pos = Position(piece.1, piece.2);
            mv.dest_pos = *dest_pos;
            mv.meta_info = QUIET_MOVE;
            moves.push(mv);
        } else {
            if !is_same_color(piece.0, dest_piece) {
                let mut mv = Move::new();
                mv.origin_piece = piece.0;
                mv.dest_piece = dest_piece;                
                mv.origin_pos = Position(piece.1, piece.2);
                mv.dest_pos = *dest_pos;
                mv.meta_info = CAPTURE;
                moves.push(mv);
            }

            break;
        }
    }

    moves
}

pub fn generate_bishop_moves(piece: PiecePosition, board: &Board) -> MoveList {
    let mut diag = diag_squares(piece.to_position());
    let mut anti_diag = anti_diag_squares(piece.to_position());

    let (pos_diag, neg_diag) = diag.squares.split_at_mut(diag.attacker_index as usize);
    pos_diag.reverse();

    let (pos_anti_diag, neg_anti_diag) = anti_diag.squares.split_at_mut(anti_diag.attacker_index as usize);
    pos_anti_diag.reverse();

    let mut moves = Vec::with_capacity(16);
    moves.append(&mut moves_from_ray(piece, pos_diag, board));
    moves.append(&mut moves_from_ray(piece, neg_diag, board));

    moves.append(&mut moves_from_ray(piece, pos_anti_diag, board));
    moves.append(&mut moves_from_ray(piece, neg_anti_diag, board));

    moves      
}

pub fn generate_rook_moves(piece: PiecePosition, board: &Board) -> MoveList {
    let mut hor = horizontal_squares(piece.to_position());
    let mut vert = vertical_squares(piece.to_position());
    
    let (pos_hor, neg_hor) = hor.squares.split_at_mut(hor.attacker_index as usize);
    let (pos_vert, neg_vert) = vert.squares.split_at_mut(vert.attacker_index as usize);
    pos_hor.reverse();
    pos_vert.reverse();

    let mut moves = Vec::with_capacity(16);
    moves.append(&mut moves_from_ray(piece, pos_hor, board));
    moves.append(&mut moves_from_ray(piece, neg_hor, board));
    moves.append(&mut moves_from_ray(piece, pos_vert, board));
    moves.append(&mut moves_from_ray(piece, neg_vert, board));

    moves   
}

pub fn generate_queen_moves(piece: PiecePosition, board: &Board) -> MoveList {
    iter::empty().chain(generate_bishop_moves(piece, board)).chain(generate_rook_moves(piece, board)).collect::<Vec<Move>>()
}

pub fn generate_knight_moves(piece: PiecePosition, board: &Board) -> MoveList {
    let offsets: [[i8; 2]; 8] = [
        [-1, 2],
        [1, 2],
        [2, -1],
        [2, 1],
        [1, -2],
        [-1, -2],
        [-2, -1],
        [-2, 1],
    ];

    offsets.iter().filter_map(|offset| {
        let f = piece.1 as i8 + offset[0];
        let r = piece.2 as i8 + offset[1];
        let other_piece = board.mb.get(f as File, r as Rank);
        let is_valid = 
            (f >= 0 && f < FILE_COUNT as i8) 
            && (r >= 0 && r < RANK_COUNT as i8) 
            && (!is_same_color(piece.0, other_piece) || other_piece == NO_PIECE);
             
        match is_valid {
            true => {
                let mv = Move {
                    origin_piece: piece.0,
                    origin_pos: Position(piece.1, piece.2),
                    dest_piece: other_piece,
                    dest_pos: Position(f as File, r as Rank),
                    meta_info: if other_piece == NO_PIECE { QUIET_MOVE } else { CAPTURE }
                };

                Some(mv) 
            },
            false => { None }
        }
    }).collect::<Vec<Move>>()
}

pub fn generate_king_moves(piece: PiecePosition, board: &Board) -> MoveList {
    let mut moves: MoveList = Vec::with_capacity(8);
    
    let min_f = cmp::max(piece.1 - 1, 0);
    let max_f = cmp::min(piece.1 + 1, FILE_COUNT - 1);
    
    let min_r = cmp::max(piece.2 - 1, 0);
    let max_r = cmp::min(piece.2 + 1, RANK_COUNT - 1);

    for f in min_f..max_f + 1 {
        for r in min_r..max_r + 1 {
            let other_piece = board.mb.get(f, r);
            if !is_same_color(piece.0, other_piece) || other_piece == NO_PIECE {
                moves.push(Move {
                    origin_piece: piece.0,
                    origin_pos: Position(piece.1, piece.2),
                    dest_piece: other_piece,
                    dest_pos: Position(f, r),
                    meta_info: if other_piece == NO_PIECE { QUIET_MOVE } else { CAPTURE }
                })
            }
            
        }
    }

    moves.into_iter().filter(|m| {
        //filter the ones under attack by opposing pieces
        true
    }).collect::<Vec<Move>>()
}
