use super::types::*;
use util::*;
use constants::*;
use board::Board;
use types::*;


pub fn make_move(board: &mut Board, mv: &Move) {
    assert!(mv.is_valid());
    assert!(board.to_move == color_of(mv.origin_piece));
    let is_white: bool = board.to_move == WHITE;

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
            assert_eq!(to_white(board.mb.get(mv.origin_pos.0, mv.origin_pos.1)), W_PAWN);
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
            assert!(to_white(board.mb.getp(mv.origin_pos)) == W_PAWN);
            assert!(to_white(board.mb.getp(mv.dest_pos)) == NO_PIECE);
            board.mb.setp(mv.origin_pos, NO_PIECE);
            board.mb.setp(mv.dest_pos, to_color(W_KNIGHT, is_white));
            board.halfmove_counter = 0;
        },

        BISHOP_PROMOTION => {
            assert!(to_white(board.mb.getp(mv.origin_pos)) == W_PAWN);
            assert!(to_white(board.mb.getp(mv.dest_pos)) == NO_PIECE);
            board.mb.setp(mv.origin_pos, NO_PIECE);
            board.mb.setp(mv.dest_pos, to_color(W_BISHOP, is_white));
            board.halfmove_counter = 0;
        },

        ROOK_PROMOTION => {
            assert!(to_white(board.mb.getp(mv.origin_pos)) == W_PAWN);
            assert!(to_white(board.mb.getp(mv.dest_pos)) == NO_PIECE);
            board.mb.setp(mv.origin_pos, NO_PIECE);
            board.mb.setp(mv.dest_pos, to_color(W_ROOK, is_white));
            board.halfmove_counter = 0;
        },
        
        QUEEN_PROMOTION => {
            assert!(to_white(board.mb.getp(mv.origin_pos)) == W_PAWN);
            assert!(to_white(board.mb.getp(mv.dest_pos)) == NO_PIECE);
            board.mb.setp(mv.origin_pos, NO_PIECE);
            board.mb.setp(mv.dest_pos, to_color(W_QUEEN, is_white));
            board.halfmove_counter = 0;
        },
        
        KNIGHT_PROMO_CAPTURE => {
            board.halfmove_counter = 0;
            assert!(to_white(board.mb.getp(mv.origin_pos)) == W_PAWN);
            board.mb.setp(mv.origin_pos, NO_PIECE);
            board.mb.setp(mv.dest_pos, to_color(W_KNIGHT, is_white));

        },

        BISHOP_PROMO_CAPTURE => {
            board.halfmove_counter = 0;
            assert!(to_white(board.mb.getp(mv.origin_pos)) == W_PAWN);
            board.mb.setp(mv.origin_pos, NO_PIECE);
            board.mb.setp(mv.dest_pos, to_color(W_BISHOP, is_white));
        },

        ROOK_PROMO_CAPTURE => {
            assert!(to_white(board.mb.getp(mv.origin_pos)) == W_PAWN);
            board.mb.setp(mv.origin_pos, NO_PIECE);
            board.mb.setp(mv.dest_pos, to_color(W_ROOK, is_white));
            board.halfmove_counter = 0;
        },
        
        QUEEN_PROMO_CAPTURE => {
            assert!(to_white(board.mb.getp(mv.origin_pos)) == W_PAWN);
            board.mb.setp(mv.origin_pos, NO_PIECE);
            board.mb.setp(mv.dest_pos, to_color(W_QUEEN, is_white));
            board.mb.setp(mv.dest_pos, to_color(W_QUEEN, is_white));
            board.halfmove_counter = 0;
        },
    
        _ => {
            assert!(false);
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
}

// Sort of randomized pieces
const TEST_FEN1: &'static str = "r1bk1b1K/pp2p1p1/N1p1Pq1B/2B1rp2/Rn1P1PQP/1p1n1R2/P1P1P1P1/1N6 w - - 6 1";

// Black to move.  Possible to mate.
const TEST_FEN2: &'static str = "r1bk2K1/pp2p1B1/N1p3rq/2B2pQ1/Rn1P1P2/1p1n1RP1/P1P1P2p/1N6 b - - 17 9";

// Black in check.
const TEST_FEN3: &'static str = "1rbk2K1/pp2B1B1/N1p3rq/5pQ1/Rn1P1P2/1p1n1RP1/P1P1P2p/1N6 b - - 19 10";

// White in check but can capture the piece that has the king in check.
const TEST_FEN4: &'static str = "1rb1k1K1/pp4q1/N1p2Br1/5pQ1/Rn1P1P2/1p1n1RP1/P1P1P2p/1N6 w - - 22 12";

// Black pawn can promo capture from c2 to b1
const PROMO_CAPTURE_TEST: &'static str = "r1bk1b1K/pp2p1p1/NBp1Pq1B/4rpQ1/Rn1P1P1P/3n1R2/P1p1P1P1/1N6 b - - 1 2";


mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]        
    fn quiet_move__non_pawn() {
        let mut board = Board::from_fen(TEST_FEN1);
        assert_eq!(board.halfmove_counter, 6);

        make_move(&mut board, &Move {
            origin_pos: Position(5, 2),
            origin_piece: W_ROOK,
            dest_pos: Position(6, 2),
            dest_piece: NO_PIECE,
            meta_info: QUIET_MOVE
        });

        assert_eq!(board.halfmove_counter, 7);
        assert_eq!(board.mb.get(6, 2), W_ROOK);
    }
    
    #[test]        
    fn quiet_move__pawn() {
        let mut board = Board::from_fen(TEST_FEN1);
        assert_eq!(board.mb.get(7, 3), W_PAWN);

        make_move(&mut board, &Move {
            origin_pos: Position(7, 3),
            origin_piece: W_PAWN,
            dest_pos: Position(7, 4),
            dest_piece: NO_PIECE,
            meta_info: QUIET_MOVE
        });

        assert_eq!(board.mb.get(7, 4), W_PAWN);
        assert_eq!(board.halfmove_counter, 0);
    }


    #[test]        
    fn double_push__pawn() {
        let mut board = Board::from_fen("r2qk2r/p5bp/3p2p1/1p2Pp1n/2PB1Qb1/7P/PP4P1/RN2KB1R w KQkq f6 5 3");
        
        make_move(&mut board, &Move {
           origin_piece: W_PAWN,
           dest_piece: NO_PIECE,
           origin_pos: Position(0,1),
           dest_pos: Position(0,3),
           meta_info: DOUBLE_PAWN_PUSH
        });
        
        assert_eq!(board.mb.get(0, 1), NO_PIECE);
        assert_eq!(board.mb.get(0, 2), NO_PIECE);
        assert_eq!(board.mb.get(0, 3), W_PAWN);
        assert_eq!(board.halfmove_counter, 0);
    }
    
    #[test]        
    fn ep_capture__pawn() {
        let mut board = Board::from_fen("r2qk2r/p5bp/3p2p1/1p2Pp1n/2PB1Qb1/7P/PP4P1/RN2KB1R w KQkq f6 5 3");
        make_move(&mut board, &Move {
           origin_piece: W_PAWN,
           dest_piece: NO_PIECE,
           origin_pos: Position(4,4),
           dest_pos: Position(5,5),
           meta_info: EP_CAPTURE
        });

        assert_eq!(board.mb.get(4, 4), NO_PIECE);
        assert_eq!(board.mb.get(5, 5), W_PAWN);
        assert_eq!(board.mb.get(4, 5), NO_PIECE);
        assert_eq!(board.halfmove_counter, 0);
    }
    
    #[test]        
    fn capture_move() {
        let mut board = Board::from_fen(TEST_FEN1);
        assert_eq!(board.mb.get(5, 3), W_PAWN);

        make_move(&mut board, &Move {
            origin_pos: Position(5, 3),
            origin_piece: W_PAWN,
            dest_pos: Position(4, 4),
            dest_piece: B_ROOK,
            meta_info: CAPTURE 
        });

        assert_eq!(board.mb.get(5, 3), NO_PIECE);
        assert_eq!(board.mb.get(4, 4), W_PAWN);
    }
    
    #[test]        
    fn knight_promo_move() {
        let mut board = Board::from_fen(TEST_FEN2);
        
        assert_eq!(board.mb.get(F8, R2), B_PAWN);
        assert_eq!(board.mb.get(F8, R1), NO_PIECE);
        board.print_board();

        make_move(&mut board, &Move {
            origin_pos: Position(F8, R2),
            origin_piece: B_PAWN,
            dest_pos: Position(F8, R1),
            dest_piece: B_KNIGHT,
            meta_info: KNIGHT_PROMOTION
        });

        assert_eq!(board.mb.get(F8, R2), NO_PIECE);
        assert_eq!(board.mb.get(F8, R1), B_KNIGHT);
    }
    
    #[test]        
    fn bishop_promo_move() {
        let mut board = Board::from_fen(TEST_FEN2);
        
        assert_eq!(board.mb.get(F8, R2), B_PAWN);
        assert_eq!(board.mb.get(F8, R1), NO_PIECE);
        board.print_board();

        make_move(&mut board, &Move {
            origin_pos: Position(F8, R2),
            origin_piece: B_PAWN,
            dest_pos: Position(F8, R1),
            dest_piece: B_BISHOP,
            meta_info: BISHOP_PROMOTION
        });

        assert_eq!(board.mb.get(F8, R2), NO_PIECE);
        assert_eq!(board.mb.get(F8, R1), B_BISHOP);
    }
    
    #[test]
    fn rook_promo_move() {
        let mut board = Board::from_fen(TEST_FEN2);
         
        assert_eq!(board.mb.get(F8, R2), B_PAWN);
        assert_eq!(board.mb.get(F8, R1), NO_PIECE);

        board.print_board();

        make_move(&mut board, &Move {
            origin_pos: Position(F8, R2),
            origin_piece: B_PAWN,
            dest_pos: Position(F8, R1),
            dest_piece: B_ROOK,
            meta_info: ROOK_PROMOTION
        });

        assert_eq!(board.mb.get(F8, R2), NO_PIECE);
        assert_eq!(board.mb.get(F8, R1), B_ROOK);

    }
    
    #[test]
    fn queen_promo_move() {
        let mut board = Board::from_fen(TEST_FEN2);
         
        assert_eq!(board.mb.get(F8, R2), B_PAWN);
        assert_eq!(board.mb.get(F8, R1), NO_PIECE);

        make_move(&mut board, &Move {
            origin_pos: Position::from_pgn("h2"),
            origin_piece: B_PAWN,
            dest_pos: Position::from_pgn("h1"),
            dest_piece: B_QUEEN,
            meta_info: QUEEN_PROMOTION
        });

        assert_eq!(board.mb.getp(Position::from_pgn("h2")), NO_PIECE);
        assert_eq!(board.mb.getp(Position::from_pgn("h1")), B_QUEEN);
    }
    
    #[test]
    fn knight_promo_capture() {
        let mut board = Board::from_fen(PROMO_CAPTURE_TEST);
        assert_eq!(board.mb.getp(Position::from_pgn("c2")), B_PAWN);
        assert_eq!(board.mb.getp(Position::from_pgn("b1")), W_KNIGHT);

        make_move(&mut board, &Move {
            origin_pos: Position::from_pgn("c2"),
            origin_piece: B_PAWN,
            dest_pos: Position::from_pgn("b1"),
            dest_piece: B_KNIGHT,
            meta_info: KNIGHT_PROMO_CAPTURE
        });
        
        assert_eq!(board.mb.getp(Position::from_pgn("c2")), NO_PIECE);
        assert_eq!(board.mb.getp(Position::from_pgn("b1")), B_KNIGHT);
        assert_eq!(board.mb.getp(Position::from_pgn("b2")), NO_PIECE);
    }

    #[test]
    fn bishop_promo_capture() {
        let mut board = Board::from_fen(PROMO_CAPTURE_TEST);
        assert_eq!(board.mb.getp(Position::from_pgn("c2")), B_PAWN);
        assert_eq!(board.mb.getp(Position::from_pgn("b1")), W_KNIGHT);

        make_move(&mut board, &Move {
            origin_pos: Position::from_pgn("c2"),
            origin_piece: B_PAWN,
            dest_pos: Position::from_pgn("b1"),
            dest_piece: B_BISHOP,
            meta_info: BISHOP_PROMO_CAPTURE
        });
        
        assert_eq!(board.mb.getp(Position::from_pgn("c2")), NO_PIECE);
        assert_eq!(board.mb.getp(Position::from_pgn("b1")), B_BISHOP);
        assert_eq!(board.mb.getp(Position::from_pgn("b2")), NO_PIECE);
    }

    #[test]
    fn rook_promo_capture() {
        let mut board = Board::from_fen(PROMO_CAPTURE_TEST);
        assert_eq!(board.mb.getp(Position::from_pgn("c2")), B_PAWN);
        assert_eq!(board.mb.getp(Position::from_pgn("b1")), W_KNIGHT);

        make_move(&mut board, &Move {
            origin_pos: Position::from_pgn("c2"),
            origin_piece: B_PAWN,
            dest_pos: Position::from_pgn("b1"),
            dest_piece: B_ROOK,
            meta_info: ROOK_PROMO_CAPTURE
        });
        
        assert_eq!(board.mb.getp(Position::from_pgn("c2")), NO_PIECE);
        assert_eq!(board.mb.getp(Position::from_pgn("b1")), B_ROOK);
        assert_eq!(board.mb.getp(Position::from_pgn("b2")), NO_PIECE);
    }
    
    #[test]
    fn queen_promo_capture() {
        let mut board = Board::from_fen(PROMO_CAPTURE_TEST);
        board.print_board();
        assert_eq!(board.mb.getp(Position::from_pgn("c2")), B_PAWN);
        assert_eq!(board.mb.getp(Position::from_pgn("b1")), W_KNIGHT);

        make_move(&mut board, &Move {
            origin_pos: Position::from_pgn("c2"),
            origin_piece: B_PAWN,
            dest_pos: Position::from_pgn("b1"),
            dest_piece: B_QUEEN,
            meta_info: QUEEN_PROMO_CAPTURE
        });
        
        assert_eq!(board.mb.getp(Position::from_pgn("c2")), NO_PIECE);
        assert_eq!(board.mb.getp(Position::from_pgn("b1")), B_QUEEN);
        assert_eq!(board.mb.getp(Position::from_pgn("b2")), NO_PIECE);
    }
}
