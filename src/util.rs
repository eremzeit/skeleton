use constants::*;
use types::*;
use std::collections::hash_map::RandomState;
use std::collections;

pub fn is_white(piece: PieceType) -> bool {
    piece >= W_PAWN && piece <= W_KING
}

pub fn color_of(piece: PieceType) -> Color {
    if is_white(piece) { WHITE } else { BLACK }
}

pub fn opposite_color(color: Color) -> Color {
    1 - color
}

pub fn to_white(piece: PieceType) -> PieceType {
    if piece >= W_PAWN && piece <= W_KING {
        piece 
    } else {
        opposite_color_piece_type(piece)
    }
}

pub fn opposite_color_piece_type(piece_type: PieceType) -> PieceType {
    if piece_type >= W_PAWN && piece_type <= W_KING {
        piece_type + PIECE_TYPE_COLOR_OFFSET
    } else if piece_type >= B_PAWN && piece_type <= B_KING {
        piece_type - PIECE_TYPE_COLOR_OFFSET
    } else {
        NO_PIECE
    }
}

pub fn is_occupied_and_enemy(piece1: PieceType, piece2: PieceType) -> bool {
    piece1 != NO_PIECE && piece2 != NO_PIECE && !is_same_color(piece1, piece2)
}

pub fn is_same_color(piece1: PieceType, piece2: PieceType) -> bool {
    (piece1 >= W_PAWN && piece1 <= W_KING) && (piece2 >= W_PAWN && piece2 <= W_KING)
        || (piece1 >= B_PAWN && piece1 <= B_KING) && (piece2 >= B_PAWN && piece2 <= B_KING)
}

pub fn assert_position_list_eq(positions1: &Vec<Position>, positions2: &Vec<Position>) {
    if cfg!(debug_assertions) {
        let diff = position_list_diff(positions1, positions2);
        if diff.len() != 0 {
            println!("Position list diff: {:?}", diff);
            assert!(false); 
        }
    }
}

pub fn position_list_diff(positions1: &Vec<Position>, positions2: &Vec<Position>) -> Vec<Position> {
    let set1 = positions1.iter().map(|p| { *p }).collect::<collections::HashSet<Position, RandomState>>();
    let set2 = positions2.iter().map(|p| { *p }).collect::<collections::HashSet<Position, RandomState>>();

    println!("set1: {:?}", set1);
    println!("set2: {:?}", set2);
    set1.symmetric_difference(&set2).map(|p| { *p }).collect::<Vec<Position>>()
}

pub fn are_positions_eq(positions1: &Vec<Position>, positions2: &Vec<Position>) -> bool {
    position_list_diff(positions1, positions2).len() == 0
}

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

//TODO: could this instead return a PieceType
pub fn char_to_piece_type(fen_char: &char) -> u8 {
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

pub fn piece_type_to_str(piece_type: PieceType) -> String {
    let mut s = String::new();
    s.push(piece_type_to_char(piece_type));
    s
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
        NO_PIECE => ' ',
        _ => '?'
    }
}

pub fn file_to_char(file_offset: File) -> &'static str {
    //assert!(file_offset >= 0 && file_offset < FILE_COUNT);

    let c: &'static str = match file_offset {
        -1 => "z",
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        8 => "i",
        _ => "x"
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
    format!("{:?}", piece_position)
}

pub fn piece_list_to_string(piece_list: &PieceList) -> String {
    let mut s = String::new();

    for (i, piece_position) in piece_list.iter().enumerate() {
        if i == piece_list.len() - 1 {
            s.push_str(&format!("{:?}", piece_position));
        } else {
            s.push_str(&format!("{:?}, ", piece_position));
        }
    }

    format!("[{}]", s)
}


mod tests {
    #[allow(unused_imports)]
    use super::*;

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
    fn test_are_positions_eq() {
        assert!(are_positions_eq(
            &vec![Position(0,1), Position(2,5), Position(2,1)],
            &vec![Position(2,5), Position(2,1), Position(0,1)]
        ));
        
        assert!(!are_positions_eq(
            &vec![Position(0,1), Position(2,5), Position(2,1)],
            &vec![Position(2,1), Position(0,1)]
        ));
    }
    
    #[test]
    fn test_opposite_color_piece_type() {
        assert_eq!(opposite_color_piece_type(W_BISHOP), B_BISHOP);
        assert_eq!(opposite_color_piece_type(B_KING), W_KING);
        assert_eq!(opposite_color_piece_type(NO_PIECE), NO_PIECE);
    }
}

