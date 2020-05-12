use crate::Window;
use alga::general::AbstractGroup;
use alga::general::Operator;
use std::collections::VecDeque;
use std::marker::PhantomData;

struct SOE<T, O>
where
    T: AbstractGroup<O> + Clone,
    O: Operator,
{
    stack: VecDeque<T>,
    agg: T,
    op: PhantomData<O>,
}

impl<T, O> Window<T, O> for SOE<T, O>
where
    T: AbstractGroup<O> + Clone,
    O: Operator,
{
    fn new() -> SOE<T, O> {
        SOE {
            stack: VecDeque::new(),
            agg: T::identity(),
            op: PhantomData,
        }
    }
    fn insert(&mut self, v: T) {
        self.agg = self.agg.operate(&v);
        self.stack.push_back(v);
    }
    fn evict(&mut self) {
        if let Some(top) = self.stack.pop_front() {
            self.agg = self.agg.operate(&top.two_sided_inverse());
        }
    }
    fn query(&self) -> T {
        self.agg.clone()
    }
}
