use alga::general::Operator;
use std::ops::Range;

pub type Count = i32;
pub(crate) type Uid = u32;

// const NEG_INFINITY: Time = i32::MIN;
// const POS_INFINITY: Time = i32::MAX;

pub trait TimeWindow<Time, Value, BinOp>
where
    Time: Ord,
    BinOp: Operator,
{
    fn new() -> Self;
    fn insert(&mut self, t: Time, v: Value);
    fn evict(&mut self, t: Time);
    fn query(&self) -> Value;
    fn range_query(&self, range: Range<Time>) -> Value;
}

pub trait Window<Value, BinOp>
where
    BinOp: Operator,
{
    fn new() -> Self;
    fn insert(&mut self, v: Value);
    fn evict(&mut self);
    fn query(&self) -> Value;
}

pub trait MultiWindow<Value, BinOp>
where
    BinOp: Operator,
{
    fn new(ranges: &[Range<Count>]) -> Self;
    fn insert(&mut self, v: Value);
}

pub trait FunctionalWindow<Value, BinOp>
where
    BinOp: Operator,
{
    fn new() -> Self;
    fn insert(&mut self, v: Value) -> Self;
    fn evict(&mut self) -> Self;
    fn query(&self) -> Value;
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
