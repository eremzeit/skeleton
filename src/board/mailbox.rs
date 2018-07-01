use constants::*;
use bitboard::BitBoard;
use std::boxed::Box;
use util::*;
use types::*;

#[derive(Copy)]
pub struct Mailbox(pub [PieceType; 128]);

impl Clone for Mailbox { fn clone(&self) -> Self { *self } }

// all valid indexes into the mailbox can be anded with this and should equal 0
pub const MAILBOX_INDEX_MASK: u8 = 0x88;

impl Mailbox {
    pub fn empty() -> Mailbox {
        let mut mb = Mailbox([OFF_BOARD; 128]);

        for r in ranks_desc() {
            for f in files_asc() {
                mb.set(f, r, NO_PIECE);
            }
        }

        mb
    }
    
    pub fn getp(&self, pos: Position) -> PieceType {
        self.get(pos.0, pos.1)
    }
    
    pub fn setp(&mut self, pos: Position, piece: PieceType) {
        self.set(pos.0, pos.1, piece);
    }

    // TODO: this is slow.  if we want to be able to handle off-board queries
    // while still being performant, we should change the underlying data-structure.
    // We could also move away from using file and rank separately.  two seperate loops means
    // we're bounds checking more often than necessary.
    pub fn get(&self, file: File, rank: Rank) -> PieceType {
        assert!(file >= -2 && file < FILE_COUNT + 2);    
        assert!(rank >= -2 && rank < RANK_COUNT + 2);

        if file >= 0 && file < FILE_COUNT && rank >= 0 && rank < RANK_COUNT {
            let ind: u8 = ((rank as u8) << 4) + (file as u8);
            assert_eq!(ind & MAILBOX_INDEX_MASK, 0);
            return self.0[((rank << 4) + file) as usize]
        } else {
            OFF_BOARD     
        }
    }
    
    pub fn set(&mut self, file: File, rank: Rank, piece:PieceType) {
        self.0[((rank << 4) + file) as usize] = piece
    }

    pub fn move_piece(&mut self, orig_pos: Position, dest_pos: Position) {
        assert!(self.get(orig_pos.0, orig_pos.1) != NO_PIECE);

        let piece = self.get(orig_pos.0, orig_pos.1);
        self.set(dest_pos.0, dest_pos.1, piece);
        self.set(orig_pos.0, orig_pos.1, NO_PIECE);
    }
}

// impl PartialEq for [PieceType; 128] {
//     fn eq(&self, other: &[PieceType; 128]) -> bool {
//         self.0.as_slice() == other.0.as_slice() 
//     }
// }

impl PartialEq for Mailbox {
    fn eq(&self, other: &Mailbox) -> bool {
        for i in 0..128 {
            if self.0[i] != other.0[i] {
                return false;
            }
        }

        true
    }
}
