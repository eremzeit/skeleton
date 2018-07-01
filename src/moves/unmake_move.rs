use super::types::*;
use util::{is_white, color_of, to_white, opposite_color, to_color};
use constants::*;
use board::Board;
use types::{Position};


use super::make_move::{make_move};
use board::utils::{
    are_boards_equal, 
    assert_boards_equal
};

// pub origin_piece: PieceType,
// pub dest_piece: PieceType,
// pub origin_pos: Position,
// pub dest_pos: Position,
// pub meta_info: MetaInfo

pub fn unmake_move(board: &mut Board, mv: &Move) {
    assert!(mv.is_valid());
    assert!(board.to_move == opposite_color(color_of(mv.origin_piece)));

    let white_did_move: bool = board.to_move == WHITE;

    let last_move = board.history.pop().unwrap();

    match mv.meta_info {
        QUIET_MOVE | DOUBLE_PAWN_PUSH => {
            assert!(board.mb.getp(mv.dest_pos) != NO_PIECE);
            board.mb.move_piece(mv.dest_pos, mv.origin_pos);
            assert!(board.mb.getp(mv.dest_pos) != mv.origin_piece);
        },

        CAPTURE => {
            board.mb.move_piece(mv.dest_pos, mv.origin_pos);
            board.mb.setp(mv.dest_pos, mv.dest_piece);
        },

        EP_CAPTURE => {
            //let mut board = Board::from_fen("r2qk2r/p5bp/3p2p1/1p2Pp1n/2PB1Qb1/7P/PP4P1/RN2KB1R w KQkq f6 5 3");
            board.mb.move_piece(mv.dest_pos, mv.origin_pos);

            // When ep is possible, the would-be captured pawn is on the same rank as the capturing
            // piece and is on the same file as where the capturing pawn ended up.
            
            let pawn_type = to_color(W_PAWN, !is_white(mv.origin_piece));
            board.mb.setp(Position(mv.dest_pos.0, mv.origin_pos.1), pawn_type);
        },

        KING_CASTLE => {
            // // move the king
            board.mb.move_piece(mv.dest_pos, mv.origin_pos);
            // 
            // move the rook
            board.mb.move_piece(Position(KING_SIDE_CASTLE_FILE - 1, mv.origin_pos.1), Position(7, mv.origin_pos.1));
        },
        
        QUEEN_CASTLE => {
            // move the king
            board.mb.move_piece(mv.dest_pos, mv.origin_pos);
            // 
            // move the rook
            board.mb.move_piece(Position(QUEEN_SIDE_CASTLE_FILE + 1, mv.origin_pos.1), Position(0, mv.origin_pos.1));
        },

        KNIGHT_PROMOTION | BISHOP_PROMOTION | ROOK_PROMOTION | QUEEN_PROMOTION  => {
            board.mb.setp(mv.dest_pos, NO_PIECE);
            board.mb.setp(mv.origin_pos, mv.origin_piece);
        },

        KNIGHT_PROMO_CAPTURE | BISHOP_PROMO_CAPTURE | ROOK_PROMO_CAPTURE | QUEEN_PROMO_CAPTURE => {
            board.mb.setp(mv.dest_pos, mv.dest_piece);
            board.mb.setp(mv.origin_pos, mv.origin_piece);
        },

        _ => {
            assert!(false);
        },
    }

    board.reset_via_move_context(&last_move);
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
    fn quiet_move_non_pawn() {
        let mut board = Board::from_fen(TEST_FEN1);
        let orig_board = board.clone();

        assert_eq!(board.halfmove_counter, 6);

        let mv: Move = Move {
            origin_pos: Position(5, 2),
            origin_piece: W_ROOK,
            dest_pos: Position(6, 2),
            dest_piece: NO_PIECE,
            meta_info: QUIET_MOVE
        };

        make_move(&mut board, mv.clone());

        unmake_move(&mut board, &mv);

        assert!(are_boards_equal(&board, &orig_board));
    }
    
    #[test]        
    fn quiet_move_pawn() {
        let mut board = Board::from_fen(TEST_FEN1);
        let orig_board = board.clone();
        
        assert_eq!(board.mb.get(7, 3), W_PAWN);
        assert_eq!(board.halfmove_counter, 6);

        let mv: Move = Move {
            origin_pos: Position(7, 3),
            origin_piece: W_PAWN,
            dest_pos: Position(7, 4),
            dest_piece: NO_PIECE,
            meta_info: QUIET_MOVE
        };

        make_move(&mut board, mv.clone());
        unmake_move(&mut board, &mv);

        assert!(are_boards_equal(&board, &orig_board));
        assert_eq!(board.halfmove_counter, 6);
    }


    #[test]        
    fn double_push_pawn() {
        let mut board = Board::from_fen("r2qk2r/p5bp/3p2p1/1p2Pp1n/2PB1Qb1/7P/PP4P1/RN2KB1R w KQkq f6 5 3");
        let orig_board = board.clone();
        
        let mv: Move = Move {
           origin_piece: W_PAWN,
           dest_piece: NO_PIECE,
           origin_pos: Position(0,1),
           dest_pos: Position(0,3),
           meta_info: DOUBLE_PAWN_PUSH
        };

        make_move(&mut board, mv.clone());
        unmake_move(&mut board, &mv);

        assert!(are_boards_equal(&board, &orig_board));
    }
    
    #[test]        
    fn ep_capture_pawn() {
        let mut board = Board::from_fen("r2qk2r/p5bp/3p2p1/1p2Pp1n/2PB1Qb1/7P/PP4P1/RN2KB1R w KQkq f6 5 3");
        let orig_board = board.clone();

        let mv = Move {
           origin_piece: W_PAWN,
           dest_piece: NO_PIECE,
           origin_pos: Position(4,4),
           dest_pos: Position(5,5),
           meta_info: EP_CAPTURE
        };
        
        orig_board.print_board();
        make_move(&mut board, mv.clone());
        board.print_board();
        unmake_move(&mut board, &mv);
        board.print_board();
        assert_boards_equal(&board, &orig_board);
    }
    
    #[test]        
    fn capture_move() {
        let mut board = Board::from_fen(TEST_FEN1);
        let orig_board = board.clone();
        let mv = Move {
            origin_pos: Position(5, 3),
            origin_piece: W_PAWN,
            dest_pos: Position(4, 4),
            dest_piece: B_ROOK,
            meta_info: CAPTURE 
        };

        make_move(&mut board, mv.clone());
        unmake_move(&mut board, &mv);

        assert!(are_boards_equal(&board, &orig_board));

    }
    
    #[test]        
    fn knight_promo_move() {
        let mut board = Board::from_fen(TEST_FEN2);
        let orig_board = board.clone();
        
        let mv = Move {
            origin_pos: Position(F8, R2),
            origin_piece: B_PAWN,
            dest_pos: Position(F8, R1),
            dest_piece: B_KNIGHT,
            meta_info: KNIGHT_PROMOTION
        };

        make_move(&mut board, mv.clone());
        unmake_move(&mut board, &mv);
        assert_boards_equal(&board, &orig_board);
    }
    
    #[test]        
    fn bishop_promo_move() {
        let mut board = Board::from_fen(TEST_FEN2);
        let orig_board = board.clone();
        
        let mv = Move {
            origin_pos: Position(F8, R2),
            origin_piece: B_PAWN,
            dest_pos: Position(F8, R1),
            dest_piece: B_BISHOP,
            meta_info: BISHOP_PROMOTION
        };

        make_move(&mut board, mv.clone());
        unmake_move(&mut board, &mv);
        assert_boards_equal(&board, &orig_board);
    }
    
    #[test]
    fn rook_promo_move() {
        // let mut board = Board::from_fen(TEST_FEN2);
        //  
        // assert_eq!(board.mb.get(F8, R2), B_PAWN);
        // assert_eq!(board.mb.get(F8, R1), NO_PIECE);

        // board.print_board();

        // make_move(&mut board, &Move {
        //     origin_pos: Position(F8, R2),
        //     origin_piece: B_PAWN,
        //     dest_pos: Position(F8, R1),
        //     dest_piece: B_ROOK,
        //     meta_info: ROOK_PROMOTION
        // });

        // assert_eq!(board.mb.get(F8, R2), NO_PIECE);
        // assert_eq!(board.mb.get(F8, R1), B_ROOK);

    }
    
    #[test]
    fn queen_promo_move() {
        // let mut board = Board::from_fen(TEST_FEN2);
        //  
        // assert_eq!(board.mb.get(F8, R2), B_PAWN);
        // assert_eq!(board.mb.get(F8, R1), NO_PIECE);

        // make_move(&mut board, &Move {
        //     origin_pos: Position::from_pgn("h2"),
        //     origin_piece: B_PAWN,
        //     dest_pos: Position::from_pgn("h1"),
        //     dest_piece: B_QUEEN,
        //     meta_info: QUEEN_PROMOTION
        // });

        // assert_eq!(board.mb.getp(Position::from_pgn("h2")), NO_PIECE);
        // assert_eq!(board.mb.getp(Position::from_pgn("h1")), B_QUEEN);
    }
    
    #[test]
    fn knight_promo_capture() {
        // let mut board = Board::from_fen(PROMO_CAPTURE_TEST);
        // assert_eq!(board.mb.getp(Position::from_pgn("c2")), B_PAWN);
        // assert_eq!(board.mb.getp(Position::from_pgn("b1")), W_KNIGHT);

        // make_move(&mut board, &Move {
        //     origin_pos: Position::from_pgn("c2"),
        //     origin_piece: B_PAWN,
        //     dest_pos: Position::from_pgn("b1"),
        //     dest_piece: B_KNIGHT,
        //     meta_info: KNIGHT_PROMO_CAPTURE
        // });
        // 
        // assert_eq!(board.mb.getp(Position::from_pgn("c2")), NO_PIECE);
        // assert_eq!(board.mb.getp(Position::from_pgn("b1")), B_KNIGHT);
        // assert_eq!(board.mb.getp(Position::from_pgn("b2")), NO_PIECE);
    }

    #[test]
    fn bishop_promo_capture() {
        // let mut board = Board::from_fen(PROMO_CAPTURE_TEST);
        // assert_eq!(board.mb.getp(Position::from_pgn("c2")), B_PAWN);
        // assert_eq!(board.mb.getp(Position::from_pgn("b1")), W_KNIGHT);

        // make_move(&mut board, &Move {
        //     origin_pos: Position::from_pgn("c2"),
        //     origin_piece: B_PAWN,
        //     dest_pos: Position::from_pgn("b1"),
        //     dest_piece: B_BISHOP,
        //     meta_info: BISHOP_PROMO_CAPTURE
        // });
        // 
        // assert_eq!(board.mb.getp(Position::from_pgn("c2")), NO_PIECE);
        // assert_eq!(board.mb.getp(Position::from_pgn("b1")), B_BISHOP);
        // assert_eq!(board.mb.getp(Position::from_pgn("b2")), NO_PIECE);
    }

    #[test]
    fn rook_promo_capture() {
        // let mut board = Board::from_fen(PROMO_CAPTURE_TEST);
        // assert_eq!(board.mb.getp(Position::from_pgn("c2")), B_PAWN);
        // assert_eq!(board.mb.getp(Position::from_pgn("b1")), W_KNIGHT);

        // make_move(&mut board, &Move {
        //     origin_pos: Position::from_pgn("c2"),
        //     origin_piece: B_PAWN,
        //     dest_pos: Position::from_pgn("b1"),
        //     dest_piece: B_ROOK,
        //     meta_info: ROOK_PROMO_CAPTURE
        // });
        // 
        // assert_eq!(board.mb.getp(Position::from_pgn("c2")), NO_PIECE);
        // assert_eq!(board.mb.getp(Position::from_pgn("b1")), B_ROOK);
        // assert_eq!(board.mb.getp(Position::from_pgn("b2")), NO_PIECE);
    }
    
    #[test]
    fn queen_promo_capture() {
        // let mut board = Board::from_fen(PROMO_CAPTURE_TEST);
        // board.print_board();
        // assert_eq!(board.mb.getp(Position::from_pgn("c2")), B_PAWN);
        // assert_eq!(board.mb.getp(Position::from_pgn("b1")), W_KNIGHT);

        // make_move(&mut board, &Move {
        //     origin_pos: Position::from_pgn("c2"),
        //     origin_piece: B_PAWN,
        //     dest_pos: Position::from_pgn("b1"),
        //     dest_piece: B_QUEEN,
        //     meta_info: QUEEN_PROMO_CAPTURE
        // });
        // 
        // assert_eq!(board.mb.getp(Position::from_pgn("c2")), NO_PIECE);
        // assert_eq!(board.mb.getp(Position::from_pgn("b1")), B_QUEEN);
        // assert_eq!(board.mb.getp(Position::from_pgn("b2")), NO_PIECE);
    }
}
