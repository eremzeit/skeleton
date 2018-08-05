//use super::types::*;
// use regex::Regex;
// //use util::*;
// use constants::*;
// use util::{opposite_color, piece_type_to_str};
// 
// use board::Board;
// //use types::{};
// use search::types::{
//     Score,
//     SearchHit,
//     SearchParams,
//     SearchResults,
//     SearchStyle,
//     MAX_SCORE,
//     MIN_SCORE,
//     score_string,
//     color_string,
// };
// 
// use search::observers::{
//     SearchObservable,
//     SearchLogger,
//     NoopObserver,
//     print_s,
// };
// 
// use board::utils::{PieceIter};
// use moves::generation::{generate_all_moves_for_color, is_color_checkmated};
// use moves::types::{Move};
// use moves::make_move::{make_move};
// use moves::unmake_move::{unmake_move};
// use moves::{is_color_in_check, does_match_moves};
// 
// After 1 ply the score -3.
// After 3 ply white is ahead
// but after 3 black won.
const SEARCH_TEST1: &'static str = "6bk/5p2/P4Pp1/5pP1/4pP1p/3pPp2/P2PpP2/4K3 w - - 0 1";
const SEARCH_TEST2: &'static str = "6bk/5p2/P4Pp1/5pP1/4pP1p/3pPp2/3PpP2/4K3 w - - 0 1";

#[allow(unused_imports)]
use super::*;

#[test]        
fn search_depth_2() {
    let search_params = SearchParams {
        search_ply_target: 2,
        orig_to_move: WHITE,
        search_style: SearchStyle::BASIC
    };

    let mut board = Board::from_fen(SEARCH_TEST2);
    let search_hit = ab_search(&mut board, &search_params).unwrap();
    assert_eq!(search_hit.score, -4);
}

#[test]        
fn search_depth_4() {
    let search_params = SearchParams {
        search_ply_target: 4,
        orig_to_move: WHITE,
        search_style: SearchStyle::BASIC
    };

    let mut board = Board::from_fen(SEARCH_TEST2);
    let search_hit = ab_search(&mut board, &search_params).unwrap();
    println!("search_hit: {:?}", search_hit);
    assert_eq!(search_hit.score_by_color(WHITE), 5);
}

#[test]        
fn search_depth_6() {
    let search_params = SearchParams {
        search_ply_target: 6,
        orig_to_move: WHITE,
        search_style: SearchStyle::BASIC
    };

    let mut board = Board::from_fen(SEARCH_TEST2);
    let search_hit = ab_search(&mut board, &search_params).unwrap();
    println!("search_hit: {:?}", search_hit);
    assert_eq!(search_hit.score_by_color(WHITE), 8);
}

#[test]        
fn search_depth_8() {
    let search_params = SearchParams {
        search_ply_target: 8,
        orig_to_move: WHITE,
        search_style: SearchStyle::BASIC
    };

    let mut board = Board::from_fen(SEARCH_TEST2);
    let search_hit = ab_search(&mut board, &search_params).unwrap();
    println!("search_hit: {:?}", search_hit);
    
    // white can force a stalemate with [Pa6-a7, ph4-h3, Pa7=Qa8, ph3-h2, Qa8*bg8, kh8*Qg8]
    assert_eq!(search_hit.score_by_color(WHITE), 0);
}

#[test]        
fn search_depth_possible_mate() {
    let search_params = SearchParams {
        search_ply_target: 9,
        orig_to_move: WHITE,
        search_style: SearchStyle::BASIC
    };

    let mut board = Board::from_fen(SEARCH_TEST1);
    let search_hit = ab_search(&mut board, &search_params).unwrap();
    println!("search_hit: {:?}", search_hit);
    assert_eq!(search_hit.score_by_color(WHITE), MIN_SCORE + search_hit.move_list.len() as Score);
}


#[test]        
fn test_utility() {
    let mut board = Board::from_fen(SEARCH_TEST1);
    assert_eq!(utility(&board, 0), -3);
}
