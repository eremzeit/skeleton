pub mod types;

//use super::types::*;
//use util::*;
use constants::{Color};

use board::Board;
//use types::{};
use search::types::{Score};
use moves::generation::{generate_all_moves_for_color};
use moves::types::{Move};
use moves::make_move::{make_move};
use moves::unmake_move::{unmake_move};


const MAX_SCORE: i8 = 128;
const MIN_SCORE: i8 = -128;

struct SearchParams {
    pub cutoff_depth: u8,
    pub orig_to_move: Color,
}

struct SearchResults {
    pub searchHits: Vec<SearchHit>,
}

// An individual result
pub struct SearchHit {
    pub mv: Move,
    pub depth_searched: u8,
    pub score: i8,
    pub color: Color,
    pub move_list: Vec<Move>
}

pub fn isTerminalState(board: &Board, search_params: &SearchParams, current_depth: u8, alpha: Score, beta: Score) -> bool {
    return current_depth > 8;
}


pub fn utility(board: &Board, current_depth: u8) -> Score {
    0
}

pub fn ab_search(board: &mut Board, search_params: &SearchParams, current_depth: u8) -> SearchHit {
    let max: Score = MIN_SCORE;
    let best_search_hit: SearchHit;

    for mv in generate_all_moves_for_color(board, board.to_move).iter() {
        let mut moves: Vec<Move> = vec![];

        let score: Score = -_ab_search(board, search_params, 0, max, MAX_SCORE, &mut moves);
        
        if score > max {
            max = score;
            best_search_hit = SearchHit {
                mv: mv.clone(),
                depth_searched: current_depth,
                score: score,
                color: board.to_move,
                move_list: moves.clone(),
            }; 
        }
    }
    
    best_search_hit
}

pub fn _ab_search(board: &mut Board, search_params: &SearchParams, current_depth: u8, alpha: Score, beta: Score, moves: &mut Vec<Move>) -> Score {
    let mut _alpha;
    let mut score;
    // enumerate all possible moves
    // for each move, recurse
    //
    // if at leaf node, calculate utility score
    // check for a terminal condition
    if isTerminalState(board, search_params, current_depth, alpha, beta) {
        score = utility(board, current_depth);    
        return score;
    }

    // todo: check refutation table to apply the refutation heuristic

    for mv in generate_all_moves_for_color(board, board.to_move).iter() {
        moves.push(mv.clone());
        make_move(board, mv.clone());
        let mut search_result = _ab_search(board, search_params, current_depth, -beta, -alpha, moves);
        
        if -search_result.score >= beta { //fail high (ie. we'd never get to this point via a rational player)
            search_result.score = beta;
            unmake_move(board, mv);
            return beta;
        } 

        if -search_result.score > alpha {
            alpha = -search_result.score;
        }
        
    }
    
    return alpha;
    
    // for move in moveGen:
    //     _moveList = moveList[:] + [move]
    //     zboard.MakeMove(move)

    //     // recurse  
    //     res = minimax_Min(zboard, alpha, beta, curDepth + 1, cutoffDepth, _moveList)
    //     
    //     if res.score >= beta: #fail high
    //         #pdb.set_trace()
    //         #iprint('fail high: %s' % (str(res)), curDepth)
    //         res.score = beta 
    //         zboard.UnmakeMove()
    //         return res
    //     if res.score > alpha:
    //         #iprint('res: ' + str(res), curDepth)
    //         maxRes = res
    //         alpha = res.score
    // #maxRes.score = alpha

    // if curDepth != 0: zboard.UnmakeMove()

    // return maxRes

}


