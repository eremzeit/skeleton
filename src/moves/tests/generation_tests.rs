const TEST_FEN1: &'static str = "r1bk1b1K/pp2p1p1/N1p1Pq1B/2B1rp2/Rn1P1PQP/1p1n1R2/P1P1P1P1/1N6 w - - 6 1";
const BISHOP_TEST_FEN: &'static str = "7k/8/8/1p3P2/8/3B4/8/K7 w - - 0 1";
const ROOK_TEST_FEN: &'static str = "7k/8/8/1p3P2/8/8/5r1p/K7 b - - 0 1";
const QUEEN_TEST_FEN: &'static str = "7k/8/8/1p3P2/8/8/5Q1p/K7 w - - 0 1";
const KING_TEST_FEN: &'static str = "7k/8/8/1p3P2/8/1Pp1r3/2K1b2p/8 w - - 0 1";
const KING_CHECKED_TEST_FEN: &'static str = "7k/8/8/1p3P2/8/1Pp5/2K1r2p/8 w - - 0 1";
const KNIGHT_TEST_FEN: &'static str = "7k/8/8/3b1P2/8/4n3/8/K7 b - - 0 1";
const PAWN_TEST_FEN: &'static str = "7k/8/8/1p3P2/1bn5/1Pp5/2K1r2p/8 w - - 0 1";

const ONLY_VALID_MOVE_KC5: &'static str = "8/8/7r/3K4/7r/8/7k/4r3 w - - 0 1";
const ONLY_VALID_MOVE_NXC5: &'static str = "5b2/3N4/7r/2rK4/7r/8/7k/4r3 w - - 0 1";

use constants::*;
use types::Position;
use types::PiecePosition;
use super::generation::*;
use super::types::*;
use util::*;
use board::Board;

#[test]
fn with_constrained_king_movement() {
    // only valid move is kc8 
    let board = Board::from_fen("1k6/8/1K6/8/8/8/8/R7 b - - 0 1"); 
    let moves = generate_moves_for_piece(PiecePosition::from_pgn("kb8"), &board);
    assert!(moves.len() == 1);
    assert_eq!(moves[0].dest_pos, Position::from_pgn("c8"));
}

#[test]
fn with_constrained_king_movement2() {
    let board = Board::from_fen(ONLY_VALID_MOVE_KC5);
    let moves = generate_moves_for_piece(PiecePosition::from_pgn("Kd5"), &board);
    assert!(moves.len() == 1);
    assert_eq!(moves[0].dest_pos, Position::from_pgn("c5"));
}

#[test]
fn forced_capture() {
    let board = Board::from_fen(ONLY_VALID_MOVE_NXC5);
    let moves = generate_moves_for_piece(PiecePosition::from_pgn("Nd7"), &board);
    assert!(moves.len() == 1);
    assert_eq!(moves[0].origin_piece, W_KNIGHT);
    assert_eq!(moves[0].dest_pos, Position::from_pgn("c5"));
}

#[test]        
fn test_generate_bishop_moves() {
    let board = Board::from_fen(BISHOP_TEST_FEN);
    board.print_board();
    assert_eq!(board.to_move, WHITE);
    
    let piece = board.get_piece_position(3, 2);
    assert_eq!(piece.0, W_BISHOP);
    let mut moves = generate_bishop_moves(piece, &board, false).collect::<Vec<Move>>();
    
    assert_eq!(moves.iter().all(|m| { m.origin_pos == piece.to_position() }), true);
    assert_eq!(moves.iter().all(|m| { m.origin_piece == W_BISHOP}), true); 
    assert_eq!(moves.iter().all(|m| { m.dest_piece == NO_PIECE || m.dest_piece >= B_PAWN  && m.dest_piece <= B_KING}), true); 
    
    let positions = moves.into_iter().map(|m| { 
        println!("dest_pos: {:?}", m.dest_pos);
        m.dest_pos 
    }).collect::<Vec<Position>>();
    
    let correct = Position::from_pgn_list("b1, c2, e4, f1, e2, c4, b5");
    
    assert!(are_positions_eq(&correct, &positions));
}

#[test]        
fn test_generate_rooks_moves() {
    let board = Board::from_fen(ROOK_TEST_FEN);
    board.print_board();
    
    let piece = board.get_piece_position(5, 1);
    let mut moves = generate_rook_moves(piece, &board, false).collect::<Vec<Move>>();

    assert!(moves.iter().all(|m| { m.origin_pos == piece.to_position() }));
    assert!(moves.iter().all(|m| { m.origin_piece == B_ROOK})); 
    assert!(moves.iter().all(|m| { m.dest_piece == NO_PIECE || !is_same_color(piece.0, m.dest_piece) })); 
    
    let positions = moves.into_iter().map(|m| m.dest_pos).collect::<Vec<Position>>();
    let correct = Position::from_pgn_list("a2, b2, c2, d2, e2, g2, f1, f3, f4, f5");

    assert!(positions.len() > 0);
    assert!(are_positions_eq(&correct, &positions));
}

