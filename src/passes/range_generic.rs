use prusti_contracts::*;

use core::ops::Deref;
use crate::{
    range_inclusive::*,
    passes::range_trait::*,
};

/// A struct representing an unallocated region in memory.
/// Its functions are formally verified to prevent range overlaps between chunks.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Range(RangeInclusive<usize>);

impl UniqueCheck for Range {
    #[pure]
    #[trusted]
    fn range_overlaps(&self, other: &Self) -> bool {
        let starts = if self.start() > other.start() { self.start() } else { other.start() };
        let ends   = if self.end() < other.end() { self.end() } else { other.end() };
        starts <= ends
    }

    #[pure]
    #[trusted]
    fn equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl Deref for Range {
    type Target = RangeInclusive<usize>;
    #[pure]
    fn deref(&self) -> &RangeInclusive<usize> {
        &self.0
    }
}