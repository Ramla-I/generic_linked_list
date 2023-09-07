use prusti_contracts::*;

use core::ops::Deref;
use crate::{
    range_inclusive::*,
    failing::range_trait::*,
};

/// A struct representing an unallocated region in memory.
/// Its functions are formally verified to prevent range overlaps between chunks.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Range<U: UnitTrait>(RangeInclusive<U>);

impl<U: UnitTrait> UniqueCheck for Range<U> {
    #[pure]
    #[trusted]
    fn range_overlaps(&self, other: &Self) -> bool {
        self.range_overlaps_inner(other)
    }

    #[pure]
    #[trusted]
    fn equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl<U: UnitTrait> Range<U> {
    #[ensures(result.start_frame() == start)]
    #[ensures(result.end_frame() == end)]
    pub const fn new(start: U, end: U) -> Range<U> {
        Range(RangeInclusive::new(start, end))
    }

    // #[trusted]
    #[ensures(result.is_empty())]
    pub fn empty() -> Range<U> {
        Range::new(U::new(1), U::new(0))
    }

    #[pure]
    #[trusted]
    #[ensures(result == *self.0.start())]
    pub fn start_frame(&self) -> U {
        *self.0.start()
    }

    #[pure]
    #[trusted]
    #[ensures(result == *self.0.end())]
    pub fn end_frame(&self) -> U {
        *self.0.end()
    }

    #[pure]
    #[ensures(result == (self.start_frame().greater_than(&self.end_frame())))]
    #[ensures(result == !(self.start_frame().less_than_equal(&self.end_frame())))]
    pub fn is_empty(&self) -> bool {
        !(self.start_frame().less_than_equal(&self.end_frame()))
    }

    #[pure]
    #[trusted] // has to be trusted to call itself, which then requires us to define a spec for the fn as well :(
    #[ensures(result == other.range_overlaps(&self))] // if we dont have this condition, then post-condition of push_unique_with_precond wont' verify
    #[ensures({
        let starts = max_mem_unit(self.start_frame(), other.start_frame());
        let ends   = min_mem_unit(self.end_frame(), other.end_frame());
        result == starts.less_than_equal(&ends) 
    })]
    /// Returning a FrameRange here requires use to set the RangeInclusive new function as pure which
    /// requires Idx to be Copy, so just return bool.
    pub fn range_overlaps_inner(&self, other: &Range<U>) -> bool {
        let starts = max_mem_unit(self.start_frame(), other.start_frame());
        let ends   = min_mem_unit(self.end_frame(), other.end_frame());
        starts.less_than_equal(&ends)
    }

    #[pure]
    pub fn contains_range(&self, other: &Range<U>) -> bool {
        !other.is_empty()
        && (other.start_frame().greater_than_equal(&self.start_frame()))
        && (other.end_frame().less_than_equal(&self.end_frame()))
    }
}


impl<U: UnitTrait> Deref for Range<U> {
    type Target = RangeInclusive<U>;
    #[pure]
    fn deref(&self) -> &RangeInclusive<U> {
        &self.0
    }
}

#[pure]
fn min_mem_unit<U: UnitTrait>(a: U, b: U) -> U {
    if a.less_than(&b) { a } else { b }
}

#[pure]
fn max_mem_unit<U: UnitTrait>(a: U, b: U) -> U {
    if a.greater_than(&b) { a } else { b }
}
