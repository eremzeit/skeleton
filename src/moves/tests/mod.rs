mod generation_tests;

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use util::*;


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
fn test_is_pos_attacked_2() {
    let board = Board::from_fen(INTERESTING_FEN);
    //board.print_board();
}

#[test]
fn test_is_pos_attacked() {
    let board = Board::from_fen(INTERESTING_FEN);
    board.print_board();
      
    //      A B C D E F G H
    //   8  r - - q k - - r
    //   7  p p - - - p b p
    //   6  - - - p - n p -
    //   5  - - - - - - - -
    //   4  - - P B P Q b -
    //   3  - - - - - - - -
    //   2  P P - - - - P P
    //   1  R N - - K B - R
    
    let attacks: [[bool; 2]; 24] =  [
        // 1
        [false, false],
        [true, false],
        [true, false],
        [true, true],
        [false, false],
        [true, false],
        [true, false],
        [false, false],

        //2
        [true, false], //a
        [true, false], //b
        [false, false], //c
        [true, false], //d
        [true, true], //e
        [true, false], //f
        [true, false], //g
        [true, false], //h
        
        //3
        [true, false], //a
        [true, false], //b
        [true, false], //c
        [true, false], //d
        [true, false], //e
        [true, true], //f
        [true, false], //g
        [true, true], //h
    ];

    for i in 0..24 {
        let is_attacked = attacks[i];
        let pos = Position((i as File) % 8, (i / 8) as Rank);
        
        let correct = (
            is_pos_attacked_by(&board, pos, WHITE) == is_attacked[0]
            && is_pos_attacked_by(&board, pos, BLACK) == is_attacked[1]
        ); 

        if !correct {
            println!("failed at: {:?}", pos);
            assert!(correct)
        }
    }
    
    assert!(!is_pos_attacked_by(&board, Position::from_pgn("g7"), WHITE));
    assert!(!is_pos_attacked_by(&board, Position::from_pgn("g7"), BLACK));
    
    assert!(!is_pos_attacked_by(&board, Position::from_pgn("g6"), WHITE));
    assert!(is_pos_attacked_by(&board, Position::from_pgn("g6"), BLACK));

    assert!(is_pos_attacked_by(&board, Position::from_pgn("f6"), WHITE));
    assert!(is_pos_attacked_by(&board, Position::from_pgn("f6"), BLACK));
    
    assert!(is_pos_attacked_by(&board, Position::from_pgn("a2"), WHITE));
    assert!(!is_pos_attacked_by(&board, Position::from_pgn("a2"), BLACK));
    
    assert!(!is_pos_attacked_by(&board, Position::from_pgn("a8"), WHITE));
    assert!(is_pos_attacked_by(&board, Position::from_pgn("a8"), BLACK));
    
    assert!(!is_pos_attacked_by(&board, Position::from_pgn("b8"), WHITE));
    assert!(is_pos_attacked_by(&board, Position::from_pgn("b8"), BLACK));

    assert!(is_pos_attacked_by(&board, Position::from_pgn("a7"), WHITE));
    assert!(is_pos_attacked_by(&board, Position::from_pgn("a7"), BLACK));
    
    assert!(!is_pos_attacked_by(&board, Position::from_pgn("a6"), WHITE));
    assert!(is_pos_attacked_by(&board, Position::from_pgn("a6"), BLACK));
    
    assert!(!is_pos_attacked_by(&board, Position::from_pgn("a6"), WHITE));
    assert!(is_pos_attacked_by(&board, Position::from_pgn("a6"), BLACK));

    assert!(!is_pos_attacked_by(&board, Position::from_pgn("f7"), WHITE));
    assert!(is_pos_attacked_by(&board, Position::from_pgn("f7"), BLACK));
}
