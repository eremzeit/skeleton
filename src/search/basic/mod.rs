pub mod tests;

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

use search::observers::{
    SearchObservable,
    SearchLogger,
    NoopObserver,
    print_s,
};

use board::utils::{PieceIter};
use moves::generation::{generate_all_moves_for_color, is_color_checkmated};
use moves::types::{Move};
use moves::make_move::{make_move};
use moves::unmake_move::{unmake_move};
use moves::{is_color_in_check, does_match_moves};

pub fn is_terminal_state(board: &Board, search_params: &SearchParams, depth: u8, alpha: Score, beta: Score) -> bool {
    // the utility function gets run in the next level, so intuitively we the cutoff refers to the
    // last depth that actually makes a move.
    
    // if is_color_checkmated(board, board.to_move) {
    //     //println!("{}", board.to_fen());
    //     board.print_board_indent(depth as usize);
    // }

    depth >= search_params.search_ply_target - 1 || is_color_checkmated(board, board.to_move)
}

fn get_piece_value(piece: PieceType) -> Score {
    match piece {
        NO_PIECE => 0,
        W_PAWN => 1,
        W_KNIGHT => 3,
        W_BISHOP => 3,
        W_ROOK => 5,
        W_QUEEN => 10,
        W_KING => 100,
        
        B_PAWN => -1,
        B_KNIGHT => -3,
        B_BISHOP => -3,
        B_ROOK => -5,
        B_QUEEN => -10,
        B_KING => -100,
        _ => 0,
    }
}

// TODO:
pub fn utility(board: &Board, depth: u8) -> Score {
    let piece_iter = PieceIter::new(&board);
    
    let mut sum: Score = 0;
    for piece in piece_iter {
        //println!("{} -> {}", piece_type_to_str(piece), get_piece_value(piece));
        sum = sum + get_piece_value(piece);
    }

    if is_color_checkmated(board, WHITE) {
        return MIN_SCORE + depth as Score;       
    } else if is_color_checkmated(board, BLACK) {
        return MAX_SCORE - depth as Score;       
    } else {
        sum
    }
}

fn best_score(color: Color) -> Score {
    match color {
        WHITE => MAX_SCORE,
        _ => MIN_SCORE,
    }
}

fn worst_score(color: Color) -> Score {
    match color {
        WHITE => MIN_SCORE,
        _ => MAX_SCORE,
    }
}

fn search_hit(mv: &Move, score: Score, depth: u8, color_frame: Color, moves: Vec<Move>) -> SearchHit {
    SearchHit {
        mv: mv.clone(),
        depth_searched: depth,
        score: score,
        color: color_frame,
        move_list: moves,
    } 
}

pub fn ab_search(board: &mut Board, search_params: &SearchParams) -> Option<SearchHit> {
    board.print_board();
    let mut moves: Vec<Move> = vec![]; 
    let our_best = worst_score(board.to_move);
    let their_best = worst_score(opposite_color(board.to_move));
    let observer: Box<SearchObservable> = Box::new(NoopObserver{});
    return _ab_search(board, search_params, 0, our_best, their_best, &mut moves, &observer);
}

// Responsibility: The board should be back in the original state after the function has 
// completed.  As a rule of thumb, if the current stack frame has added a mv to the move list it
// should also remove it.

