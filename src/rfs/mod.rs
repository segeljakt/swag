use crate::Window;
use alga::general::AbstractMonoid;
use alga::general::Operator;
use std::marker::PhantomData;

struct RFS<T, O>
where
    T: AbstractMonoid<O> + Clone,
    O: Operator,
{
    stack: Vec<T>,
    op: PhantomData<O>,
}

impl<T, O> Window<T, O> for RFS<T, O>
where
    T: AbstractMonoid<O> + Clone,
    O: Operator,
{
    fn new() -> RFS<T, O> {
        RFS {
            stack: Vec::new(),
            op: PhantomData,
        }
    }
    fn insert(&mut self, v: T) {
        self.stack.push(v);
    }
    fn evict(&mut self) {
        self.stack.pop();
    }
    fn query(&self) -> T {
        self.stack
            .iter()
            .fold(T::identity(), |acc, elem| acc.operate(&elem))
    }
}
