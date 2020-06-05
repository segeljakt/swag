<h1 align="center">SWAG - Sliding Window Aggregation</h1>
<h3 align="center">(Work in progress)</h3>

This is a crate for Sliding Window Aggregation (SWAG).

# Windowing Problem

The Windowing Problem is defined by the following characteristics, definition from [1]:

<p align="center">
  <img src="https://github.com/segeljakt/assets/raw/master/WindowingProblem.jpeg">
</p>

# API

## Out-of-Order API [3]

* Aggregation Functions
  * Associativity (Required)
* Stream Order
  * Out-of-order (Focus)
* Window Measures
  * Time (Required)
* State
  * In-Memory (Required)

> **Definition 1.**

> Let(⊗, 1) be a binary operator from a monoid and its identity. The out-of-order sliding-window aggregation (OoO SWAG) ADT is to maintain a time-ordered sliding window (t1, v1) … (tn, vn), ti < ti+1, supporting the following operations:

> — insert(t: Time, v: Agg) checks whether t is already in the window, i.e., whether there is an i such that t = ti. If so, it replaces vi by (ti, vi) by (ti,vi⊗v). Otherwise, it inserts v into the window at the appropriate location.

> — evict(t: Time) checks whether t is in the window, i.e., whether there is an i such that t=ti. If so, it removes ti from the window. Otherwise, it does nothing.

> — query(): Agg combines the values in time order using the ⊗ operator. In other words, it returns v1 ⊗ … ⊗ vn if the window is non-empty, or 1 if empty.

# Algorithms

| Algorithm                             | Alias | Time           | In-Order | Space | Invertible | Associative | Commutative | FIFO |
|---------------------------------------|-------|----------------|----------|-------|------------|-------------|-------------|------|
| **Subtract on Evict**                 [2] | SoE   | Worst O(1)     | Yes      | O(1)  | Yes        | No          | No          | No   |
| **Recalculate from Scratch**          [2] | RFS   | Worst O(n)     | Yes      | O(n)  | No         | No          | No          | No   |
| **Reactive Aggregator**               [4] | -     | Avg O(log n)   | Yes      | O(n)  | No         | No          | No          | No   |
| **Two-Stacks**                        [2] | -     | Avg O(1)       | Yes      | O(n)  | No         | No          | No          | Yes  |
| **Functional Okasaki Aggregator**     [2] | FOA   | Worst O(1)     | Yes      | O(n)  | No         | No          | No          | Yes  |
| **Imperative Okasaki Aggregator**     [2] | IOA   | Worst O(1)     | Yes      | O(n)  | No         | No          | No          | Yes  |
| **De-Amortized Banker's Aggregator**  [2] | DABA  | Worst O(1)     | Yes      | O(n)  | No         | No          | No          | Yes  |
| **Finger B-Tree Aggregator**          [3] | FiBA  | Worst O(log n) | No       | O(n)  | No         | Yes         | No          | No   |

# References

[1] Traub, J., Grulich, P.M., Cuéllar, A.R., Breß, S., Katsifodimos, A., Rabl, T. and Markl, V., 2019. **Efficient Window Aggregation with General Stream Slicing.** In EDBT (pp. 97-108).

[2] Tangwongsan, K., Hirzel, M. and Schneider, S., 2017, June. **Low-Latency Sliding-Window Aggregation in Worst-Case Constant Time.** In Proceedings of the 11th ACM International Conference on Distributed and Event-based Systems (pp. 66-77).

[3] Tangwongsan, K., Hirzel, M. and Schneider, S., 2019. **Optimal and General Out-of-Order Sliding-Window Aggregation.** Proceedings of the VLDB Endowment, 12(10), pp.1167-1180.

[3] Tangwongsan, K., Hirzel, M., Schneider, S. and Wu, K.L., 2015. **General Incremental Sliding-Window Aggregation**. Proceedings of the VLDB Endowment, 8(7), pp.702-713.


