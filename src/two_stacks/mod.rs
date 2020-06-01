use crate::Window;
use alga::general::AbstractMonoid;
use alga::general::Operator;
use std::marker::PhantomData;

struct Item<Value: Clone> {
    agg: Value,
    val: Value,
}

impl<Value: Clone> Item<Value> {
    fn new(agg: Value, val: Value) -> Item<Value> {
        Item { agg, val }
    }
}

pub struct TwoStacks<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone,
    BinOp: Operator,
{
    front: Vec<Item<Value>>,
    back: Vec<Item<Value>>,
    op: PhantomData<BinOp>,
}

impl<Value, BinOp> Window<Value, BinOp> for TwoStacks<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone,
    BinOp: Operator,
{
    fn new() -> TwoStacks<Value, BinOp> {
        TwoStacks {
            front: Vec::new(),
            back: Vec::new(),
            op: PhantomData,
        }
    }
    fn insert(&mut self, v: Value) {
        self.back
            .push(Item::new(Self::agg(&self.back).operate(&v), v));
    }
    fn evict(&mut self) {
        if self.front.is_empty() {
            while let Some(top) = self.back.pop() {
                self.front
                    .push(Item::new(top.val.operate(&Self::agg(&self.front)), top.val))
            }
        }
        self.front.pop();
    }
    fn query(&self) -> Value {
        Self::agg(&self.front).operate(&Self::agg(&self.back))
    }
}

impl<T, O> TwoStacks<T, O>
where
    T: AbstractMonoid<O> + Clone,
    O: Operator,
{
    #[inline(always)]
    fn agg(stack: &Vec<Item<T>>) -> T {
        if let Some(top) = stack.last() {
            top.agg.clone()
        } else {
            T::identity()
        }
    }
}
