use alga::general::AbstractMagma;
use alga::general::AbstractMonoid;
use alga::general::AbstractSemigroup;
use alga::general::Identity;
use alga::general::Operator;
use swag::daba::*;
use swag::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Value(i32);

#[derive(Copy, Clone, Debug)]
struct Max;

impl Operator for Max {
    fn operator_token() -> Max {
        Max
    }
}

impl Identity<Max> for Value {
    fn identity() -> Value {
        Value(0)
    }
}

impl AbstractMagma<Max> for Value {
    fn operate(&self, other: &Self) -> Self {
        Value(i32::max(self.0, other.0))
    }
}

impl AbstractSemigroup<Max> for Value {}
impl AbstractMonoid<Max> for Value {}

#[test]
fn daba() {
    let mut window = DABA::<Value, Max>::new();

    window.insert(Value(3));
    assert_eq!(window.query(), Value(3));

    window.insert(Value(4));
    assert_eq!(window.query(), Value(4));

    window.insert(Value(5));
    println!("{:?}", window);
    assert_eq!(window.query(), Value(5));

    window.evict();
    println!("{:?}", window);
    assert_eq!(window.query(), Value(4));
}
