use prusti_contracts::*;

use core::ops::Deref;
use crate::{
    *,
    failing::{linked_list::*, range_trait::*, range_generic::*},
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
pub struct Chunk<U: UnitTrait> {
    frames: Range<U>
}

impl<U: UnitTrait> Chunk<U> {
    fn new(chunk_range: Range<U>, chunk_list: &mut List<Range<U>>) -> Result<Chunk<U>, ChunkCreationError> {
        let overlap_idx = chunk_list.elem_overlaps_in_list(chunk_range, 0);
        if overlap_idx.is_some(){
            Err(ChunkCreationError::Overlap(overlap_idx.unwrap()))
        } else {
            chunk_list.push(chunk_range);
            Ok(Chunk { frames: chunk_range })
        }
    }
}


impl<U: UnitTrait> Deref for Chunk<U> {
    type Target = Range<U>;
    #[pure]
    fn deref(&self) -> &Range<U> {
        &self.frames
    }
}
