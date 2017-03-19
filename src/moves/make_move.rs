use super::types::*;
use util::*;
use constants::*;
use board::Board;
use types::*;

pub fn make_move(current_board: &mut Board, mv: &Move) -> Board {
    let mut board = current_board.clone();
    assert!(mv.is_valid());
    assert!(board.to_move == color_of(mv.origin_piece));

    let is_white = board.to_move == WHITE;
    
    match mv.meta_info {
        QUIET_MOVE => {
            board.mb.move_piece(mv.origin_pos, mv.dest_pos);
            
            if to_white(mv.origin_piece) == W_PAWN {
                board.halfmove_counter = 0;
            } else {
                board.halfmove_counter += 1;
            }
        },
        DOUBLE_PAWN_PUSH => {
            board.en_passant = mv.origin_pos.0;

            board.mb.move_piece(mv.origin_pos, mv.dest_pos);
            board.halfmove_counter = 0;
        },
        CAPTURE => {
            board.mb.move_piece(mv.origin_pos, mv.dest_pos);
            board.halfmove_counter = 0;
        },
        EP_CAPTURE => {
            // move friendly pawn 
            board.mb.move_piece(mv.origin_pos, mv.dest_pos);
            
            let enemy_rank: Rank = if is_white { BLACK_DOUBLE_PUSH_RANK } else { WHITE_DOUBLE_PUSH_RANK };
            
            //should be the opposite color pawn
            assert_eq!(board.mb.get(mv.dest_pos.0, enemy_rank), opposite_color_piece_type(mv.origin_piece));

            // remove the other pawn
            board.mb.set(mv.dest_pos.0, enemy_rank, NO_PIECE);
            board.halfmove_counter = 0;
        },

        KING_CASTLE => {
            // move the king
            board.mb.move_piece(mv.origin_pos, mv.dest_pos);
            
            // move the rook
            board.mb.move_piece(Position(7, mv.origin_pos.1), Position(KING_SIDE_CASTLE_FILE - 1, mv.origin_pos.1));
            
            // update the board history
            let castle_mask = if is_white { W_OO } else { B_OO };
            board.castling = board.castling & !castle_mask;
            board.halfmove_counter +=1;
        },
        
        QUEEN_CASTLE => {
            assert!(to_white(board.mb.getp(mv.origin_pos)) == W_KING);

            // move the king
            board.mb.move_piece(mv.origin_pos, mv.dest_pos);
            
            // move the rook
            assert!(to_white(board.mb.get(0, mv.origin_pos.1)) == W_ROOK);
            board.mb.move_piece(Position(0, mv.origin_pos.1), Position(KING_SIDE_CASTLE_FILE - 1, mv.origin_pos.1));
            
            // update the castling flag
            let castle_mask = if is_white { W_OOO } else { B_OOO };
            board.castling = board.castling & !castle_mask;
            board.halfmove_counter +=1;
        },

        KNIGHT_PROMOTION => {
            board.halfmove_counter = 0;
        },

        BISHOP_PROMOTION => {
            board.halfmove_counter = 0;
        },
        ROOK_PROMOTION => {
            board.halfmove_counter = 0;
        },
        
        QUEEN_PROMOTION => {
            board.halfmove_counter = 0;
        },
        
        KNIGHT_PROMO_CAPTURE => {
            board.halfmove_counter = 0;
        },

        BISHOP_PROMO_CAPTURE => {
            board.halfmove_counter = 0;
        },

        ROOK_PROMO_CAPTURE => {
            board.halfmove_counter = 0;
        },
        
        QUEEN_PROMO_CAPTURE => {
            board.halfmove_counter = 0;
        },
    
        _ => {
         
        },
    }

    if mv.meta_info != DOUBLE_PAWN_PUSH {
        board.en_passant = NO_EN_PASSANT;
    }
    
    let back_rank: Rank = if is_white { WHITE_BACK_RANK } else { BLACK_BACK_RANK };

    // queen side castle invalidation check
    if mv.origin_pos == Position(0, back_rank) && to_white(mv.origin_piece) == W_ROOK {
        let mask = if is_white { W_OOO } else { B_OOO };
        board.castling = board.castling & !mask;
    }
    
    // king side castle invalidation check
    if mv.origin_pos == Position(FILE_COUNT - 1, back_rank) && to_white(mv.origin_piece) == W_ROOK {
        let mask = if is_white { W_OO } else { B_OO };
        board.castling = board.castling & !mask;
    }
    
    if to_white(mv.origin_piece) == W_KING {
        let mask = if is_white { W_OOO | W_OO } else { B_OOO | B_OO };
        board.castling = board.castling & !mask;
    }

    board.to_move = opposite_color(board.to_move);
    
    // if black just moved, increment
    if color_of(mv.origin_piece) == BLACK {
        board.fullmove_counter += 1;
    }

    board.normalize();
    board
}

const TEST_FEN1: &'static str = "r1bk1b1K/pp2p1p1/N1p1Pq1B/2B1rp2/Rn1P1PQP/1p1n1R2/P1P1P1P1/1N6 w - - 6 1";

mod tests {
    #[allow(unused_imports)]
    use super::*;

    fn quiet_move__quiet_non_pawn() {
        let mut board = Board::from_fen(TEST_FEN1);
        assert!(board.halfmove_counter == 6);

        let new_board = make_move(&mut board, &Move {
            origin_pos: Position(5, 2),
            origin_piece: W_ROOK,
            dest_pos: Position(6, 2),
            dest_piece: NO_PIECE,
            meta_info: QUIET_MOVE
        });

        assert!(new_board.mb.get(6, 2) == W_ROOK);
    }
}
