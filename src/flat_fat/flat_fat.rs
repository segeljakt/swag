use crate::flat_fat::fat::FAT;
use alga::general::AbstractMonoid;
use alga::general::Operator;

#[derive(Debug)]
pub struct FlatFAT<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone + std::fmt::Debug,
    BinOp: Operator,
{
    /// A flat binary tree, indexed as:
    ///       0
    ///      / \
    ///     /   \
    ///    1     2
    ///   / \   / \
    ///  3   4 5   6
    pub(crate) tree: Vec<Value>,
    /// Number of leaves which can be stored in the tree
    pub(crate) capacity: usize,
    binop: std::marker::PhantomData<BinOp>,
}

impl<Value, BinOp> FlatFAT<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone + std::fmt::Debug,
    BinOp: Operator,
{
    /// Returns all leaf nodes of the tree
    pub(crate) fn leaves(&self, range: std::ops::Range<usize>) -> &[Value] {
        &self.tree[self.leaf(range.start)..self.leaf(range.end)]
    }
    /// Returns the index of the root node
    fn root(&self) -> usize {
        0
    }
    /// Returns the index of a leaf
    fn leaf(&self, i: usize) -> usize {
        i + self.capacity - 1
    }
    /// Returns the index of an node's left child
    fn left(&self, i: usize) -> usize {
        2 * (i + 1) - 1
    }
    /// Returns the index of an node's right child
    fn right(&self, i: usize) -> usize {
        2 * (i + 1)
    }
    /// Returns the index of an node's parent
    fn parent(&self, i: usize) -> usize {
        (i - 1) / 2
    }
}

impl<Value, BinOp> FAT<Value, BinOp> for FlatFAT<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone + std::fmt::Debug,
    BinOp: Operator,
{
    /// Creates an aggregate binary tree from a list of values
    fn new(values: &[Value]) -> Self {
        let capacity = values.len();
        let mut new = Self::with_capacity(capacity);
        new.update_ordered(values);
        new.update_parents();
        new
    }
    /// Creates an empty-window from a list of values
    fn with_capacity(capacity: usize) -> Self {
        assert_ne!(capacity, 0, "Capacity of window must be greater than 0");
        Self {
            tree: vec![Value::identity(); 2 * capacity - 1],
            binop: std::marker::PhantomData,
            capacity,
        }
    }
    /// Adds a batch of values to the window
    fn update(&mut self, batch: &[(usize, Value)]) {
        for (idx, val) in batch {
            let leaf = self.leaf(*idx);
            self.tree[leaf] = val.clone();
        }
        let mut parents: Vec<usize> = batch
            .iter()
            .map(|&(idx, _)| self.parent(self.leaf(idx)))
            .collect();
        let mut new_parents: Vec<usize> = Vec::new();
        loop {
            for parent in parents.drain(..) {
                let left = self.left(parent);
                let right = self.right(parent);
                self.tree[parent] = self.tree[left].operate(&self.tree[right]);
                if parent != self.root() {
                    new_parents.push(self.parent(parent));
                }
            }
            if new_parents.is_empty() {
                break;
            } else {
                std::mem::swap(&mut parents, &mut new_parents);
            }
        }
    }
    fn update_ordered(&mut self, values: &[Value]) {
        for (idx, val) in values.iter().enumerate() {
            let leaf = self.leaf(idx);
            self.tree[leaf] = val.clone();
        }
    }
    fn update_parents(&mut self) {
        for parent in (0..self.capacity - 1).into_iter().rev() {
            let left = self.left(parent);
            let right = self.right(parent);
            self.tree[parent] = self.tree[left].operate(&self.tree[right]);
        }
    }
    fn aggregate(&self) -> Value {
        self.tree[self.root()].clone()
    }
    fn prefix(&self, idx: usize) -> Value {
        let mut node = self.leaf(idx);
        let mut agg = self.tree[node].clone();
        while node != self.root() {
            let parent = self.parent(node);
            if node == self.right(parent) {
                let left = self.left(parent);
                agg = self.tree[left].operate(&agg);
            }
            node = parent;
        }
        return agg;
    }
    fn suffix(&self, i: usize) -> Value {
        let mut node = self.leaf(i);
        let mut agg = self.tree[node].clone();
        while node != self.root() {
            let parent = self.parent(node);
            if node == self.left(parent) {
                agg = agg.operate(&self.tree[self.right(parent)]);
            }
            node = parent;
        }
        return agg;
    }
}