#[test]
fn test_generate_queen_moves() {
    let board = Board::from_fen(QUEEN_TEST_FEN);
    board.print_board();

    let piece = board.get_piece_position(5, 1);
    let mut moves = generate_queen_moves(piece, &board, false);
    let positions = moves.map(|m| { m.dest_pos }).collect::<Vec<Position>>();
    let correct = Position::from_pgn_list("a2, b2, c2, d2, e2, g2, h2, f1, f3, f4, e3, d4, c5, b6, a7, e1, g1, g3, h4");
    assert!(are_positions_eq(&positions, &correct));
}

#[test]
fn test_generate_pawn_moves() {
    let board = Board::from_fen("r2qk2r/p5bp/3p2p1/1p2Pp1n/2PB1Qb1/7P/PP4P1/RN2KB1R w KQkq f6 5 3");
    board.print_board();

    // there's an EP from e5-f6
    let pawn = board.get_piece_by_pgn("e5"); 
    let moves = generate_pawn_moves(pawn, &board, true, false);

    let diff = move_list_diff(&moves.collect::<Vec<_>>(), &vec![
        Move {
           origin_piece: W_PAWN,
           dest_piece: NO_PIECE,
           origin_pos: Position(4,4),
           dest_pos: Position(5,5),
           meta_info: EP_CAPTURE
        },

        Move {
           origin_piece: W_PAWN,
           dest_piece: B_PAWN,
           origin_pos: Position(4,4),
           dest_pos: Position(3,5),
           meta_info: CAPTURE
        },
        
        Move {
           origin_piece: W_PAWN,
           dest_piece: NO_PIECE,
           origin_pos: Position(4,4),
           dest_pos: Position(4,5),
           meta_info: QUIET_MOVE
        }
    ]);

    assert!(diff.len() == 0);
}

#[test]
fn test_generate_pawn_moves_double_push() {
    let board = Board::from_fen("r2qk2r/p5bp/3p2p1/1p2Pp1n/2PB1Qb1/7P/PP4P1/RN2KB1R w KQkq f6 5 3");
    board.print_board();

    // there's an EP from e5-f6
    let pawn = board.get_piece_by_pgn("a2"); 
    let moves = generate_pawn_moves(pawn, &board, true, false);

    let diff = move_list_diff(&moves.collect::<Vec<_>>(), &vec![
        Move {
           origin_piece: W_PAWN,
           dest_piece: NO_PIECE,
           origin_pos: Position(0,1),
           dest_pos: Position(0,3),
           meta_info: DOUBLE_PAWN_PUSH
        },
        
        Move {
           origin_piece: W_PAWN,
           dest_piece: NO_PIECE,
           origin_pos: Position(0,1),
           dest_pos: Position(0,2),
           meta_info: QUIET_MOVE
        }
    ]);

    assert!(diff.len() == 0);
}

#[test]
fn test_generate_pawn_moves_black() {
    let board = Board::from_fen("r2qk2r/p5bp/3p2p1/1p2Pp1n/2PB1Qb1/7P/PP4P1/RN2KB1R w KQkq f6 5 3");
    board.print_board();

    // there's an EP from e5-f6
    let pawn = board.get_piece_by_pgn("h7"); 
    let moves = generate_pawn_moves(pawn, &board, true, false);

    let diff = move_list_diff(&moves.collect::<Vec<_>>(), &vec![
        Move {
           origin_piece: B_PAWN,
           dest_piece: NO_PIECE,
           origin_pos: Position(7,6),
           dest_pos: Position(7,5),
           meta_info: QUIET_MOVE
        }
    ]);

    assert!(diff.len() == 0);
}

#[test]
fn test_generate_pawn_moves_attacks_only() {
    let board = Board::from_fen("r2qk2r/p5bp/3p2p1/1p2Pp1n/2PB1Qb1/7P/PP4P1/RN2KB1R w KQkq f6 5 3");
    board.print_board();

    // there's an EP from e5-f6
    let pawn = board.get_piece_by_pgn("d6"); 
    let moves = generate_pawn_moves(pawn, &board, false, true);

    let diff = move_list_diff(&moves.collect::<Vec<_>>(), &vec![
        Move {
           origin_piece: B_PAWN,
           dest_piece: W_PAWN,
           origin_pos: Position(3,5),
           dest_pos: Position(4,4),
           meta_info: CAPTURE
        },
        Move {
           origin_piece: B_PAWN,
           dest_piece: NO_PIECE,
           origin_pos: Position(3,5),
           dest_pos: Position(2,4),
           meta_info: QUIET_MOVE 
        }
    ]);

    assert!(diff.len() == 0);
}

