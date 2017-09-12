use super::types::*;
use util::*;
use constants::*;
use board::Board;
use types::*;

struct Search {

}

struct SearchParams {
    pub cutoff_depth: u8,
    pub orig_to_move: Color,
}

// An individual result
struct SearchResult {
    pub mv: Move,
    pub depth_searched: u8,
    pub score: u8,
    pub color: Color,
    pub move_list: Vec<Move>
}


pub fn ab_search_max(board, search_params, alpha, beta) ->  {
    // enumerate all possible moves
    // for each move, recurse
    //
    // if at leaf node, calculate utility score
    //
    
    // check for a terminal condition
    if isTerminalState(zboard, curDepth, cutoffDepth):
        score = utility(zboard, curDepth)

        res = ResInfo(score, curDepth, moveList, zboard.color)
        zboard.UnmakeMove()
        return res
    
    maxRes = None
    moveGen = MoveGen.GetMoveGen(zboard)
    #iprint ('moves to process: %s' % len(moveGen), curDepth)
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



