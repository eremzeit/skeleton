#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use std::collections;

#[allow(unused_imports)]
use std::collections::hash_map::RandomState;

#[test]        
fn test_move_debug_fmt() {
    assert_eq!(format!("{:?}", Move {
        origin_piece: W_BISHOP,
        dest_piece: B_KNIGHT,
        origin_pos: Position(0,0),
        dest_pos: Position(5,5),
        meta_info: QUIET_MOVE
    }), "Ba1-f6");
    
    assert_eq!(format!("{:?}", Move {
        origin_piece: W_BISHOP,
        dest_piece: B_KNIGHT,
        origin_pos: Position(0,0),
        dest_pos: Position(5,5),
        meta_info: CAPTURE 
    }), "Ba1*nf6");
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

const TEST_FEN1: &'static str = "r1bk1b1K/pp2p1p1/N1p1Pq1B/2B1rp2/Rn1P1PQP/1p1n1R2/P1P1P1P1/1N6 w - - 6 1";
const TEST_FEN2: &'static str = "7k/8/8/1p3P2/8/3B4/8/K7 w - - 0 1";

#[test]        
fn test_generate_bishop_moves() {
    let board = Board::from_fen(TEST_FEN2);
    board.print_board();
    assert_eq!(board.to_move, WHITE);
    
    let piece = board.get_piece_position(3, 2);
    assert_eq!(piece.0, W_BISHOP);
    let moves = generate_bishop_moves(piece, &board);
    
    assert_eq!(moves.iter().all(|m| { m.origin_pos == piece.to_position() }), true);
    assert_eq!(moves.iter().all(|m| { m.origin_piece == W_BISHOP}), true); 
    assert_eq!(moves.iter().all(|m| { m.dest_piece == NO_PIECE || m.dest_piece >= B_PAWN  && m.dest_piece <= B_KING}), true); 
    
    let moves_set = moves.iter().map(|m| { m.dest_pos }).collect::<collections::HashSet<Position, RandomState>>();

    println!("Bishop moves: {:?}", moves_set);

    let correct = vec![
        //diag
        Position(1,0),
        Position(2,1),
        Position(4,3),

        //anti-diag
        Position(5,0),
        Position(4,1),
        Position(2,3),
        Position(1,2),
    ];

    let correct_set = correct.iter().map(|p| { p.clone() }).collect::<collections::HashSet<Position, RandomState>>();
    
    let diff = moves_set.difference(&correct_set).collect::<Vec<_>>();
    println!("diff: {:?}", diff);

    assert_eq!(diff.len(), 0); 
}
