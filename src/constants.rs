use std::fmt;

pub type PieceType = u8;

pub type Rank = u8;
pub type File = u8;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position(pub File, pub Rank);

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", file_to_char(self.0), self.1 + 1)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct PiecePosition(pub PieceType, pub File, pub Rank);

impl PiecePosition {
    pub fn to_position(&self) -> Position {
        Position(self.1, self.2)
    }
}

impl fmt::Debug for PiecePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", piece_type_to_char(self.0), file_to_char(self.1), self.2 + 1)
    }
}

pub type PieceList = Vec<PiecePosition>;

pub type Color = u8;
pub const WHITE: Color = 0x00;
pub const BLACK: Color = 0x01;

pub fn is_same_color(to_move1: u8, to_move2: u8) -> bool {
    (to_move1 > 0) == (to_move2 > 0)
}

pub const PIECE_TYPE_COUNT: u8 = 12;

// These function both as offsets into the existince bitboard as 
// well as piece type values on the Mailbox board.  
pub const PAWN: u8 = 0x00;
pub const KNIGHT: u8 = 0x01;
pub const BISHOP: u8 = 0x02;
pub const ROOK: u8 = 0x03;
pub const QUEEN: u8 = 0x04;
pub const KING: u8 = 0x5;

pub const W_PAWN: u8 = 0x00;
pub const W_KNIGHT: u8 = 0x01;
pub const W_BISHOP: u8 = 0x02;
pub const W_ROOK: u8 = 0x03;
pub const W_QUEEN: u8 = 0x04;
pub const W_KING: u8 = 0x05;

pub const B_PAWN: u8 = 0x06;
pub const B_KNIGHT: u8 = 0x07;
pub const B_BISHOP: u8 = 0x08;
pub const B_ROOK: u8 = 0x09;
pub const B_QUEEN: u8 = 0x0a;
pub const B_KING: u8 = 0x0b;
pub const NO_PIECE: u8 = 0x0f;

pub const W_OOO: u8 = 0b0001;
pub const W_OO: u8 = 0b0010;
pub const B_OOO: u8 = 0b0100;
pub const B_OO: u8 = 0b1000;
pub const CASTLING_DEFAULT: u8 = 0b1111;

pub const NO_EN_PASSANT: u8 = 0b1111;

pub const RANK_COUNT: u8 = 8;
pub const FILE_COUNT: u8 = 8;

// Move meta constants
//pub const QUIET_MOVE: u8 = 0b0000;
//
// trying this constant out to avoid the default
// for the data type being a sane value
pub const QUIET_MOVE: u8 = 0b0111; 
pub const DOUBLE_PAWN_PUSH: u8 = 0b0001;
pub const KING_CASTLE: u8 = 0b0010;
pub const QUEEN_CASTLE: u8 = 0b0011;
pub const CAPTURE: u8 = 0b0100;
pub const EP_CAPTURE: u8 = 0b0101;
pub const KNIGHT_PROMOTION: u8 = 0b1000;
pub const BISHOP_PROMOTION: u8 = 0b1001;
pub const ROOK_PROMOTION: u8 = 0b1010;
pub const QUEEN_PROMOTION: u8 = 0b1011;
pub const KNIGHT_PROMO_CAPTURE: u8 = 0b1100;
pub const BISHOP_PROMO_CAPTURE: u8 = 0b1101;
pub const ROOK_PROMO_CAPTURE: u8 = 0b1110;
pub const QUEEN_PROMO_CAPTURE: u8 = 0b1111;



pub fn char_to_piece_type(fen_char: &char) -> u8{
    match fen_char {
        &'P' => W_PAWN,
        &'N' => W_KNIGHT,
        &'B' => W_BISHOP,
        &'R' => W_ROOK,
        &'Q' => W_QUEEN,
        &'K' => W_KING,
        &'p' => B_PAWN,
        &'n' => B_KNIGHT,
        &'b' => B_BISHOP,
        &'r' => B_ROOK,
        &'q' => B_QUEEN,
        &'k' => B_KING,
        _ => 0
    }
}

pub fn piece_type_to_char(piece_type: PieceType) -> char {
    match piece_type {
        W_PAWN => 'P',
        W_KNIGHT => 'N',
        W_BISHOP => 'B',
        W_ROOK => 'R',
        W_QUEEN => 'Q',
        W_KING => 'K',
        B_PAWN => 'p',
        B_KNIGHT => 'n',
        B_BISHOP => 'b',
        B_ROOK => 'r',
        B_QUEEN => 'q',
        B_KING => 'k',
        _ => '-'
    }
}

