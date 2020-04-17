# SWAG - Sliding Window Aggregation

SWAG is a 

# API

> Definition 1. Let(⊗, 1) be a binary operator from a monoid and its identity. The out-of-order sliding-window aggregation (OoO SWAG) ADT is to maintain a time-ordered sliding window (t1, v1) … (tn, vn), ti < ti+1, supporting the following operations:
> — insert(t: Time, v: Agg) checks whether t is already in the window, i.e., whether there is an i such that t = ti. If so, it replaces vi by (ti, vi) by (ti,vi⊗v). Otherwise, it inserts v into the window at the appropriate location.
> — evict(t: Time) checks whether t is in the window, i.e., whether there is an i such that t=ti. If so, it removes ti from the window. Otherwise, it does nothing.
> — query(): Agg combines the values in time order using the ⊗ operator. In other words, it returns v1 ⊗ … ⊗ vn if the window is non-empty, or 1 if empty.

# Problems

Workload Characteristics:

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

Jonas Traub & Philipp Grulich


