use alga::general::AbstractMonoid;
use alga::general::AbstractMagma;
use alga::general::Operator;
use alga::general::Identity;

use crate::TimeWindow;
use crate::flat_fat::fat::FAT;
use crate::flat_fat::flat_fat::FlatFAT;
use crate::flat_fat::item::{Item,Combine};

pub struct TimeRA<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone+std::fmt::Debug,
    BinOp: Operator + std::fmt::Debug,
{
    fat: FlatFAT<Item<Value, BinOp>, Combine>,
    size: usize,
    front: usize,
    back: usize,
}
impl<Value, BinOp> TimeRA<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone+std::fmt::Debug,
    BinOp: Operator + std::fmt::Debug,
{
    fn with_capacity(capacity: usize) -> Self {
        TimeRA {
            fat: FlatFAT::with_capacity(capacity),
            size: 0,
            front: 0,
            back: 0,
        }
    }
    fn resize(&mut self) {
        
    }
}

impl<Value, BinOp> TimeWindow<usize, Value, BinOp> for TimeRA<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone+std::fmt::Debug,
    BinOp: Operator + std::fmt::Debug,
{
    fn new() -> Self {
        TimeRA {
            fat: FlatFAT::with_capacity(0),
            size: 0,
            front: 0,
            back: 0,
        }
    }
    fn insert(&mut self, t: usize, v: Value) {
        self.size += 1;
        if self.size <= (3 * self.fat.capacity) / 4 {
            
        }
    }
    fn evict(&mut self, t: usize) {
        self.fat.update(&[(t, Item::identity())]);
        self.size -= 1;
        if self.size <= self.fat.capacity / 4 {
        }
    }
    fn query(&self) -> Value {
        if self.front < self.back {
            self.fat.suffix(self.front).operate(&self.fat.prefix(self.back))
        } else {
            self.fat.aggregate()
        }.get_value()
    }
}