#[test]
fn test_generate_king_moves_main() {
    // A B C D E F G H
    // 
    // 8  - - - - - - - k
    // 7  - - - - - - - -
    // 6  - - - - - - - -
    // 5  - p - - - P - -
    // 4  - - - - - - - -
    // 3  - P p - r - - -
    // 2  - - K - b - - p
    // 1  - - - - - - - -

    let board = Board::from_fen(KING_TEST_FEN);
    board.print_board();

    //TODO: test the case of include_illegals = true
    let piece = board.get_piece_by_pgn("c2");
    let moves = generate_king_moves(piece, &board, false);
    let positions = moves.map(|m| { m.dest_pos }).collect::<Vec<Position>>();
    let correct = Position::from_pgn_list("c1, b1");
    
    assert!(are_positions_eq(&positions, &correct));
}

#[test]
fn test_generate_king_moves_include_as_attacks() {
    let board = Board::from_fen(KING_TEST_FEN);
    board.print_board();

    let piece = board.get_piece_by_pgn("c2");
    let moves = generate_king_moves(piece, &board, true);
    let positions = moves.map(|m| { m.dest_pos }).collect::<Vec<Position>>();
    let correct = Position::from_pgn_list("c3, d1, d2, d3, c1, b1, b2, b3");
    
    assert!(are_positions_eq(&positions, &correct));
}

#[test]
fn test_generate_knight_moves() {
    let board = Board::from_fen(KNIGHT_TEST_FEN);
    board.print_board();

    let piece = board.get_piece_position(4, 2);
    let moves = generate_knight_moves(piece, &board, false);
    let positions = moves.map(|m: Move| { m.dest_pos }).collect::<Vec<Position>>();

    let correct = Position::from_pgn_list("f5, g4, g2, d1, f1, c2, c4");
    
    assert!(are_positions_eq(&positions, &correct));
}

#[test]        
fn test_horizontal_squares() {
    let ray = horizontal_squares(Position(5,0));
    assert_eq!(ray.squares, vec![
        Position(0,0),
        Position(1,0),
        Position(2,0),
        Position(3,0),
        Position(4,0),
        Position(5,0),
        Position(6,0),
        Position(7,0),
    ]);
    assert_eq!(ray.attacker_index, 5);
}

#[test]
fn test_anti_diag_squares() {
    let mut ray = anti_diag_squares(Position(0,0));
                    
    assert_eq!(ray.squares, vec![
        Position(0,0),
    ]);

    assert_eq!(ray.attacker_index, 0);

    ray = anti_diag_squares(Position(3,6));
    assert_eq!(ray.squares, vec![
        Position(7,2),
        Position(6,3),
        Position(5,4),
        Position(4,5),
        Position(3,6),
        Position(2,7),
    ]);

    assert_eq!(ray.attacker_index, 4);
    
    ray = anti_diag_squares(Position(7,2));
    assert_eq!(ray.squares, vec![
        Position(7,2),
        Position(6,3),
        Position(5,4),
        Position(4,5),
        Position(3,6),
        Position(2,7),
    ]);
    assert_eq!(ray.attacker_index, 0);
    
    ray = anti_diag_squares(Position(0,7));
    assert_eq!(ray.squares, vec![
        Position(7,0),
        Position(6,1),
        Position(5,2),
        Position(4,3),
        Position(3,4),
        Position(2,5),
        Position(1,6),
        Position(0,7),
    ]);

    assert_eq!(ray.attacker_index, 7);
}

#[test]        
fn test_diag_squares() {
    let mut ray = diag_squares(Position(0,0));
                    
    assert_eq!(ray.squares, vec![
        Position(0,0),
        Position(1,1),
        Position(2,2),
        Position(3,3),
        Position(4,4),
        Position(5,5),
        Position(6,6),
        Position(7,7),
    ]);

    assert_eq!(ray.attacker_index, 0);

    ray = diag_squares(Position(3,6));
    assert_eq!(ray.squares, vec![
        Position(0,3),
        Position(1,4),
        Position(2,5),
        Position(3,6),
        Position(4,7),
    ]);

    assert_eq!(ray.attacker_index, 3);
    
    ray = diag_squares(Position(7,2));
    assert_eq!(ray.squares, vec![
        Position(5,0),
        Position(6,1),
        Position(7,2),
    ]);

    assert_eq!(ray.attacker_index, 2);
    
    ray = diag_squares(Position(3,2));
    assert_eq!(ray.squares, vec![
        Position(1,0),
        Position(2,1),
        Position(3,2),
        Position(4,3),
        Position(5,4),
        Position(6,5),
        Position(7,6),
    ]);
    assert_eq!(ray.attacker_index, 2);
}
