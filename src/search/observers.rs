//use super::types::*;
use regex::Regex;
//use util::*;
use constants::*;
use util::{opposite_color, piece_type_to_str};

use board::Board;
//use types::{};
use search::types::{
    Score,
    SearchHit,
    SearchParams,
    SearchResults,
    SearchStyle,
    MAX_SCORE,
    MIN_SCORE,
    score_string,
    color_string,
};

use board::utils::{PieceIter};
use moves::generation::{generate_all_moves_for_color, is_color_checkmated};
use moves::types::{Move};
use moves::make_move::{make_move};
use moves::unmake_move::{unmake_move};
use moves::{is_color_in_check, does_match_moves};

pub fn print_s(_string: &String, search_depth: u8, moves: &Vec<Move>) {
    let mut indent_max = 3;

    // if does_match_moves(r"\[Pa6-a7, ph4-h3, Pa7=Qa8, ph3-h2, Qa8[*]bg8, kh8[*]Qg8.*", moves) {
    //     indent_max = 8;
    // }

    if search_depth <= indent_max {
        println!("|{}{}", " ".repeat(search_depth as usize * 4), _string);  
    }
}

pub trait SearchObservable {
    fn search_start(&self, frame_color: Color, board: &Board) { }
    fn recursed(&self, frame_color: Color, depth: u8, our_best: Score, their_best: Score, move_list: &Vec<Move>) {}
    fn fail_high(&self, move_score: Score, frame_color: Color, depth: u8, our_best: Score, their_best: Score, move_list: &Vec<Move>) {}
    fn leaf_node(&self, move_score: Score, frame_color: Color, depth: u8, our_best: Score, their_best: Score, move_list: &Vec<Move>) {}
    fn fail_low(&self, depth: u8) {}
    fn new_best(&self, depth: u8, our_local_best: Score, moves: &Vec<Move>) {}
    fn move_scored(&self, search_hit: &SearchHit, mv: &Move, depth: u8, moves: &Vec<Move>) {}
    fn moves_generated(&self, depth: u8, possible_moves: &Vec<Move>, move_history: &Vec<Move>) {}
    fn finished(&self, search_hit: &Option<SearchHit>) {}
}

pub struct SearchLogger { }

impl SearchObservable for SearchLogger {
    fn search_start(&self, frame_color: Color, board: &Board) {
        board.print_board();
    }

    fn recursed(&self, frame_color: Color, depth: u8, our_best: Score, their_best: Score, move_list: &Vec<Move>) {
        print_s(&format!("----"), depth, move_list);
        print_s(&format!("[{}] SEARCH: {}, {}, {}, {:?}", depth, color_string(frame_color), our_best, their_best, move_list), depth, move_list);
    }

    fn fail_high(&self, move_score: Score, frame_color: Color, depth: u8, our_best: Score, their_best: Score, move_list: &Vec<Move>) {
        let mv = move_list.last().unwrap();
        print_s(&format!("[{}] pruning caused by {:?} with value {} against their best score: {}", depth, mv, move_score, their_best), depth, move_list);
    }

    fn leaf_node(&self, move_score: Score, frame_color: Color, depth: u8, our_best: Score, their_best: Score, move_list: &Vec<Move>) {
        let mv = move_list.last().unwrap();
        print_s(&format!("[{}] pruning caused by {:?} with value {} against their best score: {}", depth, mv, move_score, their_best), depth, move_list);
    }

    fn fail_low(&self, depth: u8) {

    }

    fn new_best(&self, depth: u8, our_local_best: Score, moves: &Vec<Move>) {
        print_s(&format!("[{}] new local best: {:?}", depth, our_local_best), depth, moves);
    }

    fn move_scored(&self, search_hit: &SearchHit, mv: &Move, depth: u8, moves: &Vec<Move>) {
        print_s(&format!("[{}] Move {:?} has score {:?}", depth, mv, search_hit), depth, moves);
    }

    fn moves_generated(&self, depth: u8, possible_moves: &Vec<Move>, move_history: &Vec<Move>) {
        print_s(&format!("[{}] Possible moves: {:?}", depth, possible_moves), depth, move_history);
    }
    
    fn finished(&self, search_hit: &Option<SearchHit>) {
        if search_hit.is_some() {
            println!("Best move found: {:?}", search_hit.as_ref());
        } else {
            println!("Best move found: None");
        }

    //print_s(&format!("[{}] Best move found {:?}", depth, best_search_hit), depth, moves);
    }
}

pub struct NoopObserver { }

impl SearchObservable for NoopObserver {}
