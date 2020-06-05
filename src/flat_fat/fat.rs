use alga::general::Operator;
use alga::general::AbstractMonoid;

pub trait FAT<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + Clone,
    BinOp: Operator,
{
    /// Returns a window from a batch of values
    fn new(batch: &[Value]) -> Self;
    /// Returns a window with uninitialized values
    fn with_capacity(capacity: usize) -> Self;
    /// Updates a batch of leaves in a window
    fn update(&mut self, batch: &[(usize, Value)]);
    /// Updates a contiguous array of leaves
    fn update_ordered(&mut self, batch: &[Value]);
    /// Updates all parents
    fn update_parents(&mut self);
    fn aggregate(&self) -> Value;
    fn prefix(&self, i: usize) -> Value;
    fn suffix(&self, i: usize) -> Value;
}
