use super::{Board};
use constants::{Rank, File, PieceType, FILE_COUNT, RANK_COUNT, BLACK, WHITE};
use util::{piece_list_to_string};
use board::{parse_fen_pieces};
use board::mailbox::{Mailbox};

pub struct PieceIter<'a> {
    board: &'a Board,
    current_rank: Rank,
    current_file: File,
}

impl<'a> PieceIter<'a> {
    pub fn new(board: &'a Board) -> Self {
        PieceIter {
            board: board,
            current_rank: 0,
            current_file: 0,
        }
    }
}

impl<'a> Iterator for PieceIter<'a> {
    type Item = PieceType;

    fn next(&mut self) -> Option<PieceType> {
        let res: Option<PieceType>;

        if self.current_rank >= RANK_COUNT {
            res = None;
        } else {
            let ty: PieceType = self.board.mb.get(self.current_file, self.current_rank);
            res = Some(ty);
        }

        if self.current_file + 1 >= FILE_COUNT {
            self.current_file = 0;
            self.current_rank += 1;
        } else {
            self.current_file += 1;
        }


        res
    }
}

pub fn assert_boards_equal(board1: &Board, board2: &Board) {
    assert!(board1.bb == board2.bb);
    assert_eq!(board1.to_move, board2.to_move);
    assert_eq!(board1.castling, board2.castling);
    assert_eq!(board1.en_passant, board2.en_passant);
    assert_eq!(board1.zhash, board2.zhash);
    assert!(board1.mb.eq(&board2.mb));
}

pub fn are_boards_equal(board1: &Board, board2: &Board) -> bool {
    board1.bb == board2.bb &&
    board1.to_move == board2.to_move &&
    board1.zhash == board2.zhash &&
    board1.castling == board2.castling &&
    board1.en_passant == board2.en_passant &&
    board1.mb.eq(&board2.mb)
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    use constants::*;

    mod piece_iter {
        #[allow(unused_imports)]
        use super::*;

        // #[test]        
        // fn from_fen_start_board() {
        //     let board = Board::from_fen(START_FEN);
        //     let subject = PieceIter::new(board);
        //     let pieces = subject.collect::<Vec<PieceType>>();

        //     assert_eq!(pieces., NO_EN_PASSANT);
        // }
        
        #[test]        
        fn from_fen_custom_board() {
            let board = Board::from_fen("r1bqkbnr/p2ppp2/2n3p1/Pp6/2PNP2p/8/1P3PPP/RNBQKB1R w KQkq b6 7 4");
               
            assert_eq!(board.en_passant, 1);
            assert_eq!(board.castling, 0b1111);
            assert_eq!(board.mb.get(3,3), W_KNIGHT);

            assert_eq!(board.mb.get(7,7), B_ROOK);
            assert_eq!(board.mb.get(4,3), W_PAWN);
            assert_eq!(board.to_move, WHITE);
            assert_eq!(board.halfmove_counter, 7);
            assert_eq!(board.fullmove_counter, 3);

        }
        
        #[test]
        fn test_print_board() {
            let board = Board::from_fen(START_FEN);
            board.print_board();
        }
        
        #[test]
        fn test_get_pieces_of_color() {
            let board = Board::from_fen(START_FEN);
            let white_pieces = board.get_pieces_of_color(WHITE);
            let black_pieces = board.get_pieces_of_color(BLACK);

            assert_eq!(white_pieces.len(), 16);
            assert_eq!(black_pieces.len(), 16);
            board.print_board();
        }
    }

    mod mailbox {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn get() {
            let mb: Mailbox = Mailbox([5; 128]);

            for f in 0..FILE_COUNT {
                for r in 0..RANK_COUNT {
                    assert_eq!(mb.get(f,r), 5);
                }
            }
        }

        #[test]
        fn set() {
            let mut mb: Mailbox = Mailbox([5; 128]);
            mb.set(0,0, 10);
            assert_eq!(mb.get(0,0), 10);
        }
    }

    #[test]
    fn parse_fen_pieces_default() {
        let pieces = parse_fen_pieces("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert_eq!(pieces.len(), 32);

        let s = piece_list_to_string(&pieces);
        assert_eq!(s, "[ra8, nb8, bc8, qd8, ke8, bf8, ng8, rh8, pa7, pb7, pc7, pd7, pe7, pf7, pg7, ph7, Pa2, Pb2, Pc2, Pd2, Pe2, Pf2, Pg2, Ph2, Ra1, Nb1, Bc1, Qd1, Ke1, Bf1, Ng1, Rh1]");
    }
}
