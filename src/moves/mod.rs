mod tests;
mod types;
mod castling;

use self::types::*;
use types::*;
use util::*;
use constants::*;
use board::Board;
use std::iter;
use std::cmp;

// maybe in the future we could convert all these generator 
// functions to return boxed iterators.
//let x: Box<Iterator<Item=Move>> = Box::new(moves.into_iter());

//pub fn generate_moves(board: Board, piece: PiecePosition) -> MoveList {
//    generate_bishop_moves(board)
//        .chain(generate_rook_moves())
//        .chain(generate_knight_moves())
//        .chain(generate_pawn_moves())
//        .chain(generate_queen_moves())
//        .chain(generate_king_moves());
//}

// NOTE: If we were really fancy, we'd have custom Iterator structs for each move generation
// function.  This, as far as I can imagine, is the only way to allow for early termination when
// generating moves.  If we want the next move to be generated only as a result of calling
// `next`, then it must be it's own struct.  This is because, across calls to `next` the exact
// location of iteration would need to be stored as data on the struct.  (static function
// variables, if they existed, could theoretically be used to simulate generator calls. if we had
// SFV then we wouldn't need to define a new struct.  However, we'd need to still define the static
// variables themselves in the function.  However, with SFV we would be able to have a unified
// sized iterator type which just has a reference to a generator function.
//
// In the mean time, we can just try to return MoveIters whereever possible and see where that
// leads us.  Even in that configuration we still could achieve early termination via the use of
// chaining operations.  
// 

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
            res.attacker_index = f as u8;
        }
    }

    res
}

pub fn vertical_squares(pos: Position) -> AttackingRay {
    let mut res = AttackingRay::new();
    
    for r in 0..RANK_COUNT {
        res.squares.push(Position(pos.0, r)); 
        if pos.1 == r {
            res.attacker_index = r as u8;
        }
    }

    res
}
    
