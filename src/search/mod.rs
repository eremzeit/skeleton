use super::types::*;
use util::*;
use constants::*;
use board::Board;
use types::*;
use search::types::{Score};
use moves::generation::{generate_all_moves_for_color}

struct SearchParams {
    pub cutoff_depth: u8,
    pub orig_to_move: Color,
}

struct SearchResults {
    pub searchHits: Vec<SearchHit>,
}

// An individual result
struct SearchHit {
    pub mv: Move,
    pub depth_searched: u8,
    pub score: u8,
    pub color: Color,
    pub move_list: Vec<Move>
}

pub fn isTerminalState(board: &Board, search_params: &SearchParams, current_depth: u8, alpha: Score, beta: Score) -> SearchHit {
    return current_depth > 8;
}

pub fn utility(board: &Board, current_depth: u8) -> Score {
        
}

pub fn ab_search_max(board: &mut Board, search_params: &SearchParams, current_depth: u8, alpha: Score, beta: Score, moves: &mut Vec<Move>) ->  {
    let mut score;
    // enumerate all possible moves
    // for each move, recurse
    //
    // if at leaf node, calculate utility score
    // check for a terminal condition
    if isTerminalState(board, search_params, current_depth, alpha, beta) {
        score = utility(board, current_depth);    
        return res
    }

    for mv in generate_all_moves_for_color(board, board.to_move).iter() {
        moves.push(mv);
        make_move(board, mv);
        let mut search_result = ab_search_min(board, search_params, current_depth, alpha, beta, moves);
        
        if search_result.score >= beta: //fail high (ie. we'd never get to this point via a rational player)
            search_result.score = beta;
            unmake_move(board, mv);

            return search_result;
        if res.score > alpha:
            maxRes = res
            alpha = res.score
    }
    
    for move in moveGen:
        _moveList = moveList[:] + [move]
        zboard.MakeMove(move)

        // recurse  
        res = minimax_Min(zboard, alpha, beta, curDepth + 1, cutoffDepth, _moveList)
        
        if res.score >= beta: #fail high
            #pdb.set_trace()
            #iprint('fail high: %s' % (str(res)), curDepth)
            res.score = beta 
            zboard.UnmakeMove()
            return res
        if res.score > alpha:
            #iprint('res: ' + str(res), curDepth)
            maxRes = res
            alpha = res.score
    #maxRes.score = alpha

    if curDepth != 0: zboard.UnmakeMove()

    return maxRes

}


//def minimax_Max(zboard, alpha, beta, curDepth, cutoffDepth, moveList):



