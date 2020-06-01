use alga::general::AbstractGroup;
use alga::general::AbstractLoop;
use alga::general::AbstractMagma;
use alga::general::AbstractMonoid;
use alga::general::AbstractQuasigroup;
use alga::general::AbstractSemigroup;
use alga::general::Identity;
use alga::general::Operator;
use alga::general::TwoSidedInverse;
use swag::daba::*;
use swag::rfs::*;
use swag::soe::*;
use swag::two_stacks::*;
use swag::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Value(i32);

#[derive(Copy, Clone, Debug)]
struct Sum;

impl Operator for Sum {
    fn operator_token() -> Sum {
        Sum
    }
}

impl Identity<Sum> for Value {
    fn identity() -> Value {
        Value(0)
    }
}

impl AbstractMagma<Sum> for Value {
    fn operate(&self, other: &Self) -> Self {
        Value(self.0 + other.0)
    }
}

impl TwoSidedInverse<Sum> for Value {
    fn two_sided_inverse(&self) -> Value {
        Value(-self.0)
    }
}

impl AbstractSemigroup<Sum> for Value {}
impl AbstractMonoid<Sum> for Value {}
impl AbstractQuasigroup<Sum> for Value {}
impl AbstractLoop<Sum> for Value {}
impl AbstractGroup<Sum> for Value {}

fn test_simple<Window>(mut window: Window)
where
    Window: FifoWindow<Value, Sum> + std::fmt::Debug,
{
    assert_eq!(window.query(), Value(0));
    window.push(Value(1));
    println!("{:?}", window);
    assert_eq!(window.query(), Value(1));

    window.push(Value(2));
    println!("{:?}", window);
    assert_eq!(window.query(), Value(3));

    window.push(Value(3));
    println!("{:?}", window);
    assert_eq!(window.query(), Value(6));

    window.pop();
    println!("{:?}", window);
    assert_eq!(window.query(), Value(5));
}

#[test]
fn test_simple_rfs() {
    test_simple(RFS::<Value, Sum>::new());
}

#[test]
fn test_simple_soe() {
    test_simple(SOE::<Value, Sum>::new());
}

#[test]
fn test_simple_two_stacks() {
    test_simple(TwoStacks::<Value, Sum>::new());
}

#[test]
fn test_simple_daba() {
    test_simple(DABA::<Value, Sum>::new());
}
