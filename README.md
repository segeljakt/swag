# SWAG - Sliding Window Aggregation

This is a crate for Sliding Window Aggregation (SWAG).

# API

> Definition 1. Let(⊗, 1) be a binary operator from a monoid and its identity. The out-of-order sliding-window aggregation (OoO SWAG) ADT is to maintain a time-ordered sliding window (t1, v1) … (tn, vn), ti < ti+1, supporting the following operations:
> — insert(t: Time, v: Agg) checks whether t is already in the window, i.e., whether there is an i such that t = ti. If so, it replaces vi by (ti, vi) by (ti,vi⊗v). Otherwise, it inserts v into the window at the appropriate location.
> — evict(t: Time) checks whether t is in the window, i.e., whether there is an i such that t=ti. If so, it removes ti from the window. Otherwise, it does nothing.
> — query(): Agg combines the values in time order using the ⊗ operator. In other words, it returns v1 ⊗ … ⊗ vn if the window is non-empty, or 1 if empty.

# Problems

Workload Characteristics [^1]:
<span id="a1">[¹](#1)</span>

* Aggregation Functions
  * Distributive
  * Algebraic
  * Holistic
  * Associativity
  * Commutativity
  * Invertibility
* Window Types
  * Context Free
  * Forward Context Free
  * Forward Context Sensitive
* Stream Order
  * In-order
  * Out-of-order
* Window Measures
  * Time
  * Tuple count
  * Arbitrary

| Algorithm                                    | Time           | In-Order | Space | Invertible | Associative | Commutative | FIFO |
|----------------------------------------------|----------------|----------|-------|------------|-------------|-------------|------|
| SOE  <span id="a1">[²](#1)</span> (Subtract on Evict)                | Worst O(1)     | Yes      | O(1)  | Yes        | No          | No          | No   |
| RFS  <span id="a1">[²](#1)</span> (Recalculate from Scratch)         | Worst O(n)     | Yes      | O(n)  | No         | No          | No          | No   |
| RA   <span id="a1">[²](#1)</span> (Reactive Aggregator)              | Avg O(log n)   | Yes      | O(n)  | No         | No          | No          | No   |
| TS   <span id="a1">[²](#1)</span> (Two-Stacks)                       | Avg O(1)       | Yes      | O(n)  | No         | No          | No          | Yes  |
| FOA  <span id="a1">[²](#1)</span> (Functional Okasaki Aggregator)    | Worst O(1)     | Yes      | O(n)  | No         | No          | No          | Yes  |
| IOA  <span id="a1">[²](#1)</span> (Imperative Okasaki Aggregator)    | Worst O(1)     | Yes      | O(n)  | No         | No          | No          | Yes  |
| DABA <span id="a1">[²](#1)</span> (De-Amortized Banker's Aggregator) | Worst O(1)     | Yes      | O(n)  | No         | No          | No          | Yes  |
| FiBA <span id="a1">[³](#1)</span> (Finger B-Tree Aggregator)         | Worst O(log n) | No       | O(n)  | No         | Yes         | No          | No   |

<span id="1">¹</span> Traub, J., Grulich, P.M., Cuéllar, A.R., Breß, S., Katsifodimos, A., Rabl, T. and Markl, V., 2019. Efficient Window Aggregation with General Stream Slicing. In EDBT (pp. 97-108).[⏎](#a1)<br>
<span id="2">²</span> Tangwongsan, K., Hirzel, M. and Schneider, S., 2017, June. Low-latency sliding-window aggregation in worst-case constant time. In Proceedings of the 11th ACM International Conference on Distributed and Event-based Systems (pp. 66-77). [⏎](#a2)<br>
<span id="3">³</span> Tangwongsan, K., Hirzel, M. and Schneider, S., 2019. Optimal and general out-of-order sliding-window aggregation. Proceedings of the VLDB Endowment, 12(10), pp.1167-1180. [⏎](#a3)<br>

