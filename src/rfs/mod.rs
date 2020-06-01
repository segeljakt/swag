use crate::FifoWindow;
use alga::general::AbstractMonoid;
use alga::general::Operator;
use std::marker::PhantomData;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct RFS<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone,
    BinOp: Operator,
{
    stack: VecDeque<Value>,
    op: PhantomData<BinOp>,
}

impl<Value, BinOp> FifoWindow<Value, BinOp> for RFS<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone,
    BinOp: Operator,
{
    fn new() -> RFS<Value, BinOp> {
        RFS {
            stack: VecDeque::new(),
            op: PhantomData,
        }
    }
    fn push(&mut self, v: Value) {
        self.stack.push_back(v);
    }
    fn pop(&mut self) {
        self.stack.pop_front();
    }
    fn query(&self) -> Value {
        self.stack
            .iter()
            .fold(Value::identity(), |acc, elem| acc.operate(&elem))
    }
}
