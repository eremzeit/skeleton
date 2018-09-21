
use self::types::*;
use types::*;
use util::*;
use constants::*;
use board::Board;
use std::iter;
use std::cmp;
use super::*;

pub fn generate_castling_moves(piece: PiecePosition, board: &Board) -> MovesIter {
    let mut moves: Vec<Move> = vec![];

    if piece.is_white() {
        if board.castling & W_OO > 0
                && board.mb.get(KING_SIDE_CASTLE_FILE - 1, WHITE_BACK_RANK) == NO_PIECE
                && board.mb.get(KING_SIDE_CASTLE_FILE, WHITE_BACK_RANK) == NO_PIECE
                && !is_pos_attacked_by(board, Position(4, WHITE_BACK_RANK), BLACK)
                && !is_pos_attacked_by(board, Position(5, WHITE_BACK_RANK), BLACK) 
                && !is_pos_attacked_by(board, Position(6, WHITE_BACK_RANK), BLACK) {

            moves.push(Move {
                origin_piece: piece.0,
                origin_pos: piece.to_position(),
                dest_piece: NO_PIECE,
                dest_pos: Position(KING_SIDE_CASTLE_FILE, WHITE_BACK_RANK),
                meta_info: KING_CASTLE 
            });
        }
        
        if board.castling & W_OOO > 0
                && board.mb.get(QUEEN_SIDE_CASTLE_FILE + 1, WHITE_BACK_RANK) == NO_PIECE
                && board.mb.get(QUEEN_SIDE_CASTLE_FILE, WHITE_BACK_RANK) == NO_PIECE
                && board.mb.get(QUEEN_SIDE_CASTLE_FILE - 1, BLACK_BACK_RANK) == NO_PIECE
                && !is_pos_attacked_by(board, Position(QUEEN_SIDE_CASTLE_FILE, WHITE_BACK_RANK), BLACK) 
                && !is_pos_attacked_by(board, Position(QUEEN_SIDE_CASTLE_FILE + 1, WHITE_BACK_RANK), BLACK)
                && !is_pos_attacked_by(board, Position(QUEEN_SIDE_CASTLE_FILE + 2, WHITE_BACK_RANK), BLACK) {

            moves.push(Move {
                origin_piece: piece.0,
                origin_pos: piece.to_position(),
                dest_piece: NO_PIECE,
                dest_pos: Position(QUEEN_SIDE_CASTLE_FILE, WHITE_BACK_RANK),
                meta_info: QUEEN_CASTLE 
            });
        }
        
    } else {
        if board.castling & B_OO > 0
                && board.mb.get(KING_SIDE_CASTLE_FILE - 1, BLACK_BACK_RANK) == NO_PIECE
                && board.mb.get(KING_SIDE_CASTLE_FILE, BLACK_BACK_RANK) == NO_PIECE
                && !is_pos_attacked_by(board, Position(KING_SIDE_CASTLE_FILE - 2, BLACK_BACK_RANK), WHITE)
                && !is_pos_attacked_by(board, Position(KING_SIDE_CASTLE_FILE - 1, BLACK_BACK_RANK), WHITE) 
                && !is_pos_attacked_by(board, Position(KING_SIDE_CASTLE_FILE, BLACK_BACK_RANK), WHITE) {

            moves.push(Move {
                origin_piece: piece.0,
                origin_pos: piece.to_position(),
                dest_piece: NO_PIECE,
                dest_pos: Position(KING_SIDE_CASTLE_FILE, BLACK_BACK_RANK),
                meta_info: KING_CASTLE 
            });
        }
        
        if board.castling & B_OOO > 0
                && board.mb.get(QUEEN_SIDE_CASTLE_FILE - 1, BLACK_BACK_RANK) == NO_PIECE
                && board.mb.get(QUEEN_SIDE_CASTLE_FILE, BLACK_BACK_RANK) == NO_PIECE 
                && board.mb.get(QUEEN_SIDE_CASTLE_FILE + 1, BLACK_BACK_RANK) == NO_PIECE
                && !is_pos_attacked_by(board, Position(QUEEN_SIDE_CASTLE_FILE, BLACK_BACK_RANK), WHITE) 
                && !is_pos_attacked_by(board, Position(QUEEN_SIDE_CASTLE_FILE + 1, BLACK_BACK_RANK), WHITE)
                && !is_pos_attacked_by(board, Position(QUEEN_SIDE_CASTLE_FILE + 2, BLACK_BACK_RANK), WHITE) {
            
            // the dest_pos is where the king ends up.
            moves.push(Move {
                origin_piece: piece.0,
                origin_pos: piece.to_position(),
                dest_piece: NO_PIECE,
                dest_pos: Position(QUEEN_SIDE_CASTLE_FILE, BLACK_BACK_RANK),
                meta_info: QUEEN_CASTLE 
            });
        }
    }

    MovesIter::from_vec(moves)
}

