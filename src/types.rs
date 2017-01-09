use std::fmt;
use constants::*;
use util::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position(pub File, pub Rank);

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", file_to_char(self.0), self.1 + 1)
    }
}

impl Position {
    pub fn from_pgn(s: &str) -> Vec<Position> {
        s.replace(" ", "").split(",").map(|pgn| {
            let chars = pgn.chars().collect::<Vec<char>>();
            assert!(chars.len() == 2);

            let mut file_str = String::new();
            file_str.push(chars[0]);

            let rank: Rank = (chars[1].to_digit(10).expect("invalid number") - 1) as u8;

            Position(
                char_to_file(&file_str),
                rank,
            )
        }).collect::<Vec<Position>>()
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct PiecePosition(pub PieceType, pub File, pub Rank);

impl PiecePosition {
    pub fn to_position(&self) -> Position {
        Position(self.1, self.2)
    }

    pub fn piece_class(&self) -> PieceClass {
        (match self.0 {
            NO_PIECE => { NO_PIECE },
            p @ _ if !is_white(p) => { p - PIECE_TYPE_COLOR_OFFSET },
            _ => { self.0 }
        }) as PieceClass
    }

    pub fn from_pgn(s: &str) -> Vec<PiecePosition> {
        s.replace(" ", "").split(",").map(|pgn| {
            let chars = pgn.chars().collect::<Vec<char>>();
            assert!(chars.len() == 3);

            let mut file_str = String::new();
            file_str.push(chars[1]);

            let rank: Rank = (chars[2].to_digit(10).expect("invalid number") - 1) as u8;

            PiecePosition(
                char_to_piece_type(&chars[0]),
                char_to_file(&file_str),
                rank,
            )
        }).collect::<Vec<PiecePosition>>()
    }
}

impl fmt::Debug for PiecePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", piece_type_to_char(self.0), file_to_char(self.1), self.2 + 1)
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
        pub fn test_from_pgn() {
            let pieces = Position::from_pgn("a1,  b3, h7");
            assert_eq!(pieces, vec![
                Position(0, 0), 
                Position(1, 2), 
                Position(7, 6), 
            ]);
        }
    }
    
    mod piece_position {
        #[allow(unused_imports)]
        use super::*;
        
        #[test]
        pub fn test_from_pgn() {
            let pieces = PiecePosition::from_pgn("Ba1,  bb3, Nh7");
            assert_eq!(pieces, vec![
                PiecePosition(W_BISHOP, 0, 0), 
                PiecePosition(B_BISHOP, 1, 2), 
                PiecePosition(W_KNIGHT, 7, 6), 
            ]);
        }
    }
}

