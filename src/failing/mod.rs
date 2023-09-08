pub mod linked_list;
pub mod range_generic;
pub mod range_trait;

use range_generic::Range;
use linked_list::List;

/// Returns true if the given range does not overlap with any of the ranges in the list.
fn can_create_new<U: Copy + PartialOrd>(chunk_range: Range<U>, chunk_list: &mut List<Range<U>>) -> Result<(), usize> {
    let overlap_idx = chunk_list.elem_overlaps_in_list(chunk_range, 0);
    if overlap_idx.is_some(){
        Err(overlap_idx.unwrap())
    } else {
        chunk_list.push(chunk_range);
        Ok(())
    }
}