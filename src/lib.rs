use alga::general::Operator;
use std::ops::Range;

pub type Time = i32;
pub type Count = i32;
pub(crate) type Uid = u32;

const NEG_INFINITY: Time = i32::MIN;
const POS_INFINITY: Time = i32::MAX;

pub trait TimeWindow<T, O: Operator> {
    fn new() -> Self;
    fn insert(&mut self, t: Time, v: T);
    fn evict(&mut self, t: Time);
    fn query(&self) -> T;
    fn range_query(&self, range: Range<Time>) -> T;
}

pub trait Window<T, O: Operator> {
    fn new() -> Self;
    fn insert(&mut self, v: T);
    fn evict(&mut self);
    fn query(&self) -> T;
}

pub trait MultiWindow<T, O: Operator> {
    fn new(ranges: &[Range<Count>]) -> Self;
    fn insert(&mut self, v: T);
}

pub trait FunctionalWindow<T, O: Operator> {
    fn new() -> Self;
    fn insert(&mut self, v: T) -> Self;
    fn evict(&mut self) -> Self;
    fn query(&self) -> T;
}

// Finger Binary Aggregator
pub mod fiba;
// Two-Stacks
pub mod two_stacks;
// Subtract-On-Evict
pub mod soe;
// Recalculate-From-Scratch
pub mod rfs;
// Functional Okasaki Aggregator
pub mod foa;
// De-Amortized Banker's Aggregator
pub mod daba;
// Slide Side (TwoStacks with shared windows)
pub mod slide_side;
