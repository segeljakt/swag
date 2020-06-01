mod stack;
use stack::List;

use crate::FunctionalWindow;
use alga::general::AbstractGroup;
use alga::general::Operator;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Elem<T> {
    val: T,
    agg: T,
}

struct FOA<Value, BinOp>
where
    Value: AbstractGroup<BinOp> + Clone,
    BinOp: Operator,
{
    front: List<Elem<Value>>,
    next: List<Elem<Value>>,
    back: List<Elem<Value>>,
    op: PhantomData<BinOp>,
}

impl<Value, BinOp> FunctionalWindow<Value, BinOp> for FOA<Value, BinOp>
where
    Value: AbstractGroup<BinOp> + Clone,
    BinOp: Operator,
{
    fn new() -> FOA<Value, BinOp> {
        FOA {
            front: List::empty(),
            next: List::empty(),
            back: List::empty(),
            op: PhantomData,
        }
    }
    fn insert(&mut self, v: Value) -> FOA<Value, BinOp> {
        FOA {
            front: self.front.clone(),
            next: self.next.clone(),
            back: self.back.cons(Elem {
                agg: Self::agg(&self.back).operate(&v),
                val: v,
            }),
            op: self.op,
        }
        .makeq()
    }
    fn evict(&mut self) -> FOA<Value, BinOp> {
        FOA {
            front: self.front.tail(),
            next: self.next.clone(),
            back: self.back.clone(),
            op: self.op,
        }
        .makeq()
    }
    fn query(&self) -> Value {
        Self::agg(&self.front).operate(&Self::agg(&self.back))
    }
}

impl<Value, BinOp> FOA<Value, BinOp>
where
    Value: AbstractGroup<BinOp> + Clone,
    BinOp: Operator,
{
    fn agg(list: &List<Elem<Value>>) -> Value {
        list.head().map(|elem| elem.agg).unwrap_or(Value::identity())
    }
    fn makeq(&self) -> FOA<Value, BinOp> {
        if self.next.is_empty() {
            let front = Self::rot(FOA {
                front: self.front.clone(),
                next: self.back.clone(),
                back: List::empty(),
                op: self.op,
            });
            FOA {
                next: front.clone(),
                front,
                back: List::empty(),
                op: self.op,
            }
        } else {
            FOA {
                front: self.front.clone(),
                next: self.next.tail(),
                back: self.back.clone(),
                op: self.op,
            }
        }
    }
    fn rot(self) -> List<Elem<Value>> {
        let back = self.back.cons(
            self.next
                .head()
                .map(|mut elem| {
                    elem.agg = elem.val.operate(&Self::agg(&self.back));
                    elem
                })
                .unwrap(),
        );
        if self.front.is_empty() {
            back
        } else {
            FOA {
                front: self.front.tail(),
                next: self.next.tail(),
                back,
                op: self.op,
            }
            .rot()
            .cons(
                self.front
                    .head()
                    .map(|mut elem| {
                        elem.agg = elem
                            .val
                            .operate(&Self::agg(&self.next))
                            .operate(&Self::agg(&self.back));
                        elem
                    })
                    .unwrap(),
            )
        }
    }
}