fn moves_from_ray(piece: PiecePosition, squares: &[Position], board: &Board, as_attacks: bool) -> Vec<Move> {
    let mut moves: MoveList = Vec::with_capacity(20);

    for dest_pos in squares.iter() {
        let dest_piece = board.mb.get(dest_pos.0, dest_pos.1);

        if &piece.to_position() == dest_pos {
            //do nothing.
            //TODO: modify the generate bishop moves function to not return the square with the
            //attacker itself
        } else if dest_piece == NO_PIECE {
            let mut mv = Move::new();
            mv.origin_piece = piece.0;
            mv.dest_piece = dest_piece;                
            mv.origin_pos = Position(piece.1, piece.2);
            mv.dest_pos = *dest_pos;
            mv.meta_info = QUIET_MOVE;
            moves.push(mv);
        } else {
            if !is_same_color(piece.0, dest_piece) || as_attacks {
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

pub fn generate_bishop_moves(piece: PiecePosition, board: &Board, as_attacks: bool) -> MovesIter {
    let mut diag = diag_squares(piece.to_position());
    let mut anti_diag = anti_diag_squares(piece.to_position());

    let (pos_diag, neg_diag) = diag.squares.split_at_mut(diag.attacker_index as usize);
    pos_diag.reverse();

    let (pos_anti_diag, neg_anti_diag) = anti_diag.squares.split_at_mut(anti_diag.attacker_index as usize);
    pos_anti_diag.reverse();

    let mut moves = Vec::with_capacity(16);
    moves.append(&mut moves_from_ray(piece, pos_diag, board, as_attacks));
    moves.append(&mut moves_from_ray(piece, neg_diag, board, as_attacks));

    moves.append(&mut moves_from_ray(piece, pos_anti_diag, board, as_attacks));
    moves.append(&mut moves_from_ray(piece, neg_anti_diag, board, as_attacks));
    
    MovesIter::from_vec(moves)
}

pub fn generate_rook_moves(piece: PiecePosition, board: &Board, as_attacks: bool) -> MovesIter {
    let mut hor = horizontal_squares(piece.to_position());
    let mut vert = vertical_squares(piece.to_position());
    
    let (pos_hor, neg_hor) = hor.squares.split_at_mut(hor.attacker_index as usize);
    let (pos_vert, neg_vert) = vert.squares.split_at_mut(vert.attacker_index as usize);
    pos_hor.reverse();
    pos_vert.reverse();

    let mut moves = Vec::with_capacity(16);
    moves.append(&mut moves_from_ray(piece, pos_hor, board, as_attacks));
    moves.append(&mut moves_from_ray(piece, neg_hor, board, as_attacks));
    moves.append(&mut moves_from_ray(piece, pos_vert, board, as_attacks));
    moves.append(&mut moves_from_ray(piece, neg_vert, board, as_attacks));
    
    MovesIter::from_vec(moves)
}

pub fn generate_queen_moves(piece: PiecePosition, board: &Board, as_attacks: bool) -> MovesIter {
    let moves = iter::empty()
        .chain(generate_bishop_moves(piece, board, as_attacks))
        .chain(generate_rook_moves(piece, board, as_attacks))
        .collect::<Vec<Move>>();

    MovesIter::from_vec(moves)
}

pub fn generate_knight_moves(piece: PiecePosition, board: &Board, as_attacks: bool) -> MovesIter {
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

    let moves = offsets.iter().filter_map(|offset| {
        let f = piece.1 as i8 + offset[0];
        let r = piece.2 as i8 + offset[1];
        
        let other_piece = board.mb.get(f as File, r as Rank);
        
        let is_valid = 
            other_piece != OFF_BOARD
            //(f >= 0 && f < FILE_COUNT as i8) 
            //&& (r >= 0 && r < RANK_COUNT as i8) 
            && (!is_same_color(piece.0, other_piece) || other_piece == NO_PIECE || as_attacks);
        if is_valid {
            let mv = Move {
                origin_piece: piece.0,
                origin_pos: Position(piece.1, piece.2),
                dest_piece: other_piece,
                dest_pos: Position(f as File, r as Rank),
                meta_info: if other_piece == NO_PIECE { QUIET_MOVE } else { CAPTURE }
            };

            Some(mv) 
        } else {
            None
        }
    }).collect::<Vec<Move>>();

    MovesIter::from_vec(moves)
}
    
const KING_OFFSETS: [[i8; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
    [1, 0],
    [1, -1],
    [0, -1],
];



pub fn _generate_pawn_promotions(piece: PiecePosition, board: &Board, as_attacks: bool) -> MovesIter {
    MovesIter::from_vec(vec![])
}

pub fn is_color_in_check(board: &Board, color: Color) -> bool {
    match board.get_first_piece(W_KING) {
        Some(piece_pos) =>  is_pos_attacked_by(board, piece_pos.to_position(), opposite_color(color)),
        None => false
    }
}

pub fn generate_king_moves(piece: PiecePosition, board: &Board, as_attacks: bool) -> MovesIter {
    let color = color_of(piece.0);
    assert!(to_white(piece.0) == W_KING);

    let moves = KING_OFFSETS.iter().filter_map(|offset| {
        let f: File = piece.1 as i8 + offset[0];
        let r: Rank = piece.2 as i8 + offset[1];
        let other_piece = board.mb.get(f, r);
        

        if other_piece == OFF_BOARD {
            None
        } else {
            //println!("Testing: {:?} + {:?} => {}, {}", piece.to_position(), offset, f, r);
            let other_piece = board.mb.get(f as File, r as Rank);
            
            let mut can_attack = (
                other_piece != OFF_BOARD
                && as_attacks || (
                    !is_same_color(piece.0, other_piece)
                    && !is_pos_attacked_by(board, Position(f, r), opposite_color(color))
                )
            );

            if can_attack {
                Some(Move {
                    origin_piece: piece.0,
                    origin_pos: Position::new(piece.1, piece.2),
                    dest_piece: other_piece,
                    dest_pos: Position::new(f as File, r as Rank),
                    meta_info: if other_piece == NO_PIECE { QUIET_MOVE } else { CAPTURE }
                })
            } else {
                None
            }
        }
    }).collect::<Vec<Move>>();

    MovesIter::from_vec(moves)
}

//query piece P on board B where P
pub fn does_piece_attack(piece: PiecePosition, target_position: Position, board: &Board) -> bool {  
    // start casting your moves list as iterators and attempting to use them only from that
    // interface.  particularly, try to cancel out of the iterator early if you can.
    get_piece_attacks(piece, board).find(|m: &Move| { 
        //println!("is {:?} attacking {:?}?", m.dest_pos, Position(target_position.0, target_position.1));
        m.dest_pos == Position(target_position.0, target_position.1) 
    }).is_some()
}
    
// NOTE: this is different from a list of moves.  it represents theoretical attacks.  
// for example, pawns might move forward one or two, but they may only attack their 
// diagnals.
pub fn get_piece_attacks(piece: PiecePosition, board: &Board) -> MovesIter { 
    match to_white(piece.0) {
        PAWN => generate_pawn_moves(piece, board, true, true),
        KNIGHT => generate_knight_moves(piece, board, true),
        BISHOP => generate_bishop_moves(piece, board, true),
        ROOK => generate_rook_moves(piece, board, true),
        QUEEN => generate_queen_moves(piece, board, true),
        KING => generate_king_moves(piece, board, true),
        _ => MovesIter::from_vec(vec![])
    }
}

pub fn is_pos_attacked_by(board: &Board, pos: Position, color: Color) -> bool {
    println!("is_pos_attacked_by: {:?}", pos);
    // for each piece, does it attack this square
    let r = board.get_pieces_iter().find(|piece_pos| {
        color_of(piece_pos.0) == color && does_piece_attack(*piece_pos, pos, board)
    }).is_some();
    
    println!("is pos attacked: {:?} by color({}).... {}", pos, color, r);
    r
}

pub fn get_attackers(board: &Board, pos: Position) -> Vec<PiecePosition> {
    // logically:  query pieces p where get_attacks(p) has_member sq(e4)
    //
    // or iteratorate through all pieces, and filter
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
        get_piece_attacks(p.clone(), board).find(|m| m.dest_pos == p.to_position()).is_some()
    }).collect::<Vec<_>>()
}

pub fn generate_pawn_moves(piece: PiecePosition, board: &Board, include_ep: bool, attacks_only: bool) -> MovesIter {
    let mut moves: MoveList = Vec::with_capacity(8);
    
    let starting_rank: Rank;
    let y_dir_sign: Rank;
    let double_push_rank: Rank;
    let single_push_rank: Rank;
    let back_rank: Rank;
    let ep_cap_rank: Rank;
    
    //NOTE: this code is not universal board compatible
    if piece.is_white() { 
        y_dir_sign = WHITE_Y_DIR_SIGN;
        starting_rank = WHITE_PAWN_STARTING_RANK;
        single_push_rank = WHITE_SINGLE_PUSH_RANK;
        double_push_rank = WHITE_DOUBLE_PUSH_RANK;
        back_rank = WHITE_BACK_RANK;
        ep_cap_rank = WHITE_EP_CAP_RANK
    } else { 
        y_dir_sign = BLACK_Y_DIR_SIGN;
        starting_rank = BLACK_PAWN_STARTING_RANK;
        single_push_rank = BLACK_SINGLE_PUSH_RANK;
        double_push_rank = BLACK_DOUBLE_PUSH_RANK;
        back_rank = BLACK_BACK_RANK;
        ep_cap_rank = BLACK_EP_CAP_RANK
    }

    let can_double_push =
        !attacks_only 
        && piece.2 == starting_rank 
        && board.mb.get(piece.1, single_push_rank) == NO_PIECE 
        && board.mb.get(piece.1, double_push_rank) == NO_PIECE;
    
    if can_double_push {
        let push_pos: Position = if piece.is_white() {
           Position(piece.1, 3)  
        } else {
           Position(piece.1, 4)  
        };

        let m = Move {
            origin_piece: piece.0,
            origin_pos: piece.to_position(),
            dest_piece: NO_PIECE,
            dest_pos: push_pos,
            meta_info: DOUBLE_PAWN_PUSH
        };

        moves.push(m);
    }

    let can_push = 
        !attacks_only && 
        board.mb.get(piece.1, (piece.2 as i8 + y_dir_sign) as i8) == NO_PIECE;

    if can_push {
        moves.push( Move {
            origin_piece: piece.0,
            origin_pos: piece.to_position(),
            dest_piece: NO_PIECE,
            dest_pos: Position(piece.1, (piece.2 as i8 + y_dir_sign) as i8),
            meta_info: QUIET_MOVE
        })    
    }

    if board.en_passant != NO_EN_PASSANT && include_ep && !attacks_only{
        let file_offset: i8 = (piece.1 as i8 - board.en_passant as i8) as i8;
        if piece.2 == ep_cap_rank && file_offset.abs() == 1 {
            let m = Move {
                origin_piece: piece.0,
                origin_pos: piece.to_position(),
                dest_piece: NO_PIECE,
                dest_pos: Position((piece.1 - file_offset) as i8, (piece.2 as i8 + y_dir_sign) as i8),
                meta_info: EP_CAPTURE
            };

            moves.push(m);
        }
    }
    
    let left_dest_rank: Rank = piece.2 + y_dir_sign;
    let left_dest_file: File = piece.1 - 1;

    //todo: redundant  
    let right_dest_rank: Rank = piece.2 + y_dir_sign;
    let right_dest_file: File = piece.1 + 1;

    let can_left_capture = 
        left_dest_file >= 0
        && (attacks_only || is_occupied_and_enemy(
            board.mb.get(left_dest_file, left_dest_rank),
            piece.0));

    let can_right_capture = 
        right_dest_file < FILE_COUNT 
        && (attacks_only || is_occupied_and_enemy(
            board.mb.get(right_dest_file, right_dest_rank),
            piece.0));

    
    if can_left_capture {
        let dest_piece = board.mb.get(left_dest_file, left_dest_rank);

        let m = Move {
            origin_piece: piece.0,
            origin_pos: Position(piece.1, piece.2),
            dest_piece: dest_piece,
            dest_pos: Position(left_dest_file, left_dest_rank),
            meta_info: if dest_piece == NO_PIECE { QUIET_MOVE } else { CAPTURE }
        };

        moves.push(m);
    }

    if can_right_capture {
        let dest_piece = board.mb.get(right_dest_file, right_dest_rank);
        let m = Move {
            origin_piece: piece.0,
            origin_pos: Position(piece.1, piece.2),
            dest_piece: dest_piece,
            dest_pos: Position(right_dest_file, right_dest_rank),
            meta_info: if dest_piece == NO_PIECE { QUIET_MOVE } else { CAPTURE }
        };

        moves.push(m);  
    }

    MovesIter::from_vec(moves)
}

