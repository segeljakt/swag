use crate::fiba::{Item, Node, FIBA};
use alga::general::{AbstractMonoid, Operator};
use itertools::*;

pub trait Pretty {
    fn pretty(&self, indent: usize) -> String;
    fn indent(indent: usize) -> String {
        const TAB: &str = "  ";
        format!(
            "\n{}",
            (0..indent).into_iter().map(|_| TAB).collect::<String>()
        )
    }
}

impl<Time, Value, BinOp> Pretty for FIBA<Time, Value, BinOp>
where
    Time: Ord + std::fmt::Display,
    Value: AbstractMonoid<BinOp> + 'static,
    BinOp: Operator,
{
    fn pretty(&self, indent: usize) -> String {
        unsafe {
            format!(
                "Fingers(LEFT=Node{left},RIGHT=Node{right})\n{root}",
                left = self.left_finger.as_ref().uid,
                right = self.right_finger.as_ref().uid,
                root = self.root.pretty(indent)
            )
        }
    }
}

impl<Time, Value, BinOp> Pretty for Node<Time, Value, BinOp>
where
    Time: std::fmt::Display,
{
    fn pretty(&self, indent: usize) -> String {
        let members = self
            .children
            .iter()
            .map(|child| child.pretty(indent + 1))
            .interleave(self.items.iter().map(|item| item.pretty(indent + 1)))
            .collect::<Vec<String>>()
            .join(&format!(",{}", Self::indent(indent + 1)));
        format!(
            "Node{uid}^{parent}<{left},{right}>:{sum:?}Σ[{s1}{members}{s0}]",
            uid = self.uid,
            parent = self
                .parent
                .as_ref()
                .map(|parent| unsafe { parent.as_ref().uid })
                .unwrap_or(0),
            sum = 0,
            //             sum = self.agg,
            members = members,
            left = self.spine.left,
            right = self.spine.right,
            s0 = Self::indent(indent),
            s1 = Self::indent(indent + 1),
        )
    }
}

impl<Time, Value> Pretty for Item<Time, Value>
where
    Time: std::fmt::Display,
{
    fn pretty(&self, _: usize) -> String {
        format!(
            "Item({time}:{value:?})",
            time = self.time,
            value = 0,
            //             value = self.value,
        )
    }
}
