
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
    
    //TODO is this being called anywhere?
    pub fn is_valid(&self) -> bool {
        let is_piece = self.origin_piece != NO_PIECE;
        let valid_capture = self.meta_info != CAPTURE || self.dest_piece != NO_PIECE;


        let valid_ep_capture = (
            self.meta_info != EP_CAPTURE
            || (is_white(self.origin_piece) && self.origin_pos.1 == WHITE_EP_CAP_RANK)
            || (!is_white(self.origin_piece) && self.origin_pos.1 == BLACK_EP_CAP_RANK)
        );

        let valid_castling = true;
        let valid_promotion = true;
        
        let positions_valid = self.origin_pos.is_valid() && self.dest_pos.is_valid();

        is_piece && valid_capture && valid_ep_capture && valid_castling && valid_promotion && positions_valid
    }

    pub fn color(&self) -> u8 {
       if self.origin_piece >= B_PAWN && self.origin_piece <= B_KING { BLACK } else { WHITE }
    }

    pub fn kingside_castle(color: Color) -> Move {
        let back_rank = match color {
            WHITE => WHITE_BACK_RANK,
            _ => BLACK_BACK_RANK,
        };

        Move {
            origin_piece: to_color(KING),
            origin_pos: Position(KING_FILE, back_rank),
            dest_piece: NO_PIECE,
            dest_pos: Position(KING_SIDE_CASTLE_FILE, back_rank),
            meta_info: KING_CASTLE 
        }
    }
    
    pub fn kingside_castle(color: Color) -> Move {
        let back_rank = match color {
            WHITE => WHITE_BACK_RANK,
            _ => BLACK_BACK_RANK,
        };

        Move {
            origin_piece: to_color(QUEEN),
            origin_pos: Position(QUEEN_FILE, back_rank),
            dest_piece: NO_PIECE,
            dest_pos: Position(QUEEN_SIDE_CASTLE_FILE, back_rank),
            meta_info: QUEEN_CASTLE 
        }
    }

    // from pure coordinate notation
    pub fn from_pcn_string(move_str: &str, board: &Board) -> Move {
        match move_str {
            "0000" => {
                return Move {
                    origin_piece: NO_PIECE,
                    dest_piece: NO_PIECE,
                    origin_pos: Position(0, 0),
                    dest_pos: Position(0, 0),
                    meta_info: NULL_MOVE, 
                };
            }
            
            "e1g1" if board.castling & W_OO => {
                return Move::kingside_castle(WHITE);
            },

            "e8g8" if board.castling & B_OO => {
                return Move::kingside_castle(BLACK);
            },

            "e1c1" if board.castling & W_OOO => {
                return Move::queenside_castle(WHITE);
            },
            "e8c8" if board.castling & B_OOO => {
                return Move::queenside_castle(BLACK);
            },

            _ => ()
        };

        let mv = Move::new();
        mv.meta_info = QUIET_MOVE;
         
        let moves: Vec<char> = move_str.chars().collect();
        
        mv.origin_pos = Position(
            char_to_file(moves.nth(0)),
            moves.nth(1).parse::<Rank>().unwrap()
        );
        
        mv.dest_pos = Position(
            char_to_file(moves.nth(2)),
            moves.nth(3).parse::<Rank>().unwrap()
        );

        mv.origin_piece = board.getp(mv.origin_pos);
        mv.dest_piece = board.getp(mv.dest_pos);

        if moves.len() >= 5 {
            assert_eq!(to_white(board.getp(mv.origin_pos)), W_PAWN);

            // promotion
            let promotion = moves.nth(4);

            let is_promo_capture = mv.origin_pos.0 != mv.dest_pos.0;

            mv.meta_info = if is_promo_capture {
                match promotion {
                    "q" => QUEEN_PROMO_CAPTURE,
                    "r" => ROOK_PROMO_CAPTURE,
                    "b" => BISHOP_PROMO_CAPTURE,
                    "n" => KNIGHT_PROMO_CAPTURE,
                    _ => 0
                }
            } else {
                match promotion {
                    "q" => QUEEN_PROMOTION,
                    "r" => ROOK_PROMOTION,
                    "b" => BISHOP_PROMOTION,
                    "n" => KNIGHT_PROMOTION,
                    _ => 0
                }
            };

            return mv;
        } else if to_white(board.getp(mv.origin_pos)) == W_PAWN {
            let is_double = (mv.origin_pos.1 - mv.dest_pos.1).abs() > 1;
            if is_double { 
                mv.meta_info = DOUBLE_PAWN_PUSH;
                return mv;
            }
            
            let is_capture = mv.origin_pos.0 != mv.dest_pos.0;
            let is_ep_capture = is_capture && board.getp(mv.dest_pos) == NO_PIECE; 

            if is_ep_capture {
                let neighbor_is_pawn = to_white(board.get(mv.dest_pos.0, mv.origin_pos.1)) == W_PAWN;
                assert!(!is_ep_capture || neighbor_is_pawn);
                assert!(!is_ep_capture || board.en_passant == mv.dest_pos.0);
                mv.meta_info = EP_CAPTURE;
            }
        }

        mv
    }
}

fn promotion_move_type_to_piece(meta_info: MetaInfo, color: Color) -> &'static str {
    match (meta_info) {
        ROOK_PROMOTION | ROOK_PROMO_CAPTURE => "R", 
        KNIGHT_PROMOTION | KNIGHT_PROMO_CAPTURE => "N",
        QUEEN_PROMOTION | QUEEN_PROMO_CAPTURE => "Q",
        BISHOP_PROMOTION | BISHOP_PROMO_CAPTURE => "B",
        _ => "X"
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        match self.meta_info {
            QUIET_MOVE | DOUBLE_PAWN_PUSH =>  {
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
            
            ROOK_PROMOTION | KNIGHT_PROMOTION | QUEEN_PROMOTION | BISHOP_PROMOTION => {
                write!(f, "{}{}{}={}{}{}", 
                    piece_type_to_char(self.origin_piece), 
                    file_to_char(self.origin_pos.0), 
                    self.origin_pos.1 + 1,
                    promotion_move_type_to_piece(self.meta_info, color_of(self.origin_piece)), 
                    file_to_char(self.dest_pos.0), 
                    self.dest_pos.1 + 1,
                )    
            },
            
            ROOK_PROMO_CAPTURE | KNIGHT_PROMO_CAPTURE | QUEEN_PROMO_CAPTURE | BISHOP_PROMO_CAPTURE => {
                write!(f, "{}{}{}x={}{}{}", 
                    piece_type_to_char(self.origin_piece), 
                    file_to_char(self.origin_pos.0), 
                    self.origin_pos.1 + 1,
                    promotion_move_type_to_piece(self.meta_info, color_of(self.origin_piece)), 
                    file_to_char(self.dest_pos.0), 
                    self.dest_pos.1 + 1,
                )    
            },
            
            _ => {
                write!(f, "{}{}{}?{}?{}{}", 
                    piece_type_to_char(self.origin_piece), 
                    file_to_char(self.origin_pos.0), 
                    self.origin_pos.1 + 1,

                    self.meta_info,
                    
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
