use prusti_contracts::*;

pub trait UniqueCheck: Copy + PartialEq {
    #[pure]
    #[trusted] // has to be trusted to call itself, which then requires us to define a spec for the fn as well :(
    #[ensures(result == other.range_overlaps(&self))] // if we dont have this condition, then post-condition of push_unique_with_precond wont' verify
    fn range_overlaps(&self, other: &Self) -> bool;

    #[pure]
    #[trusted]
    #[ensures(result == other.equals(&self))] // if we dont have this condition, then post-condition of push_unique_with_precond wont' verify
    fn equals(&self, other: &Self) -> bool;
}

pub trait UnitTrait: Copy + PartialEq + PartialOrd {
    #[pure]
    fn number(&self) -> usize;

    #[ensures(result.number() == number)]
    fn new(number: usize) -> Self;

    #[pure]
    #[trusted]
    #[ensures(result == (self.number() < rhs.number()))]
    #[ensures(!result ==> self.greater_than_equal(&rhs))]
    fn less_than(self, rhs: &Self) -> bool;

    #[pure]
    #[trusted]
    #[ensures(result == (self.number() <= rhs.number()))]
    #[ensures(!result ==> self.greater_than(&rhs))]
    #[ensures(result ==> self == *rhs || self.less_than(&rhs))]
    fn less_than_equal(self, rhs: &Self) -> bool;

    #[pure]
    #[trusted]
    #[ensures(result == (self.number() > rhs.number()))]
    #[ensures(!result ==> self.less_than_equal(&rhs))]
    fn greater_than(self, rhs: &Self) -> bool;

    #[pure]
    #[trusted]
    #[ensures(result == (self.number() >= rhs.number()))]
    #[ensures(!result ==> self.less_than(&rhs))]
    #[ensures(result ==> self == *rhs || self.greater_than(&rhs))]
    fn greater_than_equal(self, rhs: &Self) -> bool;
}
