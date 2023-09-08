pub mod linked_list;
pub mod range_trait;
pub mod frame_range;
pub mod frame_chunk;

use frame_range::FrameRange;
use linked_list::List;

fn can_create_new(chunk_range: FrameRange, chunk_list: &mut List<FrameRange>) -> Result<(), usize> {
    let overlap_idx = chunk_list.elem_overlaps_in_list(chunk_range, 0);
    if overlap_idx.is_some(){
        Err(overlap_idx.unwrap())
    } else {
        chunk_list.push(chunk_range);
        Ok(())
    }
}