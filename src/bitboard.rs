use std::ops::{Index, IndexMut};
use board::Mailbox;
use constants::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BitBoard(pub [u64; 0x10]);


pub const ALL_WHITE: u8 = 0x0c;
pub const ALL_BLACK: u8 = 0x0d;
pub const ALL: u8 = 0x0e;

impl BitBoard {
    pub fn create_from(mailbox: &Mailbox) -> Self {
        let mut bb = BitBoard([0; 0x10]);

        for f in 0..FILE_COUNT {
            for r in 0..RANK_COUNT {

                let piece_type = mailbox.get(f, r);
                
                if piece_type != NO_PIECE {
                    let bb_index = r * 8 + f;
                    bb[piece_type] = bb[piece_type] | 1 << bb_index;
                }
            }
        }

        bb.normalize();
        bb
    }

    /// Populate the bitboard entries for occupancy
    pub fn normalize(&mut self) {
        self[ALL_WHITE] = self[W_PAWN] | self[W_KNIGHT] | self[W_BISHOP] | self[W_ROOK] |
                          self[W_QUEEN] | self[W_KING];

        self[ALL_BLACK] = self[B_PAWN] | self[B_KNIGHT] | self[B_BISHOP] | self[B_ROOK] |
                          self[B_QUEEN] | self[B_KING];

        self[ALL] = self[ALL_WHITE] | self[ALL_BLACK]
    }
}

impl Index<u8> for BitBoard {
    type Output = u64;
     
    fn index(&self, index: u8) -> &u64 {
        &self.0[index as usize]
    }
}

impl IndexMut<u8> for BitBoard {
    fn index_mut(&mut self, index: u8) -> &mut u64 {
        // what's going on here?
        &mut self.0[index as usize]
    }
}

mod tests {
    #[test]
    fn it_works() {
        
    }
}