// Because we are using negamax, in each frame `our_best` is framed positively and `their_best` is
// framed negatively.  That is, if we want to compare a score in the current frame to `their_best`
// then you need to negate one of them.
pub fn _ab_search(board: &mut Board, search_params: &SearchParams, depth: u8, _our_best: Score, their_best: Score, moves: &mut Vec<Move>, observer: &Box<SearchObservable>) -> Option<SearchHit> {
    let frame_color = board.to_move;

    if depth == 0 {
        observer.search_start(frame_color, board);
    } else {
        observer.recursed(frame_color, depth, _our_best, their_best, moves);
    }

    let mut max: Score = best_score(frame_color);
    let mut best_search_hit: Option<SearchHit> = None;
    let mut our_best = _our_best;

    let mut our_local_best: Score = MIN_SCORE;

    // if at leaf node, calculate utility score
    // check for a terminal condition
    if is_terminal_state(board, search_params, depth, our_best, their_best) {
        let score = utility(board, depth);

        let mv: Move = moves.last().unwrap().clone();
        
        let search_hit = search_hit(&mv, score, depth, frame_color, moves.clone());


        observer.leaf_node(score, frame_color, depth, our_best, their_best, moves);
        //print_s(&format!("[{}] leaf node {} -- {:?}", depth, score, search_hit), depth, moves);
        return Some(search_hit);
    }
    
    // enumerate all possible moves
    // for each move, recurse and find the best move.  
    //
    // todo: check refutation table to apply the refutation heuristic
    let all_moves = generate_all_moves_for_color(board, frame_color);

    observer.moves_generated(depth, &all_moves, moves);
    
    if all_moves.len() == 0 {
        print_s(&"stalemate".to_string(), depth, moves);

        //board.print_board_indent(depth as usize);
        // No legal moves means a stalemate, which has the value 0
        return Some(search_hit(&moves.first().unwrap().clone(), 0, depth, frame_color, moves.clone()));
    }
    
    for mv in all_moves.iter() {
        //print_s(&format!("[{}] Making move {:?}", depth, mv), depth, moves);

        moves.push(mv.clone());
        make_move(board, mv.clone());

        let mut maybe_search_hit: Option<SearchHit> = _ab_search(board, search_params, depth+1, -their_best, -our_best, moves, observer);
        
        if maybe_search_hit.is_some() {
            let mut search_hit = maybe_search_hit.unwrap();
            let score = search_hit.score_by_color(frame_color);

            //print_s(&format!("[{}] Move {:?} has score {} {:?}", depth, mv, score, search_hit), depth, moves);
            observer.move_scored(&search_hit, mv, depth, moves);
            
            // We should try to avoid having ties of scores.  Score should be switched to be in
            // terms of centipawns.  However, how would the engine search differently if this were
            // GT instead of GTE?  If there's 4 nodes with a score that match the cutoff value,
            // then using GTE would mean that all 4 of those would get pruned.  In the GTE method, 
            // there's less search time spent on exploring nodes that appear to have equal scores,
            // which allows for more searching on the first node that was found with that value.
            // The GT method has an advantage that it lessens the order effects in searching
            // because all nodes with equal value are searched.
            //

            // if score > 100 {
            //     print_s(&format!("[{}] Found what appears to be a checkmate.  With score: {}", depth, score), depth, moves);
            // }
            
            if score + depth as Score == best_score(frame_color) {
                // if it's a checkmate then we can just immediately return because that's the
                // highest score possible.
                // TODO: Technically this implementation loses us the ability to prioritize
                // checkmates at smaller depths.
                
                //board.print_board(); 
                //print_s(&format!("[{}] Returning early because we found a checkmate", depth), depth, moves);

                unmake_move(board, mv);
                moves.pop();
                
                return Some(search_hit)
            } else if score >= their_best { 
                // fail high (ie. prune)
                // 
                // If we find a score that is better than a score that the opposite side could
                // get to by another choice of moves then we don't bother searching down this path anymore.
                // 
                
                observer.fail_high(score, frame_color, depth, our_best, their_best, moves);
                unmake_move(board, mv);
                moves.pop();
                
                // Even if we are pruning we must return a score.  If we return none, then the maximizer 2 plys before us will report that this line is unfruitful, which 
                return Some(search_hit);
            } else if score > our_local_best {
                our_local_best = score;
                best_search_hit = Some(search_hit);
                observer.new_best(depth, our_local_best, moves);

                if our_local_best > our_best {
                    //print_s(&format!("[{}] new best: {:?}", depth, our_local_best), depth, moves);
                    our_best = our_local_best;
                }
            }
            
            unmake_move(board, mv);
            moves.pop();
        } else {
            print_s(&format!("[{}] no search hits for move {:?}", depth, mv), depth, moves);
            unmake_move(board, mv);
            moves.pop();
        }
    }
        
    if depth == 0 {
        observer.finished(&best_search_hit);
    }

    best_search_hit
}
