use moves::types::{Move};
use constants::{Color, WHITE, BLACK};
use std::fmt;
use util::{opposite_color};

pub type Score = i16;

pub enum SearchStyle {
    BASIC,
}

pub const MIN_SCORE: i16 = -32767;
pub const MAX_SCORE: i16 = 32767;

pub struct SearchParams {
    pub search_ply_target: u8,
    pub orig_to_move: Color,
    pub search_style: SearchStyle,
}

pub struct SearchResults {
    pub search_hits: Vec<SearchHit>,
}

// An individual result
pub struct SearchHit {
    pub mv: Move,
    pub depth_searched: u8,
    pub score: Score,
    pub color: Color,
    pub move_list: Vec<Move>
}

impl SearchHit {
    pub fn toggle_color_frame(&self) -> SearchHit {
        SearchHit {
            mv: self.mv.clone(),
            depth_searched: self.depth_searched,
            score: -self.score,
            color: opposite_color(self.color),
            move_list: self.move_list.clone(),
        }
    }

    pub fn set_score_by_color(&mut self, score: Score, color: Color) {
        self.score = match(color) {
            WHITE => score,
            _ => -score,
        }
    }
    pub fn score_by_color(&self, color: Color) -> Score {
        match(color) {
            WHITE => self.score,
            _ => -self.score,
        }
    }
}

pub fn color_string(color: Color) -> String {
    match(color) {
        WHITE => "WHITE",
        _ => "BLACK",
    }.to_string()
}


pub fn score_string(score: Score, color: Color) -> String {
    let _score: Score = match(color) {
        WHITE => score,
        _ => -score,
    };
    
    format!("{}:{}", color_string(color), _score)
}

impl fmt::Debug for SearchHit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<({}) {:?} {}>", score_string(self.score, self.color), self.move_list, self.depth_searched)    
    }
}

