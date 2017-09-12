
pub type Color = u8;
pub const WHITE: Color = 0x00;
pub const BLACK: Color = 0x01;

pub const PIECE_TYPE_COUNT: u8 = 12;
pub const PIECE_TYPE_COLOR_OFFSET: u8 = 6;

// These function both as offsets into the existince bitboard as 
// well as piece type values on the Mailbox board.  
pub type PieceClass = u8;
pub const PAWN: u8 = 0x00;
pub const KNIGHT: u8 = 0x01;
pub const BISHOP: u8 = 0x02;
pub const ROOK: u8 = 0x03;
pub const QUEEN: u8 = 0x04;
pub const KING: u8 = 0x5;

pub type PieceType = u8;
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
pub const OFF_BOARD: u8 = 0x0e;

pub const W_OOO: u8 = 0b0001;
pub const W_OO: u8 = 0b0010;
pub const B_OOO: u8 = 0b0100;
pub const B_OO: u8 = 0b1000;
pub const CASTLING_DEFAULT: u8 = 0b1111;

pub const NO_EN_PASSANT: File = 0b1111;

pub type Rank = i8;
pub const RANK_COUNT: i8 = 8;

pub type File = i8;
pub const FILE_COUNT: i8 = 8;

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

pub const WHITE_Y_DIR_SIGN: i8 = 1;
pub const WHITE_PAWN_STARTING_RANK: i8 = 1;
pub const WHITE_SINGLE_PUSH_RANK: i8 = 2;
pub const WHITE_DOUBLE_PUSH_RANK: i8 = 3;
pub const WHITE_BACK_RANK: i8 = 0;

// this is the rank that a white pawn needs to be on in order for it
// to be eligable for an EP capture.
pub const WHITE_EP_CAP_RANK: i8 = 4;
    
pub const BLACK_Y_DIR_SIGN: i8 = -1;
pub const BLACK_PAWN_STARTING_RANK: i8 = 6;
pub const BLACK_SINGLE_PUSH_RANK: i8 = 5;
pub const BLACK_DOUBLE_PUSH_RANK: i8 = 4;
pub const BLACK_BACK_RANK: i8 = 7;
pub const BLACK_EP_CAP_RANK: i8 = 3;

// ie. the file that the king will end up on
pub const QUEEN_SIDE_CASTLE_FILE: File = 2;
pub const KING_SIDE_CASTLE_FILE: File = 6;

pub const START_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const INTERESTING_FEN: &'static str = "r2qk2r/pp3pbp/3p1np1/8/2PBPQb1/8/PP4PP/RN2KB1R b KQkq - 0 12";

pub const R1: Rank = 0;
pub const R2: Rank = 1;
pub const R3: Rank = 2;
pub const R4: Rank = 3;
pub const R5: Rank = 4;
pub const R6: Rank = 5;
pub const R7: Rank = 6;
pub const R8: Rank = 7;

pub const F1: File = 0;
pub const F2: File = 1;
pub const F3: File = 2;
pub const F4: File = 3;
pub const F5: File = 4;
pub const F6: File = 5;
pub const F7: File = 6;
pub const F8: File = 7;
