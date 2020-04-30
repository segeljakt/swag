mod pretty;
// #![allow(unused)]
use crate::pretty::*;
use alga::general::AbstractMonoid;
use alga::general::Operator;
use arrayvec::ArrayVec;
use std::ops::Range;
use std::ptr::NonNull;

pub type Time = i32;
const NEG_INFINITY: Time = i32::MIN;
const POS_INFINITY: Time = i32::MAX;
type Uid = u32;

// While MIN_ARITY can be any integer greater than 1, most B-tree variations
// require that MAX_ARITY be at least 2*MIN_ARITY-1. Let α(y) denote the arity
// of node y, and just α in clear context. Then a B-tree obeys the following
// size invariants:
// *     Root nodes have:         2 <= α <= MAX_ARITY
// * Non-root nodes have: MIN_ARITY <= a <= MAX_ARITY
// * All nodes have α-1 timestamps and values: (t0,v0), ..., (tα-2,vα-2)
// * All non-leaf nodes have α child pointers z0,...zα-1
// For simplicity, we use MAX_ARITY = 2*MIN_ARITY
const MIN_ARITY: usize = 2;
const MAX_ARITY: usize = 4;

// Non-spine nodes store the up-aggregate: Π↑
//   * Such a node is neither a finger nor an ancestor of a finger.
//   * This aggregate must be repaired whenever the subtree below it changes.
// The root stores the inner aggregate: Π^
//   * This aggregate is only affected by changes to the inner part of the tree.
//   * And not by changes below the left-most or right-most child of the root.
// Non-root nodes on the left spine store the left aggregate: Π←
//   * For a given node y, the left aggregate encompasses all nodes under the
//     left-most child of the root except for y’s left-most child z0.
//   * When a change occurs below the left-most child of the root, the only
//     aggregates that need to be repaired are those on a traversal up to the
//     left spine and then down to the left finger.
// Non-root nodes on the right spine store the right aggregate: Π→
//   * This is symmetric to the left aggregate Π←.
//   * When a change occurs below the right-most child of the root, only
//     aggregates on a traversal to the right finger are repaired.
//
// Π↑(y) = Π↑(z0) + v0 + ... + vα-2 + Π↑(zα-1)
// Π^(y) = v0 + Π↑(z1) + ... + Π↑(zα-2) + vα-2
// Π←(y) = Π^(y) + Π^↑(zα-1) + (x = root ? 1 : Π←(y))
// Π→(y) = (x = root ? 1 : Π→(y)) + Π^↑(z0) + Π^(y)
//

pub struct Tree<T: Clone + AbstractMonoid<O>, O: Operator> {
    root: Box<Node<T, O>>,
    left_finger: NonNull<Node<T, O>>,
    right_finger: NonNull<Node<T, O>>,
    counter: Uid,
}

#[derive(PartialEq, Eq, Clone)]
struct Item<T> {
    time: Time,
    value: T,
}

struct Node<T, O> {
    children: ArrayVec<[Box<Node<T, O>>; MAX_ARITY + 1]>,
    items: ArrayVec<[Item<T>; MAX_ARITY]>,
    parent: Option<NonNull<Node<T, O>>>,
    agg: T,
    spine: Spine,
    uid: Uid,
    pt: PhantomData<O>,
}

use std::marker::PhantomData;

#[derive(PartialEq, Eq, Copy, Clone)]
struct Spine {
    left: bool,
    right: bool,
}

#[derive(PartialEq, Eq)]
enum AggKind {
    Up,
    Inner,
    Left,
    Right,
}

impl<T> Item<T> {
    fn new(time: Time, value: T) -> Item<T> {
        Item { time, value }
    }
}

impl Spine {
    fn new(left: bool, right: bool) -> Spine {
        Spine { left, right }
    }
}

