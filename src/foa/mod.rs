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

struct FOA<T, O>
where
    T: AbstractGroup<O> + Clone,
    O: Operator,
{
    front: List<Elem<T>>,
    next: List<Elem<T>>,
    back: List<Elem<T>>,
    op: PhantomData<O>,
}

impl<T, O> FunctionalWindow<T, O> for FOA<T, O>
where
    T: AbstractGroup<O> + Clone,
    O: Operator,
{
    fn new() -> FOA<T, O> {
        FOA {
            front: List::empty(),
            next: List::empty(),
            back: List::empty(),
            op: PhantomData,
        }
    }
    fn insert(&mut self, v: T) -> FOA<T, O> {
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
    fn evict(&mut self) -> FOA<T, O> {
        FOA {
            front: self.front.tail(),
            next: self.next.clone(),
            back: self.back.clone(),
            op: self.op,
        }
        .makeq()
    }
    fn query(&self) -> T {
        Self::agg(&self.front).operate(&Self::agg(&self.back))
    }
}

impl<T, O> FOA<T, O>
where
    T: AbstractGroup<O> + Clone,
    O: Operator,
{
    fn agg(list: &List<Elem<T>>) -> T {
        list.head().map(|elem| elem.agg).unwrap_or(T::identity())
    }
    fn makeq(&self) -> FOA<T, O> {
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
    fn rot(self) -> List<Elem<T>> {
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
