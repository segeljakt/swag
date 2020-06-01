use crate::FifoWindow;
use alga::general::AbstractGroup;
use alga::general::Operator;
use std::collections::VecDeque;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct SOE<Value, BinOp>
where
    Value: AbstractGroup<BinOp> + Clone,
    BinOp: Operator,
{
    stack: VecDeque<Value>,
    agg: Value,
    op: PhantomData<BinOp>,
}

impl<Value, BinOp> FifoWindow<Value, BinOp> for SOE<Value, BinOp>
where
    Value: AbstractGroup<BinOp> + Clone,
    BinOp: Operator,
{
    fn new() -> SOE<Value, BinOp> {
        SOE {
            stack: VecDeque::new(),
            agg: Value::identity(),
            op: PhantomData,
        }
    }
    fn push(&mut self, v: Value) {
        self.agg = self.agg.operate(&v);
        self.stack.push_back(v);
    }
    fn pop(&mut self) {
        if let Some(top) = self.stack.pop_front() {
            self.agg = self.agg.operate(&top.two_sided_inverse());
        }
    }
    fn query(&self) -> Value {
        self.agg.clone()
    }
}