impl<T, O> Node<T, O>
where
    T: Clone + AbstractMonoid<O> + 'static,
    O: Operator + 'static,
{
    fn new(uid: Uid) -> Node<T, O> {
        Node {
            children: ArrayVec::new(),
            items: ArrayVec::new(),
            parent: None,
            agg: T::identity(),
            spine: Spine::new(false, false),
            uid,
            pt: PhantomData,
        }
    }
    // The root stores the inner aggregate: Π^
    // Non-spine nodes (neither finger nor ancestor of a finger) store the up-aggregate: Π↑
    // Non-root nodes on the left spine store the left aggregate: Π←
    // Non-root nodes on the right spine store the right aggregate: Π→
    fn agg_kind(&self) -> AggKind {
        match (self.spine.left, self.is_root(), self.spine.right) {
            (_, true, _) => AggKind::Inner,
            (true, false, false) => AggKind::Left,
            (false, false, true) => AggKind::Right,
            (false, false, false) => AggKind::Up,
            x => panic!("Unknown aggregate {:?}", x),
        }
    }

    // Checks
    #[inline(always)]
    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }
    #[inline(always)]
    fn is_root(&self) -> bool {
        self.parent.is_none()
    }
    fn is_descendent_of(&self, other: &Node<T, O>) -> bool {
        let mut node = self;
        while node.parent.is_some() {
            node = node.get_parent();
            if node.uid == other.uid {
                return true;
            }
        }
        false
    }
    #[inline(always)]
    fn has_agg_up(&self) -> bool {
        self.agg_kind() == AggKind::Up
    }
    // Getters
    #[inline(always)]
    fn get_parent(&self) -> &'static mut Node<T, O> {
        unsafe { self.parent.unwrap().as_ptr().as_mut().unwrap() }
    }
    #[inline(always)]
    fn get_arity(&self) -> usize {
        self.items.len() + 1
    }
    fn get_youngest(&mut self) -> (&mut Node<T, O>, Item<T>) {
        let mut node = self;
        while node.children.last().is_some() {
            node = node.children.last_mut().unwrap();
        }
        let item = node.items.last().unwrap().clone();
        (node, item)
    }
    fn get_oldest(&mut self) -> (&mut Node<T, O>, Item<T>) {
        let mut node = self;
        while node.children.first().is_some() {
            node = node.children.first_mut().unwrap();
        }
        let item = node.items.first().unwrap().clone();
        (node, item)
    }

    // BTree operations: Splits a node in two by the median item/child. The median item
    // separating the splitted node is inserted at the parent node.
    fn split(&mut self, tree: &mut Tree<T, O>) {
        let mut left = self;
        // Create new node
        let mut right = tree.new_node();
        let parent = left.get_parent();
        right.parent = left.parent;
        // Split items
        let middle = left.get_arity() / 2;
        right.items = left.items.drain(middle + 1..).collect();
        let middle_item = left.items.remove(middle);
        // Split children (if any)
        if !left.is_leaf() {
            let parent = Some(unsafe { NonNull::new_unchecked(right.as_mut()) });
            for mut child in left.children.drain(middle + 1..) {
                child.parent = parent;
                right.children.push(child);
            }
        };
        // Find an index for the middle element
        let i = parent.item_idx(middle_item.time).unwrap_err();
        // Update spines
        if i == 0 && (parent.is_root() || parent.spine.left) {
            left.spine = Spine::new(true, false);
            if left.is_leaf() {
                tree.left_finger = unsafe { NonNull::new_unchecked(left as *mut Node<T, O>) };
            }
        } else {
            left.spine = Spine::new(false, false);
        }
        if i == parent.items.len() && (parent.is_root() || parent.spine.right) {
            right.spine = Spine::new(false, true);
            if right.is_leaf() {
                tree.right_finger = unsafe { NonNull::new_unchecked(right.as_mut()) };
            }
        } else {
            right.spine = Spine::new(false, false);
        }
        parent.items.insert(i, middle_item);
        parent.children.insert(i + 1, right);
        let right = &mut parent.children[i + 1];
        // Repair aggregates, NOTE: Parent is repaired later
        // (Either in the next split, or when rebalancing tops out)
        left.local_repair_agg();
        right.local_repair_agg();
    }
    fn merge(
        &mut self,
        node_idx: usize,
        sibling_idx: usize,
        tree: &mut Tree<T, O>,
    ) -> &mut Node<T, O> {
        // Merge a and b into one node, and transfer the item between them to the new node
        // Parent is self
        // FIXME: Maybe this can be handled better
        let uid = self.children[node_idx].uid;
        let (left_idx, right_idx) = if sibling_idx < node_idx {
            (sibling_idx, node_idx)
        } else {
            (node_idx, sibling_idx)
        };
        let right = self.children.remove(right_idx);
        let middle_item = self.items.remove(left_idx);
        let mut left = &mut self.children[left_idx];
        let left_ptr = NonNull::from(left.as_ref());
        if right.spine.right && right.is_leaf() {
            tree.right_finger = left_ptr;
        }
        left.items.push(middle_item);
        for item in right.items {
            left.items.push(item);
        }
        for mut child in right.children {
            child.parent = Some(left_ptr);
            // TODO: Do we need to repair the child here?
            left.children.push(child);
        }
        left.spine.left = left.spine.left || right.spine.left;
        left.spine.right = left.spine.right || right.spine.right;
        left.uid = uid;
        // NOTE: left and parent are repaired later
        left
    }
    fn steal(&mut self, node_idx: usize, sibling_idx: usize) {
        // The node steals an item from the parent (self) and a child from the sibling
        // The parent then steals an item from the sibling
        // NOTE: Sibling's child will never be a finger, so no need to adjust
        let parent = self;
        let sibling = &mut parent.children[sibling_idx];
        if node_idx < sibling_idx {
            // Steal from the right sibling
            let sibling_child = sibling.children.pop_at(0);
            let sibling_item = sibling.items.remove(0);
            // Steal from the parent
            let parent_item = parent.items.remove(0);
            let node = &mut parent.children[node_idx];
            node.items.push(parent_item);
            parent.items.insert(sibling_idx - 1, sibling_item);
            if let Some(mut sibling_child) = sibling_child {
                sibling_child.parent = Some(NonNull::from(node.as_ref()));
                node.children.push(sibling_child);
                node.children.last_mut().unwrap().local_repair_agg();
            }
        } else {
            // Steal from the left sibling
            let sibling_child = sibling.children.pop();
            let sibling_item = sibling.items.pop().unwrap();
            // Steal from the parent
            let parent_item = parent.items.pop().unwrap();
            // Update the node
            let node = &mut parent.children[node_idx];
            node.items.insert(0, parent_item);
            parent.items.insert(sibling_idx, sibling_item);
            if let Some(mut sibling_child) = sibling_child {
                sibling_child.parent = Some(NonNull::from(node.as_ref()));
                node.children.insert(0, sibling_child);
                node.children.first_mut().unwrap().local_repair_agg();
            }
        }
        // Repairs, NOTE: Parent is repaired later
        parent.children[node_idx].local_repair_agg();
        parent.children[sibling_idx].local_repair_agg();
        parent.local_repair_agg();
    }

    // Π↑(y) = Π↑(z0) + v0 + ... + vα-2 + Π↑(zα-1)
    fn up_agg(&self) -> T {
        let mut agg = T::identity();
        if self.is_leaf() {
            for i in 0..self.get_arity() - 1 {
                agg = agg.operate(&self.items[i].value);
            }
            agg
        } else {
            for i in 0..self.get_arity() - 1 {
                agg = agg.operate(&self.children[i].agg);
                agg = agg.operate(&self.items[i].value);
            }
            agg.operate(&self.children.last().unwrap().agg)
        }
    }
    // Π^(y) = v0 + Π↑(z1) + ... + Π↑(zα-2) + vα-2
    fn inner_agg(&self) -> T {
        let mut agg = T::identity();
        if self.is_leaf() {
            for i in 0..self.get_arity() - 1 {
                agg = agg.operate(&self.items[i].value);
            }
        } else {
            agg = agg.operate(&self.items.first().unwrap().value);
            for i in 1..self.get_arity() - 1 {
                agg = agg.operate(&self.children[i].agg);
                agg = agg.operate(&self.items[i].value);
            }
        }
        agg
    }
    // Π←(y) = Π^(y) + Π^↑(zα-1) + (x = root ? 1 : Π←(y))
    fn left_agg(&self) -> T {
        let mut agg = self.inner_agg();
        if !self.is_leaf() {
            agg = agg.operate(&self.children.last().unwrap().agg);
        }
        agg = agg.operate(&self.parent_agg());
        agg
    }
    // Π→(y) = (x = root ? 1 : Π→(y)) + Π^↑(z0) + Π^(y)
    fn right_agg(&self) -> T {
        let mut agg = self.parent_agg();
        if !self.is_leaf() {
            agg = agg.operate(&self.children.first().unwrap().agg);
        }
        agg = agg.operate(&self.inner_agg());
        agg
    }
    // x = root ? 1 : ...
    fn parent_agg(&self) -> T {
        let parent = self.get_parent();
        if parent.is_root() {
            T::identity()
        } else {
            parent.agg.clone()
        }
    }
    fn local_repair_agg(&mut self) {
        self.agg = match self.agg_kind() {
            AggKind::Up => self.up_agg(),
            AggKind::Inner => self.inner_agg(),
            AggKind::Left => self.left_agg(),
            AggKind::Right => self.right_agg(),
        };
    }
    fn local_repair_agg_if_up(&mut self) {
        match self.agg_kind() {
            AggKind::Up => self.agg = self.up_agg(),
            _ => {}
        }
    }
    fn local_search(&self, t: Time) -> Option<usize> {
        self.items.binary_search_by_key(&t, |item| item.time).ok()
    }
    // Insert (t,v) and update stored aggregate
    fn local_insert_time_and_value(&mut self, t: Time, v: T) {
        match self.item_idx(t) {
            Ok(i) => self.items[i].value = self.items[i].value.operate(&v),
            Err(i) => self.items.insert(i, Item::new(t, v)),
        }
        self.local_repair_agg();
    }
    // Evict (t,v) from a leaf and update stored aggregate
    fn local_evict_time_and_value(&mut self, t: Time) {
        if let Ok(i) = self.items.binary_search_by_key(&t, |item| item.time) {
            self.items.remove(i);
            self.local_repair_agg()
        }
    }

    /// Returns the index of an item
    fn item_idx(&mut self, t: Time) -> std::result::Result<usize, usize> {
        self.items.binary_search_by_key(&t, |item| item.time)
    }
    fn child_idx(&mut self, uid: Uid) -> usize {
        let (idx, _) = self
            .children
            .iter()
            .enumerate()
            .find(|(_, node)| node.uid == uid)
            .unwrap();
        idx
    }
    fn search(&mut self, t: Time) -> &mut Node<T, O> {
        let mut node = self;
        while !node.is_leaf() {
            if let Err(i) = node.item_idx(t) {
                node = &mut node.children[i]
            } else {
                break;
            }
        }
        node
    }
    fn search_from_left_finger(&mut self, t: Time) -> &mut Node<T, O> {
        let mut node = self;
        while let Err(i) = node.item_idx(t) {
            if node.parent.is_none() {
                break;
            } else {
                let parent = node.get_parent();
                if parent.items.first().unwrap().time <= t {
                    node = parent;
                } else if !node.is_leaf() {
                    return node.children[i].search(t);
                } else {
                    break;
                }
            }
        }
        node
    }
    fn search_from_right_finger(&mut self, t: Time) -> &mut Node<T, O> {
        let mut node = self;
        while let Err(i) = node.item_idx(t) {
            if node.parent.is_none() {
                break;
            } else {
                let parent = node.get_parent();
                if t <= parent.items.first().unwrap().time {
                    node = parent;
                } else if !node.is_leaf() {
                    return node.children[i].search(t);
                } else {
                    break;
                }
            }
        }
        node
    }

    // Finally, they repair any remaining aggregate values not repaired during
    // rebalancing, starting above the node where rebalancing topped out and
    // visiting all ancestors up to the root.
    fn repair_aggs(&mut self, hit: Spine) {
        self.repair_up();
        self.repair_left(hit.left);
        self.repair_right(hit.right);
    }
    fn repair_up(&mut self) {
        let mut node = self;
        if node.has_agg_up() {
            while node.has_agg_up() {
                node = node.get_parent();
                node.local_repair_agg();
            }
        } else {
            node.local_repair_agg();
        }
    }
    fn repair_left(&mut self, hit_left: bool) {
        let mut node = self;
        if node.spine.left || node.is_root() && hit_left {
            while !node.is_leaf() {
                node = node.children.first_mut().unwrap();
                node.local_repair_agg();
            }
        }
    }
    fn repair_right(&mut self, hit_right: bool) {
        let mut node = self;
        if node.spine.right || node.is_root() && hit_right {
            while !node.is_leaf() {
                node = node.children.last_mut().unwrap();
                node.local_repair_agg();
            }
        }
    }
    fn pick_eviction_sibling(&self) -> (usize, usize) {
        let parent = self.get_parent();
        let idx = parent.child_idx(self.uid);
        // TODO: Change to idx > 0
        if idx + 1 < parent.children.len() {
            (idx, idx + 1)
        } else {
            (idx, idx - 1)
        }
    }
    // Rebalance the tree, walking from that node towards the root as necessary
    // to fix any size invariant violations, while also repairing aggregate
    // values along the way.
    // After-the-fact strategy, amortized constant as long as MAX_ARITY ≥ 2*MIN_ARITY
    // The amortized cost is O(1) as rebalancing rarely goes all the way up the tree.
    // The worst-case cost is O(log(n)), bounded by the tree height.
    fn rebalance_for_insert(&mut self, tree: &mut Tree<T, O>) -> (&mut Node<T, O>, Spine) {
        let mut node = self;
        let mut hit = node.spine;
        while node.get_arity() > MAX_ARITY {
            if node.is_root() {
                tree.height_increase();
                hit = Spine::new(true, true);
            }
            node.split(tree);
            node = node.get_parent();
            hit.left = hit.left || node.spine.left;
            hit.right = hit.right || node.spine.right;
        }
        return (node, hit);
    }
    // Rebalance the tree, walking from that node towards the root as necessary
    // to fix any size invariant violations, while also repairing aggregate
    // values along the way.
    fn rebalance_for_evict(
        &mut self,
        to_repair: Option<Uid>,
        tree: &mut Tree<T, O>,
    ) -> (&mut Node<T, O>, Spine) {
        let mut node = self;
        let mut hit = node.spine;
        if Some(node.uid) == to_repair {
            node.local_repair_agg_if_up();
        }
        while !node.is_root() && node.get_arity() < MIN_ARITY {
            let parent = node.get_parent();
            let (node_idx, sibling_idx) = node.pick_eviction_sibling();
            let sibling = &mut parent.children[sibling_idx];
            hit.left = hit.left || sibling.spine.left;
            hit.right = hit.right || sibling.spine.right;
            if sibling.get_arity() <= MIN_ARITY {
                node = parent.merge(node_idx, sibling_idx, tree);
                let parent = node.get_parent();
                if parent.is_root() && parent.get_arity() == 1 {
                    tree.height_decrease();
                } else {
                    // parent.local_repair_agg(); // ????
                    node = parent;
                }
            } else {
                parent.steal(node_idx, sibling_idx);
                node = parent;
            }
            if Some(node.uid) == to_repair {
                node.local_repair_agg_if_up();
            }
            hit.left = hit.left || node.spine.left;
            hit.right = hit.right | node.spine.right;
        }
        (node, hit)
    }
    // To evict something from an inner node
    // Function evict_inner creates an obligation to repair an extra node during
    // rebalancing, handled by parameter to_repair.
    fn evict_inner(&mut self, idx: usize, tree: &mut Tree<T, O>) -> (&mut Node<T, O>, Spine) {
        let node = unsafe { (self as *mut Node<T, O>).as_mut().unwrap() };
        let (leaf, item) = if self.children[idx + 1].get_arity() > MIN_ARITY {
            let right = &mut self.children[idx + 1];
            right.get_oldest()
        } else {
            let left = &mut self.children[idx];
            left.get_youngest()
        };
        // Evict a substitute from a leaf instead
        leaf.local_evict_time_and_value(item.time);
        // Writes substitute over the evicted slot
        node.items[idx] = item;
        let (mut top, mut hit) = leaf.rebalance_for_evict(Some(node.uid), tree);
        if top.is_descendent_of(node) {
            while top.uid != node.uid {
                top = top.get_parent();
                hit.left = hit.left || top.spine.left;
                hit.right = hit.right || top.spine.right;
                top.local_repair_agg_if_up();
            }
        }
        (top, hit)
    }
}

