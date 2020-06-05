use alga::general::AbstractMagma;
use alga::general::AbstractMonoid;
use alga::general::AbstractSemigroup;
use alga::general::Identity;
use alga::general::Operator;

/// An item of an out-of-order window which may or may not yet be assigned.
/// Used for representing un-assigned values of the Reactive Aggregator.
#[derive(Clone, Eq, Debug)]
pub struct Item<Value, BinOp>
where
    Value: Identity<BinOp> + std::fmt::Debug,
    BinOp: Operator + std::fmt::Debug,
{
    value: Option<Value>,
    binop: std::marker::PhantomData<BinOp>,
}

impl<Value, BinOp> Item<Value, BinOp>
where
    Value: Identity<BinOp> + Clone + std::fmt::Debug,
    BinOp: Operator + std::fmt::Debug,
{
    fn new(value: Option<Value>) -> Self {
        Self {
            value,
            binop: std::marker::PhantomData,
        }
    }
    pub fn get_value(&self) -> Value {
        self.value
            .as_ref()
            .map(|v| v.clone())
            .unwrap_or(Value::identity())
    }
}

impl<Value, BinOp> PartialEq for Item<Value, BinOp>
where
    Value: Identity<BinOp> + PartialEq + std::fmt::Debug,
    BinOp: Operator + std::fmt::Debug,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Combine;

impl Operator for Combine {
    fn operator_token() -> Combine {
        Combine
    }
}

impl<Value, BinOp> Identity<Combine> for Item<Value, BinOp>
where
    Value: Identity<BinOp> + Clone + std::fmt::Debug,
    BinOp: Operator + std::fmt::Debug,
{
    fn identity() -> Item<Value, BinOp> {
        Item::new(None)
    }
}

impl<Value, BinOp> AbstractMagma<Combine> for Item<Value, BinOp>
where
    Value: Identity<BinOp> + AbstractMagma<BinOp> + Clone + std::fmt::Debug,
    BinOp: Operator + std::fmt::Debug,
{
    fn operate(&self, other: &Self) -> Self {
        Item::new(match (self.value.as_ref(), other.value.as_ref()) {
            (Some(a), Some(b)) => Some(a.operate(&b)),
            (Some(x), None) | (None, Some(x)) => Some(x.clone()),
            (None, None) => None,
        })
    }
}

impl<Value, BinOp> AbstractSemigroup<Combine> for Item<Value, BinOp>
where
    Value: AbstractSemigroup<BinOp> + Identity<BinOp> + std::fmt::Debug,
    BinOp: Operator + std::fmt::Debug,
{
}

impl<Value, BinOp> AbstractMonoid<Combine> for Item<Value, BinOp>
where
    Value: AbstractMonoid<BinOp> + std::fmt::Debug,
    BinOp: Operator + std::fmt::Debug,
{
}
