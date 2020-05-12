use crate::Window;
use alga::general::AbstractMonoid;
use alga::general::Operator;
use std::marker::PhantomData;

struct Item<T: Clone> {
    agg: T,
    val: T,
}

impl<T: Clone> Item<T> {
    fn new(agg: T, val: T) -> Item<T> {
        Item { agg, val }
    }
}

pub struct TwoStacks<T, O>
where
    T: AbstractMonoid<O> + Clone,
    O: Operator,
{
    front: Vec<Item<T>>,
    back: Vec<Item<T>>,
    op: PhantomData<O>,
}

impl<T, O> Window<T, O> for TwoStacks<T, O>
where
    T: AbstractMonoid<O> + Clone,
    O: Operator,
{
    fn new() -> TwoStacks<T, O> {
        TwoStacks {
            front: Vec::new(),
            back: Vec::new(),
            op: PhantomData,
        }
    }
    fn insert(&mut self, v: T) {
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
    fn query(&self) -> T {
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
