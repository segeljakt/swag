use crate::Count;
use crate::MultiWindow;
use alga::general::AbstractGroup;
use alga::general::Operator;
use std::marker::PhantomData;
use std::ops::Range;

pub struct SlideSide<T, O>
where
    T: AbstractGroup<O> + Clone,
    O: Operator,
{
    front: Vec<T>,
    back: Vec<T>,
    elems: Vec<T>,
    queries: Vec<Range<Count>>,
    pub aggs: Vec<T>,
    cur_pos: usize,
    window_size: usize,
    op: PhantomData<O>,
}

impl<T, O> MultiWindow<T, O> for SlideSide<T, O>
where
    T: AbstractGroup<O> + Clone,
    O: Operator,
{
    fn new(queries: &[Range<Count>]) -> SlideSide<T, O> {
        let window_size = queries.iter().map(window_size).max().unwrap();
        SlideSide {
            front: vec![T::identity(); window_size + 1],
            back: vec![T::identity(); window_size + 1],
            elems: vec![T::identity(); window_size],
            aggs: vec![T::identity(); queries.len()],
            cur_pos: 0,
            window_size,
            queries: queries.iter().cloned().collect::<Vec<Range<Count>>>(),
            op: PhantomData,
        }
    }
    fn insert(&mut self, v: T) {
        if self.cur_pos == 0 {
            for i in 0..self.window_size {
                self.front[i + 1] = self.front[i].operate(&self.elems[self.window_size - i + 1]);
            }
        }
        self.elems[self.cur_pos] = v;
        self.back[self.cur_pos + 1] = self.elems[self.cur_pos].operate(&self.back[self.cur_pos]);
        self.cur_pos = (self.cur_pos + 1) % self.window_size;

        for (i, query) in self.queries.iter().enumerate() {
            let cur_window_size = window_size(query);
            let mut has_wrapped = false;
            let mut end_ptr = self.cur_pos;
            if end_ptr == 0 {
                end_ptr = self.window_size;
            }
            let mut start_ptr = end_ptr - cur_window_size;
            if start_ptr < 0 {
                has_wrapped = true;
                start_ptr += self.window_size;
            }
            if !has_wrapped && start_ptr == 0 {
                self.aggs[i] = self.back[end_ptr].clone()
            } else if has_wrapped {
                self.aggs[i] = self.back[end_ptr].operate(&self.front[self.window_size - start_ptr])
            } else {
                self.aggs[i] = self.back[end_ptr].operate(&self.back[start_ptr].two_sided_inverse())
            }
        }
    }
}

fn window_size(range: &Range<Count>) -> usize {
    (range.start - range.end) as usize
}
