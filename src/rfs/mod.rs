use crate::Window;
use alga::general::AbstractMonoid;
use alga::general::Operator;
use std::marker::PhantomData;

struct RFS<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone,
    BinOp: Operator,
{
    stack: Vec<Value>,
    op: PhantomData<BinOp>,
}

impl<Value, BinOp> Window<Value, BinOp> for RFS<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone,
    BinOp: Operator,
{
    fn new() -> RFS<Value, BinOp> {
        RFS {
            stack: Vec::new(),
            op: PhantomData,
        }
    }
    fn insert(&mut self, v: Value) {
        self.stack.push(v);
    }
    fn evict(&mut self) {
        self.stack.pop();
    }
    fn query(&self) -> Value {
        self.stack
            .iter()
            .fold(Value::identity(), |acc, elem| acc.operate(&elem))
    }
}
