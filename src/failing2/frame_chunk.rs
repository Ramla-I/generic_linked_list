use prusti_contracts::*;

use core::ops::{Deref, DerefMut};
use crate::{
    range_inclusive::*,
    external_spec::{trusted_option::*, trusted_result::*},
    failing2::{
        range_trait::*,
        frame_range::*,
        linked_list::*,
    }

};


#[derive(Copy, Clone)]
#[cfg_attr(not(prusti), derive(Debug))]
pub enum ChunkCreationError {
    /// There was already a `FrameChunk` created with an overlapping range
    Overlap(usize),
    /// In the pre-heap-intialization phase, if there is no more space in the array
    NoSpace,
    /// The requested range is empty (end > start)
    InvalidRange
}

/// A struct representing an unallocated region in memory.
/// Its functions are formally verified to prevent range overlaps between chunks.
#[cfg_attr(not(prusti), derive(PartialEq, Eq))]
pub struct FrameChunk {
    frames: FrameRange
}

impl FrameChunk {
    #[pure]
    #[trusted]
    #[ensures(result == *self.frames.start())]
    pub fn start(&self) -> Frame {
        *self.frames.start()
    }

    #[pure]
    #[trusted]
    #[ensures(result == *self.frames.end())]
    pub fn end(&self) -> Frame {
        *self.frames.end()
    }

    #[ensures(result.is_empty())]
    pub const fn empty() -> FrameChunk {
        FrameChunk { frames: FrameRange::empty() }
    }

    #[pure]
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// Creates a new `FrameChunk` with `chunk_range` if no other range in `chunk_list` overlaps with `chunk_range`
    /// and adds the range of the newly created `FrameChunk` to `chunk_list`.
    /// Returns an Err if there is an overlap, with the error value being the index in `chunk_list` of the element which overlaps with `frames`.
    /// 
    /// # Post-conditions:
    /// * If it fails than there was an overlap with an existing chunk or an empty range was passed as an argument
    /// * If it succeeds, then the newly created chunk has the same bounds as `chunk_range`
    /// * If it succeeds, then `chunk_range` is added to the list
    /// * If it succeeds, then the length of `chunk_list` has increased by 1
    /// * If it succeeds, then all other elements in the `chunk_list` remain unchanged
    /// * If it succeeds, then `chunk_range` did not overlap with any element in the old `chunk_list`
    // #[ensures(result.is_err() ==> {
    //     match peek_err(&result) {
    //         ChunkCreationError::Overlap(idx) => (idx < chunk_list.len()) & chunk_list.lookup(idx).overlaps(&chunk_range),
    //         _ => true
    //     }
    // })]
    // #[ensures(result.is_ok() ==> {
    //     let new_chunk = peek_result_ref(&result);
    //     new_chunk.start() == *chunk_range.start() && new_chunk.end() == *chunk_range.end()
    // })]
    // #[ensures(result.is_ok() ==> {
    //     chunk_list.len() >= 1 && chunk_list.lookup(0) == chunk_range
    // })]
    // #[ensures(result.is_ok() ==> chunk_list.len() == old(chunk_list.len()) + 1)] 
    // #[ensures(result.is_ok() ==> {
    //     forall(|i: usize| (1 <= i && i < chunk_list.len()) ==> old(chunk_list.lookup(i-1)) == chunk_list.lookup(i))
    // })]
    // #[ensures(result.is_ok() ==> {
    //     forall(|i: usize| (0 <= i && i < old(chunk_list.len())) ==> !old(chunk_list.lookup(i)).overlaps(&chunk_range))
    // })]
    fn new(chunk_range: FrameRange, chunk_list: &mut List<FrameRange>) -> Result<FrameChunk, ChunkCreationError> {
        if chunk_range.is_empty() {
            return Err(ChunkCreationError::InvalidRange);
        }

        let overlap_idx = chunk_list.elem_overlaps_in_list(chunk_range, 0);
        if overlap_idx.is_some(){
            Err(ChunkCreationError::Overlap(overlap_idx.unwrap()))
        } else {
            chunk_list.push(chunk_range);
            Ok(FrameChunk { frames: chunk_range })
        }
    }
}


impl Deref for FrameChunk {
    type Target = FrameRange;
    #[pure]
    fn deref(&self) -> &FrameRange {
        &self.frames
    }
}
