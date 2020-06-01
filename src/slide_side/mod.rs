use crate::Count;
use crate::MultiWindow;
use alga::general::AbstractGroup;
use alga::general::Operator;
use std::marker::PhantomData;
use std::ops::Range;

pub struct SlideSide<Value, BinOp>
where
    Value: AbstractGroup<BinOp> + Clone,
    BinOp: Operator,
{
    front: Vec<Value>,
    back: Vec<Value>,
    elems: Vec<Value>,
    queries: Vec<Range<Count>>,
    pub aggs: Vec<Value>,
    cur_pos: usize,
    window_size: usize,
    op: PhantomData<BinOp>,
}

impl<Value, BinOp> MultiWindow<Value, BinOp> for SlideSide<Value, BinOp>
where
    Value: AbstractGroup<BinOp> + Clone,
    BinOp: Operator,
{
    fn new(queries: &[Range<Count>]) -> SlideSide<Value, BinOp> {
        let window_size = queries.iter().map(window_size).max().unwrap();
        SlideSide {
            front: vec![Value::identity(); window_size + 1],
            back: vec![Value::identity(); window_size + 1],
            elems: vec![Value::identity(); window_size],
            aggs: vec![Value::identity(); queries.len()],
            cur_pos: 0,
            window_size,
            queries: queries.iter().cloned().collect::<Vec<Range<Count>>>(),
            op: PhantomData,
        }
    }
    fn insert(&mut self, v: Value) {
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
            let mut end_ptr = self.cur_pos as isize;
            if end_ptr == 0 {
                end_ptr = self.window_size as isize;
            }
            let mut start_ptr = end_ptr - cur_window_size as isize;
            if start_ptr < 0 {
                has_wrapped = true;
                start_ptr += self.window_size as isize;
            }
            if !has_wrapped && start_ptr == 0 {
                self.aggs[i] = self.back[end_ptr as usize].clone()
            } else if has_wrapped {
                self.aggs[i] = self.back[end_ptr as usize]
                    .operate(&self.front[self.window_size - start_ptr as usize])
            } else {
                self.aggs[i] = self.back[end_ptr as usize]
                    .operate(&self.back[start_ptr as usize].two_sided_inverse())
            }
        }
    }
}

fn window_size(range: &Range<Count>) -> usize {
    (range.end - range.start) as usize
}
