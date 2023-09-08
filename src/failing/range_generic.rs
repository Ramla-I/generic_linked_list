use prusti_contracts::*;

use core::ops::Deref;
use crate::{
    range_inclusive::*,
    failing::range_trait::*,
};

/// A struct representing an unallocated region in memory.
/// Its functions are formally verified to prevent range overlaps between chunks.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Range<U: Copy + PartialOrd>(RangeInclusive<U>);

impl<U: Copy + PartialOrd> UniqueCheck for Range<U> {
    #[pure]
    // #[trusted]
    fn overlaps(&self, other: &Self) -> bool {
        false // just a dummy value for now
    }
}

impl<U: Copy + PartialOrd> Deref for Range<U> {
    type Target = RangeInclusive<U>;
    #[pure]
    fn deref(&self) -> &RangeInclusive<U> {
        &self.0
    }
}
