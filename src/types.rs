use std::fmt;
use constants::*;
use util::{is_white, file_to_char, char_to_file, piece_type_to_char, char_to_piece_type};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position(pub File, pub Rank);

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", file_to_char(self.0), self.1 + 1)
    }
}

impl Position {
    pub fn new(file: File, rank: Rank) -> Position {
        assert!(file >= 0 && file < FILE_COUNT && rank >= 0 && rank < RANK_COUNT);    
        Position(file, rank) 
    }

    pub fn from_pgn(s: &str) -> Position {
        let pgn = s.replace(" ", "");

        let chars = pgn.chars().collect::<Vec<char>>();
        assert!(chars.len() == 2);
        
        let mut piece_type: PieceType = NO_PIECE;
        let mut rank: Rank = (chars[1].to_digit(10).expect("invalid number") - 1) as Rank;
        let mut file_str = String::new();
        file_str.push(chars[0]);
        let mut file: File = char_to_file(&file_str);

        Position(file, rank)
    }

    pub fn from_pgn_list(s: &str) -> Vec<Position> {
        s.replace(" ", "").split(",").map(|pgn| {
            let chars = pgn.chars().collect::<Vec<char>>();
            assert!(chars.len() == 2);

            let mut file_str = String::new();
            file_str.push(chars[0]);

            let rank: Rank = (chars[1].to_digit(10).expect("invalid number") - 1) as Rank;

            Position(
                char_to_file(&file_str),
                rank,
            )
        }).collect::<Vec<Position>>()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct PiecePosition(pub PieceType, pub File, pub Rank);

impl PiecePosition {
    pub fn to_position(&self) -> Position {
        Position(self.1, self.2)
    }

    pub fn is_white(&self) -> bool {
        is_white(self.0)
    }


    pub fn piece_class(&self) -> PieceClass {
        (match self.0 {
            NO_PIECE => { NO_PIECE },
            p @ _ if !is_white(p) => { p - PIECE_TYPE_COLOR_OFFSET },
            _ => { self.0 }
        }) as PieceClass
    }
    
    pub fn from_pgn_list(s: &str) -> Vec<PiecePosition> {
        s.replace(" ", "").split(",").map(|pgn| {
            let chars = pgn.chars().collect::<Vec<char>>();
            assert!(chars.len() == 3);

            let mut file_str = String::new();
            file_str.push(chars[1]);

            let rank: Rank = (chars[2].to_digit(10).expect("invalid number") - 1) as Rank;

            PiecePosition(
                char_to_piece_type(&chars[0]),
                char_to_file(&file_str),
                rank,
            )
        }).collect::<Vec<PiecePosition>>()
    }

    pub fn from_pgn(s: &str) -> PiecePosition {
        let pgn = s.replace(" ", "");
        let chars = pgn.chars().collect::<Vec<char>>();
        assert!(chars.len() == 3 || chars.len() == 2);
        
        let mut piece_type: PieceType = NO_PIECE;
        let mut rank: Rank = 0;
        let mut file: File = 0;
        let mut file_str = String::new();

        if chars.len() == 3 {
            piece_type = char_to_piece_type(&chars[0]);

            file_str.push(chars[1]);
            file = char_to_file(&file_str);
            rank = (chars[2].to_digit(10).expect("invalid number") - 1) as Rank;
        } else if chars.len() == 2 {
            file_str.push(chars[0]);
            file = char_to_file(&file_str);
            rank = (chars[1].to_digit(10).expect("invalid number") - 1) as Rank;
        }

        PiecePosition(piece_type, file, rank)
    }
}

impl fmt::Debug for PiecePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 == NO_PIECE {
            write!(f, "{}{}", file_to_char(self.1), self.2 + 1)
        } else {
            write!(f, "{}{}{}", piece_type_to_char(self.0), file_to_char(self.1), self.2 + 1)
        }
    }
}

pub type PieceList = Vec<PiecePosition>;

mod tests {
    #[allow(unused_imports)]
    use super::*;
    
    mod position {
        #[allow(unused_imports)]
        use super::*;
        
        #[test]
        pub fn test_from_pgn_list() {
            let pieces = Position::from_pgn_list("a1,  b3, h7");
            assert_eq!(pieces, vec![
                Position(0, 0), 
                Position(1, 2), 
                Position(7, 6), 
            ]);
        }
        
        #[test]
        pub fn test_from_pgn() {
            assert_eq!(
                Position::from_pgn("a1"), 
                Position(0, 0)
            );
            
            assert_eq!(
                Position::from_pgn("c5"), 
                Position(2, 4)
            );
            
            assert_eq!(
                Position::from_pgn("e8"), 
                Position(4, 7)
            );
        }
    }
    
    mod piece_position {
        #[allow(unused_imports)]
        use super::*;
        
        #[test]
        pub fn test_from_pgn() {
            assert_eq!(
                PiecePosition::from_pgn("Ba1"), 
                PiecePosition(W_BISHOP, 0, 0)
            );
            
            assert_eq!(
                PiecePosition::from_pgn("c5"), 
                PiecePosition(NO_PIECE, 2, 4)
            );
            
            assert_eq!(
                PiecePosition::from_pgn("ke8"), 
                PiecePosition(B_KING, 4, 7)
            );
        }

        #[test]
        pub fn test_from_pgn_list() {
            let pieces = PiecePosition::from_pgn_list("Ba1,  bb3, Nh7");
            assert_eq!(pieces, vec![
                PiecePosition(W_BISHOP, 0, 0), 
                PiecePosition(B_BISHOP, 1, 2), 
                PiecePosition(W_KNIGHT, 7, 6), 
            ]);
        }
    }
}