mod tests {
    const BOTH_CASTLE_KINGSIDE: &'static str = "rnbqk2r/ppppp2p/5n1b/5pp1/3P1P2/5NP1/PPP1P1BP/RNBQK2R b KQkq - 9 5";
    const BOTH_CASTLE_KINGSIDE_BLOCKED: &'static str = "r1bqk2r/pp1p3p/2p1pn1b/3N1pp1/1B3P2/5NP1/PPP1n1BP/R2QK2R b KQkq - 19 10";

    const BOTH_CASTLE_QUEENSIDE: &'static str = "r3kbnr/pppqpppp/2n1b3/3p4/3P4/2N1B3/PPPQPPPP/R3KBNR w KQkq - 8 5";
    const BOTH_CASTLE_QUEENSIDE_BLOCKED: &'static str = "r3kbnr/pppq1ppp/2n1p3/3p2B1/3P2b1/2N1P3/PPPQ1PPP/R3KBNR w KQkq - 12 7";

    #[allow(unused_imports)]
    use super::*;

    #[allow(non_snake_case)]
    #[test]        
    fn test_castling_starting_board() {
        let board = Board::from_fen(START_FEN);
        let piece = board.get_piece_by_pgn("e1");
        let moves = generate_castling_moves(piece, &board).collect::<Vec<_>>();
        println!("moves: {:?}", moves);
        assert_eq!(moves.len(), 0);
    }
    
    #[allow(non_snake_case)]
    #[test]        
    fn test_castling_kingside_white() {
        let board = Board::from_fen(BOTH_CASTLE_KINGSIDE);
        
        let piece = board.get_piece_by_pgn("e1");
        let moves = generate_castling_moves(piece, &board);
        let diff = move_list_diff(&moves.collect::<Vec<_>>(), &vec![
            Move {
                origin_piece: W_KING,
                dest_piece: NO_PIECE,
                origin_pos: Position(4, WHITE_BACK_RANK),
                dest_pos: Position(6, WHITE_BACK_RANK),
                meta_info: KING_CASTLE
            }
        ]);

        assert!(diff.len() == 0);
    }
    
    #[test]        
    fn test_castling_kingside_black() {
        let board = Board::from_fen(BOTH_CASTLE_KINGSIDE);
        
        let piece = board.get_piece_by_pgn("e8");
        let moves = generate_castling_moves(piece, &board);

        let diff = move_list_diff(&moves.collect::<Vec<_>>(), &vec![
            Move {
                origin_piece: B_KING,
                dest_piece: NO_PIECE,
                origin_pos: Position(4, BLACK_BACK_RANK),
                dest_pos: Position(6, BLACK_BACK_RANK),
                meta_info: KING_CASTLE
            }
        ]);

        assert!(diff.len() == 0);
    }
 
    #[test]        
    fn test_castling_queenside_white() {
        let board = Board::from_fen(BOTH_CASTLE_QUEENSIDE);
        
        let piece = board.get_piece_by_pgn("e1");
        let moves = generate_castling_moves(piece, &board);
        let diff = move_list_diff(&moves.collect::<Vec<_>>(), &vec![
            Move {
                origin_piece: W_KING,
                dest_piece: NO_PIECE,
                origin_pos: Position(4, WHITE_BACK_RANK),
                dest_pos: Position(2, WHITE_BACK_RANK),
                meta_info: QUEEN_CASTLE
            }
        ]);

        assert!(diff.len() == 0);
    }
    
    #[test]        
    fn test_castling_queenside_black() {
        let board = Board::from_fen(BOTH_CASTLE_QUEENSIDE);
        
        let piece = board.get_piece_by_pgn("e8");
        let moves = generate_castling_moves(piece, &board);
        assert!(move_list_diff(&moves.collect::<Vec<_>>(), &vec![
            Move {
                origin_piece: B_KING,
                dest_piece: NO_PIECE,
                origin_pos: Position(4, BLACK_BACK_RANK),
                dest_pos: Position(2, BLACK_BACK_RANK),
                meta_info: QUEEN_CASTLE
            }
        ]).len() == 0);
    }
    
    #[test]        
    fn test_castling_kingside_blocked() {
        let board = Board::from_fen(BOTH_CASTLE_KINGSIDE_BLOCKED);
        
        let piece = board.get_piece_by_pgn("e1");
        let moves = generate_castling_moves(piece, &board);
        assert!(moves.collect::<Vec<_>>().len() == 0);
    }
    
    #[test]
    fn test_castling_queenside_blocked() {
        let board = Board::from_fen(BOTH_CASTLE_KINGSIDE_BLOCKED);
        
        let piece = board.get_piece_by_pgn("e8");
        let moves = generate_castling_moves(piece, &board);
        assert!(moves.collect::<Vec<_>>().len() == 0);
    }
}
