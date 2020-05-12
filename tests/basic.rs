use alga::general::AbstractMagma;
use alga::general::AbstractMonoid;
use alga::general::AbstractSemigroup;
use alga::general::Identity;
use alga::general::Operator;
use swag::fiba::*;
use swag::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Value(i32);

#[derive(Copy, Clone)]
struct BinOp;

impl Operator for BinOp {
    fn operator_token() -> BinOp {
        BinOp
    }
}

impl Identity<BinOp> for Value {
    fn identity() -> Value {
        Value(0)
    }
}

impl AbstractMagma<BinOp> for Value {
    fn operate(&self, other: &Self) -> Self {
        Value(self.0 + other.0)
    }
}

impl AbstractSemigroup<BinOp> for Value {}
impl AbstractMonoid<BinOp> for Value {}

#[test]
fn fiba_lifo() {
    let mut tree: FIBA<Value, BinOp> = FIBA::new();
    let count = 100;
    for i in 1..=count {
        tree.insert(i, Value(1));
        assert_eq!(tree.query(), Value(i as i32));
    }
    for i in (1..=100).rev() {
        tree.evict(i);
        assert_eq!(tree.query(), Value(i as i32 - 1));
    }
}

#[test]
fn fiba_fifo() {
    let mut tree: FIBA<Value, BinOp> = FIBA::new();
    let count = 15;
    for i in 1..=count {
        tree.insert(i, Value(1));
        assert_eq!(tree.query(), Value(i as i32));
    }
    for i in 1..=count {
        tree.evict(i);
        assert_eq!(tree.query(), Value((count - i) as i32));
    }
}

#[test]
fn fiba_random_access() {
    let mut tree: FIBA<Value, BinOp> = FIBA::new();
    const COUNT: usize = 10;

    let times: [Time; COUNT] = [10, 0, 1, 9, -1, 4, 20, 3, -10, -30];
    for (i, &time) in times.iter().enumerate() {
        assert_eq!(tree.query(), Value(i as i32));
        tree.insert(time, Value(1));
    }
    for (i, &time) in times.iter().enumerate() {
        assert_eq!(tree.query(), Value((COUNT - i) as i32));
        tree.evict(time);
    }
    for (i, &time) in times.iter().enumerate() {
        assert_eq!(tree.query(), Value(i as i32));
        tree.insert(time, Value(1));
    }
}

#[test]
fn fiba_range_query() {
    let mut tree: FIBA<Value, BinOp> = FIBA::new();
    let count = 15;
    for i in 1..=count {
        tree.insert(i, Value(1));
        assert_eq!(tree.query(), Value(i as i32));
    }

    assert_eq!(Value(10), tree.range_query(0..10));
}

#[test]
fn fiba_big_query() {
    let mut tree: FIBA<Value, BinOp> = FIBA::new();
    let timestamps = (2 as Time).pow(22);
    for timestamp in 0..timestamps {
        tree.insert(timestamp, Value(1));
    }
    for _ in 0..10 {
        for exponent in 0..22 {
            let d = (2 as Time).pow(exponent);
            tree.insert_test(d, Value(0));
            tree.evict(d);
            tree.query();
        }
    }
}
