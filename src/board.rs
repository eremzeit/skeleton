use constants::*;
use bitboard::BitBoard;
use regex::Regex;
use zobrist;
use util::*;
use types::*;

#[derive(Copy)]
pub struct Mailbox(pub [PieceType; 128]);

impl Clone for Mailbox { fn clone(&self) -> Self { *self } }

// all valid indexes into the mailbox can be anded with this and should equal 0
pub const MAILBOX_INDEX_MASK: u8 = 0x88;

impl Mailbox {
    pub fn get(&self, file: u8, rank: u8) -> PieceType {
        self.0[((rank << 4) + file) as usize]
    }
    
    pub fn set(&mut self, file: u8, rank: u8, piece:PieceType) {
        self.0[((rank << 4) + file) as usize] = piece
    }

}

#[derive(Copy, Clone)]
pub struct Board {
    pub bb: BitBoard,
    pub mb: Mailbox,
    pub to_move: u8,
    pub zhash: u64,
    pub castling: u8,
    pub en_passant: u8,
    pub halfmove_counter: u8,
    pub fullmove_counter: u8,
}

impl Board {
    pub fn new() -> Board {
        let mb = Mailbox([NO_PIECE; 128]);
        let bb = BitBoard::create_from(&mb);

        let board = Board { 
            bb: bb,
            mb: mb,
            to_move: WHITE,
            zhash: 0,
            castling: 0,
            en_passant: 0,
            halfmove_counter: 0,
            fullmove_counter: 0
        };

        board
    }
    
    pub fn whites_turn(&self) -> bool {
        self.to_move == WHITE
    }
    
    pub fn blacks_turn(&self) -> bool {
        self.to_move == BLACK
    }

    //    A B C D E F G H

    // 8   - - - - - - - -
    // 7   - - - - - - - -
    // 6   - - - - - - - -
    // 5   - - - - - - - -
    // 4   - - - - - - - -
    // 3   - - - - - - - -
    // 2   - - - - - - - -
    // 1   - - - - - - - -
    
    pub fn print_board(&self) {
        let mut s: String = String::new();
        s.push_str("   A B C D E F G H\n");
        s.push_str("                  \n");
        
        for r in ranks_desc() {
            s.push_str(&format!("{}  ", r + 1));
            
            for f in files_asc() {
                let piece_type = self.mb.get(f, r);
                
                let piece_str: String = match piece_type {
                    NO_PIECE => { "-".to_string() },
                    _ => { format!("{}", piece_type_to_char(piece_type)) }
                };

                s.push_str(&format!("{} ", &piece_str));
            }

            s.push_str("\n");

            println!("{}", s);
        }

        println!("{}", s);
    }
    
    pub fn to_hash(&self) -> u64 {
        zobrist::get_board_hash(&self.get_pieces(), self.to_move, self.castling, self.en_passant)
    }
    
    pub fn get_piece_position(&self, file: File, rank: Rank) -> PiecePosition {
       PiecePosition(self.mb.get(file, rank), file, rank)
    }
    
    pub fn from_fen(fen: &str) -> Self {
        let mut board = Board::new();

        let groups = fen.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(groups.len(), 6);
        
        let pieces: PieceList = parse_fen_pieces(groups[0]);

        for piece_position in &pieces {
           board.mb.set(piece_position.1, piece_position.2, piece_position.0);
        }

        let turn: &str = groups[1];
        assert_eq!(turn.len(), 1);
        
        if turn.starts_with("w") {
            board.to_move = WHITE;
        } else if turn.starts_with("b") {
            board.to_move = BLACK;
        } else {
            assert!(false);
        }
 
        let castling_s: &str = groups[2];
     
        board.castling = 0;
        for c in castling_s.chars() {
            let v = match c {
                'K' => W_OO,
                'Q' => W_OOO,
                'k' => B_OO,
                'q' => B_OOO,
                _ => 0
            };

            board.castling = board.castling | v
        }

        let en_passent_str = groups[3];
        let number_match = Regex::new(r"[1-8]").unwrap();

        if number_match.is_match(en_passent_str) {
            let file_char = &number_match.replace(en_passent_str, "").into_owned();
            let ep_file = char_to_file(file_char);
            board.en_passant = ep_file;
        } else {
            board.en_passant = NO_EN_PASSANT;
        }

        board.halfmove_counter = groups[4].parse::<u8>().ok().unwrap_or(0);
        
        //fullmove counter in fen is an ordinal so it starts at 1
        board.fullmove_counter = groups[5].parse::<u8>().ok().unwrap_or(1) - 1;
        
        board.normalize();

        board
    }
    
    pub fn get_pieces(&self) -> PieceList {
        let mut pieces: PieceList = vec![];

        for f in 0..FILE_COUNT {
            for r in 0..RANK_COUNT {

                let piece_type = self.mb.get(f, r);
                if piece_type != NO_PIECE {
                    pieces.push(PiecePosition(piece_type, f, r)); 
                }
            }
        }

        pieces
    }
    
    fn normalize(&mut self) {
        self.bb = BitBoard::create_from(&self.mb);
        self.zhash = self.to_hash();
    }
    
    //fn _get_pieces(&self, piece_type: PieceType) -> PieceList {
    //    let result:PieceList = vec![]; 

    //    for f in 0..FILE_COUNT {
    //        for r in 0..RANK_COUNT {

    //            let _piece_type = self.mb.get(f, r);
    //            if _piece_type == piece_type {
    //                result.push(PiecePosition(piece_type, f, r)); 
    //            }
    //        }
    //    }

    //    result
    //}
    //
    //fn get_pawns(&self, color: Color) -> PieceList {
    //    self._get_pieces(if color == WHITE { W_PAWN } else { B_PAWN });
    //}
    //
    //fn get_bishops(&self, color: Color) -> PieceList {
    //    self._get_pieces(if color == WHITE { W_BISHOP } else { B_BISHOP })
    //}
}

 
fn parse_fen_pieces(piece_str: &str) -> PieceList {
    let ranks = piece_str.split('/').collect::<Vec<&str>>();

    let mut pieces: PieceList = vec![];

    println!("{}", piece_str);

    for (rank_pos, rank_str) in ranks.into_iter().enumerate() {
        println!("rank({}): {}", rank_pos, rank_str);
        let r: u32 = 7 - rank_pos as u32;
        let mut f: u32 = 0;
            
        for c in rank_str.chars() {
            //println!("char at {}: {}", f, c);
            if c.is_digit(10) {
                let digit = c.to_digit(10).expect("invalid number in fen piece list");
                f = f + digit;
            } else if c.is_alphanumeric() {
                let piece_position = PiecePosition(char_to_piece_type(&c), f as u8, r as u8);
                println!("new piece: {:?}", piece_position);
                pieces.push(piece_position);
                f = f + 1;
            } else {
                println!("Invalid character in piece string: {}", c);
            }
        }

    }

    pieces
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    mod board {
        #[allow(unused_imports)]
        use super::*;

        #[test]        
        fn from_fen_start_board() {
           let board = Board::from_fen(START_FEN);
           assert_eq!(board.en_passant, NO_EN_PASSANT);
           assert_eq!(board.castling, 0b1111);
           assert_eq!(board.to_move, WHITE);
           assert_eq!(board.halfmove_counter, 0);
           assert_eq!(board.fullmove_counter, 0); 
        }
        
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
    }

    mod mailbox {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn get() {
            let mb: Mailbox = Mailbox([5; 128]);
            assert_eq!(mb.get(0,0), 5);

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

