use alga::general::AbstractMagma;
use alga::general::AbstractMonoid;
use alga::general::Identity;
use alga::general::Operator;

use crate::flat_fat::fat::FAT;
use crate::flat_fat::flat_fat::FlatFAT;
use crate::FifoWindow;

#[derive(Debug)]
pub struct RA<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone + std::fmt::Debug,
    BinOp: Operator,
{
    fat: FlatFAT<Value, BinOp>,
    size: usize,
    front: usize,
    back: usize,
}

impl<Value, BinOp> RA<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone + std::fmt::Debug,
    BinOp: Operator + std::fmt::Debug,
{
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            fat: FlatFAT::with_capacity(capacity),
            size: 0,
            front: 0,
            back: 0,
        }
    }
    fn inverted(&self) -> bool {
        return self.front > self.back;
    }
    fn resize(&mut self, capacity: usize) {
        let leaves = self.fat.leaves(0..self.size);
        let mut fat = FlatFAT::with_capacity(capacity);
        if self.inverted() {
            fat.update_ordered(&leaves[self.front..]);
            fat.update_ordered(&leaves[..self.back]);
        } else {
            fat.update_ordered(&leaves[self.front..self.back]);
        }
        self.fat = fat;
        self.fat.update_parents();
        self.front = 0;
        self.back = self.size;
    }
}

impl<Value, BinOp> FifoWindow<Value, BinOp> for RA<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone + std::fmt::Debug,
    BinOp: Operator + std::fmt::Debug,
{
    fn new() -> Self {
        Self {
            fat: FlatFAT::with_capacity(0),
            size: 0,
            front: 0,
            back: 0,
        }
    }
    fn push(&mut self, v: Value) {
        self.fat.update(&[(self.back, v)]);
        self.size += 1;
        self.back += 1;
        if self.size > (3 * self.fat.capacity) / 4 {
            self.resize(self.fat.capacity * 2);
        }
    }
    fn pop(&mut self) {
        self.fat.update(&[(self.front, Value::identity())]);
        self.size -= 1;
        self.front += 1;
        if self.size <= self.fat.capacity / 4 {
            self.resize(self.fat.capacity / 2);
        }
    }
    fn query(&self) -> Value {
        if self.front > self.back {
            self.fat
                .suffix(self.front)
                .operate(&self.fat.prefix(self.back))
        } else {
            self.fat.aggregate()
        }
    }
}
