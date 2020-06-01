use crate::Window;
use alga::general::AbstractGroup;
use alga::general::Operator;
use std::collections::VecDeque;
use std::marker::PhantomData;

struct SOE<Value, BinOp>
where
    Value: AbstractGroup<BinOp> + Clone,
    BinOp: Operator,
{
    stack: VecDeque<Value>,
    agg: Value,
    op: PhantomData<BinOp>,
}

impl<Value, BinOp> Window<Value, BinOp> for SOE<Value, BinOp>
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
    fn insert(&mut self, v: Value) {
        self.agg = self.agg.operate(&v);
        self.stack.push_back(v);
    }
    fn evict(&mut self) {
        if let Some(top) = self.stack.pop_front() {
            self.agg = self.agg.operate(&top.two_sided_inverse());
        }
    }
    fn query(&self) -> Value {
        self.agg.clone()
    }
}
