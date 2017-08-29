use constants::*;
use bitboard::BitBoard;
use std::boxed::Box;
use regex::Regex;
use zobrist;
use util::*;
use types::*;

#[derive(Copy)]
pub struct Mailbox(pub [PieceType; 128]);

#[derive(Copy, Clone)]
pub struct Board {
    pub bb: BitBoard,
    pub mb: Mailbox,
    pub to_move: Color,
    pub zhash: u64,
    pub castling: u8,
    pub en_passant: File,

    // Gets set to zero any time there's a non-reversible move or capture.  Increments every time
    // a player moves.  If hits fifty, the game is drawn.
    pub halfmove_counter: u8,

    // Starts at 1.  Increments every time black moves.
    pub fullmove_counter: u8,
}


impl Clone for Mailbox { fn clone(&self) -> Self { *self } }

// all valid indexes into the mailbox can be anded with this and should equal 0
pub const MAILBOX_INDEX_MASK: u8 = 0x88;

impl Mailbox {
    pub fn empty() -> Mailbox {
        let mut mb = Mailbox([OFF_BOARD; 128]);

        for r in ranks_desc() {
            for f in files_asc() {
                mb.set(f, r, NO_PIECE);
            }
        }

        mb
    }
    
    pub fn getp(&self, pos: Position) -> PieceType {
        self.get(pos.0, pos.1)
    }
    
    pub fn setp(&mut self, pos: Position, piece: PieceType) {
        self.set(pos.0, pos.1, piece);
    }

    // TODO: this is slow.  if we want to be able to handle off-board queries
    // while still being performant, we should change the underlying data-structure.
    // We could also move away from using file and rank separately.  two seperate loops means
    // we're bounds checking more often than necessary.
    pub fn get(&self, file: File, rank: Rank) -> PieceType {
        assert!(file >= -2 && file < FILE_COUNT + 2);    
        assert!(rank >= -2 && rank < RANK_COUNT + 2);

        if file >= 0 && file < FILE_COUNT && rank >= 0 && rank < RANK_COUNT {
            let ind: u8 = ((rank as u8) << 4) + (file as u8);
            assert_eq!(ind & MAILBOX_INDEX_MASK, 0);
            return self.0[((rank << 4) + file) as usize]
        } else {
            OFF_BOARD     
        }
    }
    
    pub fn set(&mut self, file: File, rank: Rank, piece:PieceType) {
        self.0[((rank << 4) + file) as usize] = piece
    }

    pub fn move_piece(&mut self, orig_pos: Position, dest_pos: Position) {
        assert!(self.get(orig_pos.0, orig_pos.1) != NO_PIECE);

        let piece = self.get(orig_pos.0, orig_pos.1);
        self.set(dest_pos.0, dest_pos.1, piece);
        self.set(orig_pos.0, orig_pos.1, NO_PIECE);
    }

}

impl Board {
    pub fn new() -> Board {
        let mb = Mailbox::empty();

        let bb = BitBoard::create_from(&mb);

        let board = Board { 
            bb: bb,
            mb: mb,
            to_move: WHITE,
            zhash: 0,
            castling: CASTLING_DEFAULT,
            en_passant: NO_EN_PASSANT,
            halfmove_counter: 0,
            fullmove_counter: 1
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
            //println!("{}", s);
        }

        println!("{}", s);
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
    pub fn print_board_with_positions(&self, positions: &Vec<Position>) {

        let mut s: String = String::new();
        s.push_str("   A B C D E F G H\n");
        s.push_str("                  \n");
        
        for r in ranks_desc() {
            s.push_str(&format!("{}  ", r + 1));
            
            for f in files_asc() {
                let piece_type = self.mb.get(f, r);
                let is_target = positions.iter().any(|&p| (p) == Position(f,r));
                let mut piece_str: String;

                if is_target {
                    piece_str = match piece_type {
                        NO_PIECE => { "-".to_string() },
                        _ => { format!("{}", piece_type_to_char(piece_type)) }
                    };
                } else {
                  piece_str = "X".to_string();
                };
                    
                s.push_str(&format!("{} ", &piece_str));
            }

            s.push_str("\n");
            //println!("{}", s);
        }

        println!("{}", s);
    }
    
    pub fn to_hash(&self) -> u64 {
        zobrist::get_board_hash(&self.get_pieces(), self.to_move, self.castling, self.en_passant)
    }
    
    pub fn get_piece_position(&self, file: File, rank: Rank) -> PiecePosition {
       PiecePosition(self.mb.get(file, rank), file, rank)
    }
    

    // is this correct?
    pub fn get_piece_by_pgn(&self, pgn: &str) -> PiecePosition {
        let p = PiecePosition::from_pgn(pgn);
        PiecePosition(self.mb.get(p.1, p.2), p.1, p.2)
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
            let ep_file: File = char_to_file(file_char);
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
    
    pub fn get_pieces_iter(&self) -> Box<Iterator<Item=PiecePosition>> {
        let r: Box<Iterator<Item=PiecePosition>> = Box::new(self.get_pieces().into_iter());

        r
    }
    
    pub fn get_first_piece(&self, piece_type: PieceType) -> Option<PiecePosition> {
        for f in 0..FILE_COUNT {
            for r in 0..RANK_COUNT {
                let _piece_type = self.mb.get(f, r);
                
                if _piece_type == piece_type {
                   return Some(PiecePosition(piece_type, f, r)); 
                }
            }
        }

        None 
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
    
    pub fn normalize(&mut self) {
        self.bb = BitBoard::create_from(&self.mb);
        self.zhash = self.to_hash();
    }
    
}

 
fn parse_fen_pieces(piece_str: &str) -> PieceList {
    let ranks = piece_str.split('/').collect::<Vec<&str>>();

    let mut pieces: PieceList = vec![];

    for (rank_pos, rank_str) in ranks.into_iter().enumerate() {
        let r: Rank = (7 - rank_pos as Rank) as Rank;
        let mut f: File = 0;
            
        for c in rank_str.chars() {
            //println!("char at {}: {}", f, c);
            if c.is_digit(10) {
                let digit: File = c.to_digit(10).expect("invalid number in fen piece list") as File;
                f = f + digit;
            } else if c.is_alphanumeric() {
                let piece_position = PiecePosition(char_to_piece_type(&c), f as File, r as Rank);
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

