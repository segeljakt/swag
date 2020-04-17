#![allow(unused)]

type Time = i32;
type Agg = i32;

const MAX_ARITY = 3;

struct Node {
    children: Vec<Node>,
    agg: Agg,
    time: Time,
    left_spine: Box<Node>,
    right_spine: Box<Node>,
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }
    fn local_insert_time_and_value(&mut self, t: Time, v: Agg) {
        self.time = t;
        self.agg = v;
    }
    fn arity(&self) -> usize {
        self.children.len()
    }
}
fn rebalance_for_insert(node: &mut Node) -> (Node, bool, bool) {
    let (hit_left, hit_right) = (node.left_spine, node.right_spine);
    while node.arity() > MAX_ARITY {
        if self.is_root() {
            
        }
    }
    todo!();
}

struct Tree {
    root: Node,
    left_finger: Node,
    right_finger: Node,
}

impl Tree {
    fn query(&self) -> Agg {
        if self.root.is_leaf() {
            self.root.agg
        } else {
            self.left_finger.agg + self.root.agg + self.right_finger.agg
        }
    }
    fn insert(&mut self, t: Time, v: Agg) {
        let mut node = self.search_node(t);
        node.local_insert_time_and_value(t, v);
        let (top, hit_left, hit_right) = self.rebalance_for_insert(node);
        self.repair_aggs(top, hit_left, hit_right);
    }
    fn search_node(&mut self, t: Time) -> Node {
        todo!()
    }
}
