use prusti_contracts::*;
use crate::passes::{range_generic::*, linked_list::*};


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
pub struct Chunk {
    frames: Range
}

impl Chunk {
    fn new(chunk_range: Range, chunk_list: &mut List<Range>) -> Result<Chunk, ChunkCreationError> {
        let overlap_idx = chunk_list.elem_overlaps_in_list(chunk_range, 0);
        if overlap_idx.is_some(){
            Err(ChunkCreationError::Overlap(overlap_idx.unwrap()))
        } else {
            chunk_list.push(chunk_range);
            Ok(Chunk { frames: chunk_range })
        }
    }
}