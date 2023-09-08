//! Most of the List code is adapted from the Prusti user guide 
//! https://viperproject.github.io/prusti-dev/user-guide/tour/summary.html

use prusti_contracts::*;
use crate::{
    external_spec::trusted_option::*,
    without_option::range_trait::*
};
use core::mem;

pub struct List<T: UniqueCheck> {
    head: Link<T>,
}

pub(crate) enum Link<T: UniqueCheck> {
    Empty,
    More(Box<Node<T>>)
}

pub(crate) struct Node<T: UniqueCheck> {
    elem: T,
    next: Link<T>,
}


#[trusted]
#[requires(src.is_empty())]
#[ensures(dest.is_empty())]
#[ensures(old(dest.len()) == result.len())]
#[ensures(forall(|i: usize| (i < result.len()) ==> 
                old(dest.lookup(i)) == result.lookup(i)))] 
fn replace<T: UniqueCheck>(dest: &mut Link<T>, src: Link<T>) -> Link<T> {
    mem::replace(dest, src)
}


impl<T: UniqueCheck> List<T> {
    #[pure]
    pub fn len(&self) -> usize {
        self.head.len()
    }

    /// Looks up an element in the list.
    /// 
    /// # Pre-conditions:
    /// * index is less than the length
    #[pure]
    #[requires(index < self.len())]
    pub fn lookup(&self, index: usize) -> T {
        self.head.lookup(index)
    }

    /// Creates an empty list.
    /// 
    /// # Post-conditions: 
    /// * length is zero.
    #[ensures(result.len() == 0)]
    pub const fn new() -> Self {
        List { head: Link::Empty }
    }


    /// Adds an element to the list.
    /// 
    /// # Post-conditions:
    /// * new_length = old_length + 1
    /// * `elem` is added at index 0
    /// * all previous elements remain in the list, just moved one index forward
    #[ensures(self.len() == old(self.len()) + 1)] 
    #[ensures(self.lookup(0) == elem)]
    #[ensures(forall(|i: usize| (1 <= i && i < self.len()) ==> old(self.lookup(i-1)) == self.lookup(i)))]
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: replace(&mut self.head, Link::Empty)
        });

        self.head = Link::More(new_node);
    }

    /// Removes element at index 0 from the list.
    /// 
    /// Post-conditions:
    /// * if the list is empty, returns None.
    /// * if the list is not empty, returns Some().
    /// * if the list is empty, the length remains 0.
    /// * if the list is not empty, new_length = old_length - 1
    /// * if the list is not empty, the returned element was previously at index 0
    /// * if the list is not empty, all elements in the old list at index [1..] are still in the list, except at one index less.
    #[ensures(old(self.len()) == 0 ==> result.is_none())]
    #[ensures(old(self.len()) > 0 ==> result.is_some())]
    #[ensures(old(self.len()) == 0 ==> self.len() == 0)]
    #[ensures(old(self.len()) > 0 ==> self.len() == old(self.len()-1))]
    #[ensures(old(self.len()) > 0 ==> *peek_option_ref(&result) == old(self.lookup(0)))]
    #[ensures(old(self.len()) > 0 ==>
        forall(|i: usize| (i < self.len()) ==> old(self.lookup(i+1)) == self.lookup(i))
    )]
    pub fn pop(&mut self) -> Option<T> {
        match replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                None
            }
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }


    /// Returns the index of the first element in the list, starting from `index`, which overlaps with `elem`.
    /// Returns None if there is no overlap.
    /// 
    /// # Pre-conditions:
    /// * index is less than or equal to the list length
    /// 
    /// # Post-conditions:
    /// * if the result is Some(idx), then idx is less than the list's length.
    /// * if the result is Some(idx), then the element at idx overlaps with `elem`
    /// * if the result is None, then no element in the lists overlaps with `elem`
    #[requires(index <= self.len())]
    #[ensures(result.is_some() ==> peek_option(&result) < self.len())]
    #[ensures(result.is_some() ==> {
            let idx = peek_option(&result);
            let range = self.lookup(idx);
            range.overlaps(&elem)
        }
    )]
    #[ensures(result.is_none() ==> 
        forall(|i: usize| (index <= i && i < self.len()) ==> {
            let range = self.lookup(i);
            !range.overlaps(&elem)
        })
    )]
    pub(crate) fn elem_overlaps_in_list(&self, elem: T, index: usize) -> Option<usize> {
        if index == self.len() {
            return None;
        }
        let ret = if self.lookup(index).overlaps(&elem) {
            Some(index)
        } else {
            self.elem_overlaps_in_list(elem, index + 1)
        };
        ret
    }

}

impl<T: UniqueCheck> Link<T> {

    /// Recursive function that returns length of the list starting from this Link/ Node
    /// 
    /// # Post-conditions:
    /// * returns 0 if the link is empty
    /// * returns >0 if the link is not empty
    #[pure]
    #[ensures(self.is_empty() ==> result == 0)]
    #[ensures(!self.is_empty() ==> result > 0)]
    fn len(&self) -> usize {
        match self {
            Link::Empty => 0,
            Link::More(node) => 1 + node.next.len(),
        }
    }

    #[pure]
    fn is_empty(&self) -> bool {
        match self {
            Link::Empty => true,
            Link::More(node) => false,
        }
    }

    /// Recursive function that returns the element at the given `index`.
    /// 
    /// # Pre-conditions:
    /// * `index` is less than the list length
    #[pure]
    #[requires(index < self.len())]
    pub fn lookup(&self, index: usize) -> T {
        match self {
            Link::Empty => unreachable!(),
            Link::More(node) => {
                if index == 0 {
                    node.elem
                } else {
                    node.next.lookup(index - 1)
                }
            }
        }
    }

}