pub fn file_to_char(file_offset: u8) -> &'static str {
    let c: &'static str = match file_offset {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => "-"
    };

    c
}

pub fn char_to_file(file_char: &str) -> File {
    let f: File = match file_char {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        "e" => 4,
        "f" => 5,
        "g" => 6,
        "h" => 7,
        _ => NO_EN_PASSANT
    };

    f
}

pub fn piece_position_to_str(piece_position: &PiecePosition) -> String {
    let p_char = piece_type_to_char(piece_position.0);
    let file_char = file_to_char(piece_position.1);
    let rank = piece_position.2 + 1;
    format!("{}{}{}", p_char, file_char, rank)  
}

pub fn piece_list_to_string(piece_list: &PieceList) -> String {
    let mut s = String::new();

    for (i, piece_position) in piece_list.iter().enumerate() {
        let piece_str = piece_position_to_str(piece_position);
        if i == piece_list.len() - 1 {
            s.push_str(&format!("{}", piece_str));
        } else {
            s.push_str(&format!("{}, ", piece_str));
        }
    }

    format!("[{}]", s)
}

pub const START_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const INTERESTING_FEN: &'static str = "r2qk2r/pp3pbp/3p1np1/8/2PBPQb1/8/PP4PP/RN2KB1R b KQkq - 0 12";


pub struct FileIndexIterator {
   next_file: i8,
   desc: bool,
}

impl FileIndexIterator {
    fn files() -> FileIndexIterator {
        FileIndexIterator { next_file: 0, desc: false }
    }
    
    fn files_desc() -> FileIndexIterator {
        FileIndexIterator { next_file: (RANK_COUNT - 1) as i8, desc: true }
    }
}

impl Iterator for FileIndexIterator {
    type Item = File;

    fn next(&mut self) -> Option<File> {
        let res: Option<File>;

        if self.next_file < FILE_COUNT as i8 && self.next_file >= 0 {
            res = Some(self.next_file as File)
        } else {
            res = None
        }

        self.next_file = if self.desc { self.next_file - 1 } else { self.next_file + 1 };

        res
    }
}

pub fn files_asc() -> FileIndexIterator {
    FileIndexIterator::files()
}

pub fn files_desc() -> FileIndexIterator {
    FileIndexIterator::files_desc()
}

pub struct RankIndexIterator {
   next_rank: i8,
   desc: bool,
}

impl RankIndexIterator {
    fn ranks() -> RankIndexIterator {
        RankIndexIterator { next_rank: 0, desc: false }
    }
    
    fn ranks_desc() -> RankIndexIterator {
        RankIndexIterator { next_rank: (RANK_COUNT - 1) as i8, desc: true }
    }
}

impl Iterator for RankIndexIterator {
    type Item = Rank;

    fn next(&mut self) -> Option<Rank> {
        let res: Option<Rank>;

        if self.next_rank < RANK_COUNT as i8 && self.next_rank >= 0 {
            res = Some(self.next_rank as Rank)
        } else {
            res = None
        }

        self.next_rank = if self.desc { self.next_rank - 1 } else { self.next_rank + 1 };

        res
    }
}

pub fn ranks_asc() -> RankIndexIterator {
    RankIndexIterator::ranks()
}

pub fn ranks_desc() -> RankIndexIterator {
    RankIndexIterator::ranks_desc()
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn piece_position_to_str_base() {
        let s = piece_position_to_str(&PiecePosition(B_BISHOP, 3, 0));
        assert_eq!(s, "bd1");
    }
    
    #[test]
    fn piece_list_to_string_base() {
        let list: Vec<PiecePosition> = vec![PiecePosition(W_QUEEN, 1, 7), PiecePosition(B_PAWN, 5, 0)];
        let s = piece_list_to_string(&list);
        assert_eq!(s, "[Qb8, pf1]");
    }
    
    #[test]
    fn test_ranks_iterator() {
        assert_eq!(
            ranks_asc().collect::<Vec<Rank>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7]
        );
        
        assert_eq!(
            ranks_desc().collect::<Vec<Rank>>(),
            vec![7, 6, 5, 4, 3, 2, 1, 0]
        );
    }
    
    #[test]
    fn test_files_iterator() {
        assert_eq!(
            files_asc().collect::<Vec<Rank>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7]
        );
        
        assert_eq!(
            files_desc().collect::<Vec<Rank>>(),
            vec![7, 6, 5, 4, 3, 2, 1, 0]
        );
    }
}
