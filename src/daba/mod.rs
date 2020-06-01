#![allow(unused)]
mod chunked_array_queue;

use crate::FifoWindow;
use alga::general::AbstractMonoid;
use alga::general::Operator;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct DABA<T, O>
where
    T: Debug + AbstractMonoid<O> + Clone,
    O: Debug + Operator,
{
    // ith oldest value in FIFO order stored at vi = vals[i]
    vals: VecDeque<T>,
    aggs: VecDeque<T>,
    // 0 ≤ l ≤ r ≤ a ≤ b ≤ aggs.len()
    l: usize, // Left,  ∀p ∈ l...r−1 : aggs[p] = vals[p] ⊕ ... ⊕ vals[r−1]
    r: usize, // Right, ∀p ∈ r...a−1 : aggs[p] = vals[R] ⊕ ... ⊕ vals[p]
    a: usize, // Accum, ∀p ∈ a...b−1 : aggs[p] = vals[p] ⊕ ... ⊕ vals[b−1]
    b: usize, // Back,  ∀p ∈ b...e−1 : aggs[p] = vals[B] ⊕ ... ⊕ vals[p]
    op: PhantomData<O>,
}

impl<Value, BinOp> FifoWindow<Value, BinOp> for DABA<Value, BinOp>
where
    Value: Debug + AbstractMonoid<BinOp> + Clone,
    BinOp: Debug + Operator,
{
    fn new() -> DABA<Value, BinOp> {
        DABA {
            vals: VecDeque::new(),
            aggs: VecDeque::new(),
            l: 0,
            r: 0,
            a: 0,
            b: 0,
            op: PhantomData,
        }
    }
    fn push(&mut self, v: Value) {
        self.aggs.push_back(self.agg_b().operate(&v));
        self.vals.push_back(v);
        self.fixup();
    }
    fn pop(&mut self) {
        if let Some(_) = self.vals.pop_front() {
            self.aggs.pop_front();
            self.l -= 1;
            self.r -= 1;
            self.a -= 1;
            self.b -= 1;
            self.fixup();
        }
    }
    fn query(&self) -> Value {
        self.agg_f().operate(&self.agg_b())
    }
}

impl<T, O> DABA<T, O>
where
    T: Debug + AbstractMonoid<O> + Clone,
    O: Debug + Operator,
{
    #[inline(always)]
    fn agg_f(&self) -> T {
        if self.aggs.is_empty() {
            T::identity()
        } else {
            self.aggs.front().unwrap().clone()
        }
    }
    #[inline(always)]
    fn agg_b(&self) -> T {
        if self.b == self.aggs.len() {
            T::identity()
        } else {
            self.aggs.back().unwrap().clone()
        }
    }
    #[inline(always)]
    fn agg_l(&self) -> T {
        if self.l == self.r {
            T::identity()
        } else {
            self.aggs[self.l].clone()
        }
    }
    #[inline(always)]
    fn agg_r(&self) -> T {
        if self.r == self.a {
            T::identity()
        } else {
            self.aggs[self.a - 1].clone()
        }
    }
    #[inline(always)]
    fn agg_a(&self) -> T {
        if self.a == self.b {
            T::identity()
        } else {
            self.aggs[self.a].clone()
        }
    }
    fn fixup(&mut self) {
        if self.b == 0 {
            self.singleton()
        } else {
            if self.l == self.b {
                self.flip()
            }
            if self.l == self.r {
                self.shift()
            } else {
                self.shrink()
            }
        }
    }
    // If F == B, that means the front list lF is empty. As Section 4.4 will show,
    // that can only happen if the back list lB has exactly one element. On a
    // singleton list, there is no difference between aggregating to the left or
    // right. Therefore, DABA can simply move pointers around to turn lB into lF
    // without having to modify aggs.
    #[inline(always)]
    fn singleton(&mut self) {
        self.l = self.aggs.len();
        self.r = self.l;
        self.a = self.l;
        self.b = self.l;
    }
    // If F != B but L == B, that means that the three sublists lL, lR, and lA of lF
    // are all empty. In that case, the entire lF is aggregated to the left •− and
    // lB is aggregated to the right −•. So DABA can simply move pointers around to
    // turn lF and lB into lL and lR , which are aggregated in the same directions.
    #[inline(always)]
    fn flip(&mut self) {
        self.l = 0;
        self.a = self.aggs.len();
        self.b = self.a;
    }
    // If L != B but L == R, that means that lL and lR are empty but lA is non-empty.
    // That means that all of lL is aggregated to the left •−. In other words, the
    // boundary of lA makes no difference for the aggregation, and DABA can increment
    // L, R, and A without having to modify aggs.
    #[inline(always)]
    fn shift(&mut self) {
        self.a += 1;
        self.r += 1;
        self.l += 1;
    }
    // If L != R, that means that lL is non-empty. As Section 4.4 will show, lL and lR
    // always have the same length, so lR is non-empty too. DABA shrinks both lL and lR
    // by one element each. This is the only case that requires modifying aggs. DABA
    // shrinks lL by incrementing L, thus moving the first element of lL to the front
    // portion of lL ; the new aggs entry for that element is ΣL⊕ ⊕ ΣR⊕ ⊕ ΣA⊕. DABA
    // shrinks lR by decrementing A, thus moving the last element of lR to lA; the new
    // aggs entry for that element is vals[A−1] ⊕ ΣA⊕ .
    #[inline(always)]
    fn shrink(&mut self) {
        self.aggs[self.l] = self.agg_l().operate(&self.agg_r()).operate(&self.agg_a());
        self.l += 1;
        self.aggs[self.a - 1] = self.vals[self.a - 1].operate(&self.agg_a());
        self.a -= 1;
    }
}
