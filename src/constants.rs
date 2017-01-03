pub type PieceType = u8;

pub type Rank = u8;
pub type File = u8;

#[derive(Copy, Clone, Debug)]
pub struct PiecePosition(pub PieceType, pub File, pub Rank);

pub type PieceList = Vec<PiecePosition>;

pub const WHITE: u8 = 0x00;
pub const BLACK: u8 = 0x01;

pub const PIECE_TYPE_COUNT: u8 = 12;

/// Offsets into the existince bitboard
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

pub const EMPTY: u8 = 0;
pub const OCCUPIED: u8 = 1;

pub const W_OOO: u8 = 0b0001;
pub const W_OO: u8 = 0b0010;
pub const B_OOO: u8 = 0b0100;
pub const B_OO: u8 = 0b1000;
pub const CASTLING_DEFAULT: u8 = 0b1111;

pub const NO_EN_PASSANT: u8 = 0b1111;

pub const RANK_COUNT: u8 = 8;
pub const FILE_COUNT: u8 = 8;

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
}