impl<T, O> Tree<T, O>
where
    T: Clone + AbstractMonoid<O> + 'static,
    O: Operator + 'static,
{
    fn height_increase(&mut self) {
        let new_root = self.new_node();
        let mut old_root = std::mem::replace(&mut self.root, new_root);
        old_root.parent = Some(NonNull::from(self.root.as_ref()));
        self.root.children.push(old_root);
    }
    fn height_decrease(&mut self) {
        let mut new_root = self.root.children.pop().unwrap();
        new_root.parent = None;
        self.root = new_root;
        if self.root.is_leaf() {
            self.left_finger = NonNull::from(self.root.as_ref());
            self.right_finger = NonNull::from(self.root.as_ref());
            self.root.spine = Spine::new(false, false);
        }
        self.root.local_repair_agg();
    }
    pub fn new() -> Tree<T, O> {
        let counter = 0;
        let root = Box::new(Node::new(counter));
        let left_finger = NonNull::from(root.as_ref());
        let right_finger = NonNull::from(root.as_ref());
        Tree {
            root,
            left_finger,
            right_finger,
            counter,
        }
    }
    fn new_node(&mut self) -> Box<Node<T, O>> {
        self.counter += 1;
        Box::new(Node::new(self.counter))
    }
    // Combines the values in time order using the + operator. In other words,
    // it returns v1 + ... + vn if the window is non-empty, or 1 if empty.
    pub fn query(&self) -> T {
        if self.root.is_leaf() {
            self.root.agg.clone()
        } else {
            unsafe {
                let mut agg = T::identity();
                agg = agg.operate(&self.left_finger.as_ref().agg);
                agg = agg.operate(&self.root.agg);
                agg = agg.operate(&self.right_finger.as_ref().agg);
                agg
            }
        }
    }
    // Checks whether t is already in the window, i.e. whether there is an i
    pub fn insert_test(&mut self, t: Time, v: T) {
        let tree = unsafe { (self as *mut Tree<T, O>).as_mut().unwrap() };
        // Search for the node where t belongs
        let node = self.search_node_test(t);
        // Update stored aggregate
        node.local_insert_time_and_value(t, v);
        //
        // While rebalancing always works bottom-up, aggregate repair works in the
        // direction of the partial aggregates: either up for up-agg or inner-agg, or
        // down for left-agg or right-agg. Our algorithm piggybacks the repair of up-aggs
        // onto the local insert or evict and onto rebalancing, and then repairs the
        // remaining aggregates separately. To facilitate the handover from the piggybacked
        // phase to the dedicated phase of aggregate repair, the rebalancing routines return
        // a pair (top, hit)
        //
        // Node top is where rebalancing topped out, and if it has an up-agg, it is the last
        // node whose aggregate has already been repaired.
        // Booleans hit.left and hit.right indicate whether rebalancing affected the left or
        // right spine, determining whether aggregates on the respective spine have to be
        // repaired.
        //
        let (top, hit) = node.rebalance_for_insert(tree);
        top.repair_aggs(hit);
    }
    // such that t = ti. If so, it replaces (ti,vi) by (ti,vi+v). Otherwise, it
    // inserts (t,v) into the window at the appropriate location.
    pub fn insert(&mut self, t: Time, v: T) {
        let tree = unsafe { (self as *mut Tree<T, O>).as_mut().unwrap() };
        // Search for the node where t belongs
        let node = self.search_node(t);
        // Update stored aggregate
        node.local_insert_time_and_value(t, v);
        //
        // While rebalancing always works bottom-up, aggregate repair works in the
        // direction of the partial aggregates: either up for up-agg or inner-agg, or
        // down for left-agg or right-agg. Our algorithm piggybacks the repair of up-aggs
        // onto the local insert or evict and onto rebalancing, and then repairs the
        // remaining aggregates separately. To facilitate the handover from the piggybacked
        // phase to the dedicated phase of aggregate repair, the rebalancing routines return
        // a pair (top, hit)
        //
        // Node top is where rebalancing topped out, and if it has an up-agg, it is the last
        // node whose aggregate has already been repaired.
        // Booleans hit.left and hit.right indicate whether rebalancing affected the left or
        // right spine, determining whether aggregates on the respective spine have to be
        // repaired.
        //
        let (top, hit) = node.rebalance_for_insert(tree);
        top.repair_aggs(hit);
    }
    // Checks whether t is in the window, i.e., whether there is an i such that
    // t = ti. If so, it removes (ti,vi) from the window. Otherwise it does nothing.
    pub fn evict(&mut self, t: Time) {
        let tree = unsafe { (self as *mut Tree<T, O>).as_mut().unwrap() };
        let node = self.search_node(t);
        if let Some(idx) = node.local_search(t) {
            let (top, hit) = if node.is_leaf() {
                node.local_evict_time_and_value(t);
                node.rebalance_for_evict(None, tree)
            } else {
                node.evict_inner(idx, tree)
            };
            top.repair_aggs(hit);
        }
    }
    // Search for the node where t belongs. We keep fingers pointers to the
    // left- and right-most leaves. Also, we keep parent pointers at each node.
    // Hence, search can start at the nearest finger, walk up to the nearest
    // common ancestor of the finger and y, and walk down from there to y.
    fn search_node_test(&mut self, t: Time) -> &mut Node<T, O> {
        unsafe {
            match self.root.items.as_slice() {
                [x, ..] if t < x.time => self.left_finger.as_mut().search_from_left_finger(t),
                [.., x] if t > x.time => self.right_finger.as_mut().search_from_right_finger(t),
                [..] => self.root.as_mut().search(t),
            }
        }
    }
    fn search_node(&mut self, t: Time) -> &mut Node<T, O> {
        unsafe {
            match self.root.items.as_slice() {
                [x, ..] if t < x.time => self.left_finger.as_mut().search_from_left_finger(t),
                [.., x] if t > x.time => self.right_finger.as_mut().search_from_right_finger(t),
                [..] => self.root.as_mut().search(t),
            }
        }
    }
    // Aggregates exactly the values in the window whose times fall within the range.
    // If the subrange contains no values, it returns the identity.
    pub fn range_query(&mut self, range: Range<Time>) -> T {
        // uses recursion starting from the least-common ancestor node whose
        // subtree encompasses the queried range
        let node_from = self.search_node(range.start);
        let node_top = Self::least_common_ancestor(node_from, range.end);
        // invoke at most two chains of recursive calls, one visiting ancestors
        // of node_from and the other visiting ancestors of node_to
        Self::query_rec(node_top, range)
    }
    // Requires that node.time < time
    fn least_common_ancestor(mut node: &mut Node<T, O>, time: Time) -> &mut Node<T, O> {
        loop {
            if node.is_root() {
                return node;
            }
            let parent = node.get_parent();
            if parent.items.last().unwrap().time <= time {
                node = parent;
            } else {
                return node;
            }
        }
    }
    fn query_rec(node: &mut Node<T, O>, range: Range<Time>) -> T {
        // The insight for preventing spurious recursive calls is that one
        // needs information about neighboring timestamps in a node’s parent to
        // determine whether the node itself is subsumed by the range. This is
        // passed down the recursive call: whether the neighboring timestamp in
        // the parent is included in the range on the left or right is indicated
        // by t_from = −∞ or t_to= +∞, respectively.
        if range.start == NEG_INFINITY && range.end == POS_INFINITY && node.has_agg_up() {
            return node.agg.clone();
        }
        let mut res = T::identity();
        if !node.is_leaf() {
            let t_next = node.items.first().unwrap().time;
            if range.start < t_next {
                let t_a = range.start;
                let t_b = if t_next <= range.end {
                    POS_INFINITY
                } else {
                    range.end
                };
                let child = &mut node.children.first_mut().unwrap();
                res = res.operate(&Self::query_rec(child, t_a..t_b))
            }
        }
        for (i, Item { time, value }) in node.items.iter().enumerate() {
            if range.start <= *time && *time <= range.end {
                res = res.operate(&value);
            }
            if !node.is_leaf() && i + 1 <= node.get_arity() - 2 {
                let t_ii = node.items[i + 1].time;
                if *time < range.end && range.start < t_ii {
                    let t_a = if range.start <= *time {
                        NEG_INFINITY
                    } else {
                        range.start
                    };
                    let t_b = if t_ii <= range.end {
                        POS_INFINITY
                    } else {
                        range.end
                    };
                    let child = &mut node.children[i + 1];
                    // each recursive call returns the aggregate of the intersection between
                    // its subtree and the queried range.
                    res = res.operate(&Self::query_rec(child, t_a..t_b));
                }
            }
        }
        if !node.is_leaf() {
            let t_curr = node.items[node.get_arity() - 2].time;
            if t_curr < range.end {
                let t_a = if range.start <= t_curr {
                    NEG_INFINITY
                } else {
                    range.start
                };
                let t_b = range.end;
                let arity = node.get_arity();
                let child = &mut node.children[arity - 1];
                res = res.operate(&Self::query_rec(child, t_a..t_b));
            }
        }
        res
    }
}
