use prusti_contracts::*;

use core::ops::Deref;
use crate::{
    range_inclusive::*,
    with_concrete_type::range_trait::*,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Range(RangeInclusive<usize>);

impl UniqueCheck for Range {
    #[pure]
    #[trusted] // needs to be trusted to use comparison operators
    fn overlaps(&self, other: &Self) -> bool {
        let starts = if self.start() > other.start() { self.start() } else { other.start() };
        let ends   = if self.end() < other.end() { self.end() } else { other.end() };
        starts <= ends
    }
}

impl Deref for Range {
    type Target = RangeInclusive<usize>;
    #[pure]
    fn deref(&self) -> &RangeInclusive<usize> {
        &self.0
    }
}