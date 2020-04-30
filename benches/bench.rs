use alga::general::AbstractMagma;
use alga::general::AbstractMonoid;
use alga::general::AbstractSemigroup;
use alga::general::Identity;
use alga::general::Operator;
use {
    criterion::*,
    swag::{Time, Tree},
};
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

/// We implemented both OoO SWAG variants in C++: the baseline classic B-tree augmented with
/// aggregates and the finger B-tree aggregator (FiBA). We present experiments with competitive min-
/// arity values: 2, 4 and 8. Higher values for min-arity were never competitive in our experiments.
/// Our experiments run outside of any particular streaming framework so we can focus on the
/// aggregation algorithms themselves. Our load generator produces synthetic data items with random
/// integers. The experiments perform rounds of evict, insert, and query to maintain a sliding
/// window that accepts a new data item, evicts an old one, and produces a result each round.
///
/// We present results with three aggregation operators, each representing a category of
/// computational cost. The operator sum performs an integer sum over the window, and its
/// computational cost is less than that of tree traversals and manipulations. The operator geomean
/// performs a geometric mean over the window. For numerical stability, this requires a
/// floating-point log on insertion and floating-point additions during data structure operations.
/// It represents a middle ground in computational cost. The most expensive operator, bloom, is a
/// Bloom filter [11] where the partial aggregations maintain a bitset of size 214. It represents
/// aggregation operators whose computational cost dominates the cost of maintaining the SWAG data
/// structure.
///
/// We ran all experiments on a machine with an Intel Xeon E5-2697 at 2.7 GHz running Red Hat
/// Enterprise Linux Server 7.5 with a 3.10.0 kernel. We compiled all experiments with g++ 4.8.5
/// with optimization level -O3.
///
/// Summary:
/// * Implementation: C++
/// * Arity 2, 4, 8
/// * Agnostic of streaming framework
/// * Synthetic data load generator
/// * Perform rounds of evict, insert, and query
/// * Aggregation operators:
///   * Arithmetic sum (integer)
///   * Geometric mean (float)
///   * Bloom filter (bitset of size 214)
/// * Machine:
///   * Intel Xeon E5-2697 2.7 GHz
///   * Red Hat Enterprise Linux Server 7.5 wiwth a 3.10.0 kernel
///   * Compiled with g++ 4.8.5 at -O3
///
mod description {}
criterion_group!(
    name = benches;
    config = Criterion::default()
               .sample_size(1000)
               .with_plots()
               .warm_up_time(std::time::Duration::new(3, 0));
    targets = experiment_1_varying_distance,
              experiment_2_latency,
              experiment_3_fifo_in_order_data,
              experiment_4_window_sharing,
              experiment_5_real_data,
              experiment_6_distance_varying_and_fifo,
              experiment_7_coarse_grained_window,

);

criterion_main!(benches);

/// We begin by investigating how insert’s out-of-order distance affects throughput. The distance
/// varying experiments, Figure 11, maintain a constant-sized window of n = 2^22 = 4_194_304 data
/// items. The x-axis is the out-of-order distance d between the newest timestamp already in the
/// window and the time-stamp created by our load generator. Our adversarial load generator
/// pre-populates the window with high timestamps and then spends the measured portion of the
/// experiment producing low timestamps. This regime ensures that after the pre-population with high
/// timestamps, the out-of-order distance of each subsequent insertion is precisely d.
///
/// This experiment confirms the prediction of the theory. The classic B-tree’s throughput is mostly
/// unaffected by the change in distance, but the finger B-tree’s throughput starts out
/// significantly higher. At the smallest values of d, the best finger B-tree outperforms the
/// corresponding classic B-tree by a factor of 3.4× for sum, 2.5× for geomean, and 4.9× for bloom.
/// For larger values of d, the finger B-tree throughput follows a log d trend. All variants enjoy
/// an uptick in performance when d = n, that is, when the distance is the size n
/// of the window. This is a degenerate special case. When n = d, the lowest timestamp to evict is
/// always in the left-most node in the tree, so the tree behaves like a last-in first-out (LIFO)
/// stack, and inserting and evicting requires no tree restructuring—O(1) time overall.
///
/// The min-arity that yields the best-performing B-tree varies with the aggregation operator. For
/// expensive operators, such as bloom, smaller min-arity trees perform better because they perform
/// fewer partial aggregations inside of a node. Conversely, for cheap operators, such as sum,
/// higher min-arity trees that require fewer rebalance and repair operations perform better. The
/// step-like throughput curves for the finger B-trees is a function of their min-arity: larger
/// min-arity means longer sections where the increased out-of-order distance still affects only a
/// subtree with the same height. When the throughput suddenly drops, the increase in d meant an
/// increase in the height of the affected subtree, causing more rebalances and updates.
///
/// Summary:
/// * Constant-sized window of n = 2^22 data items
/// * Pre-populate window with high timestamps
/// * Then, produce only low timestamps (with out-of-order distance d with respect to n)
/// * d = highest_timestamp - generated_timestamp
/// * Measure throughput [million items/s] against out-of-order distance [d]
/// * Implementations: bclassic[2,4,8], bfinger[2,4,8]
/// * In this case, when d = n, FiBA behaves like a O(1) stack for insert/evict
/// * Smaller min-arity => Better for expensive operators, since fewer partial aggregations
/// * Higher min-arity => Better for cheap operators, since fewer rebalances/repairs
fn experiment_1_varying_distance(criterion: &mut Criterion) {
    // Setup
    let mut tree = Tree::<Value, BinOp>::new();
    let timestamps = (2 as Time).pow(22);
    for timestamp in 0..timestamps {
        tree.insert(timestamp, Value(0));
    }
    let mut g = criterion.benchmark_group("bfinger2");
    let g = g.throughput(Throughput::Elements(1));
    // Experiment
    for exponent in 0..22 {
        let d = (2 as Time).pow(exponent);
        g.bench_with_input(format!("2^{}", exponent), &d, |bench, d| {
            bench.iter(|| {
                tree.insert(black_box(*d), black_box(Value(0)));
                tree.evict(black_box(*d));
                tree.query();
            })
        });
    }
}

// The worst-case latency for both classic and finger B-trees is O(log n), but we expect finger
// variants to reduce average latency. The experiments in Figure 12 confirm this expectation. All
// latency experiments use a window size of n = 2^22. The top set of experiments uses an
// out-of-order distance of d = 0 and the bottom set uses an out-of-order distance of
// d = 2^20 = 1_048_576. (We chose the latter distance because it is among the worst-performing in the
// throughput experiments.) The experimental setup is the same as for the throughput experiments,
// and the latency is for an entire round of evict, insert, and query. The y-axis is the number of
// processor cycles for a round, in log scale. Since we used a 2.7 GHz machine, 103 cycles take 370
// nanoseconds and 106 cycles take 370 microseconds. The brown bars show the median latency, the
// shaded regions show the distribution of latencies, and the black bars are the 99.9th percentile.
// The range is the minimum and maximum latency.
//
// When the out-of-order distance is 0 and the aggregation operator is cheap or only moderately
// expensive, the worst-case latency in practice for the classic and finger B-trees is similar.
// This is expected, as the time is dominated by tree operations, and they are worst- case O(log
// n). However, the minimum and median latencies are orders of magnitude better for the finger
// B-trees. This is also expected, since for d = 0, the fingers enable amortized O(1) updates. When
// the aggregation operator is expensive, the finger B-trees have significantly lower latency as
// they repair fewer partial aggregates.
//
// With an out-of-order distance of d = 2^20 and cheap or moderately expensive operators, the
// classic and finger B-trees have similar latency. This is expected: as d approaches n, the
// worst-case latency for finger B-trees approaches O(logn). Again, with expensive operators, the
// minimum, median, and 99.9th percentile of the finger B-tree with min-arity 2 is orders of
// magnitude lower than that of classic B-trees. There is, however, a curious effect clearly
// present in the bloom experiments with finger B-trees, but still observable in the others:
// min-arity 2 has the lowest latency; it gets worse with min-arity 4, then improves with min-arity
// 8. Recall that the root may be slimmer than the min-arity. With d = 220, depending on the arity
// of the root, some aggregation repairs walk almost to the root and then back down a spine while
// others walk to the root and no further. The former case, which walks twice the height, is more
// expensive than the latter, which walks just the whole height. The frequency of the expensive
// case is a function of the window size, tree arity, and out-of-order distance, and these factors
// do not interact linearly.
//
// Summary:
// * Window size n = 2^22
// * Out-of-order distance d = 0, d^20-1
// * Measure number of clock cycles per round
fn experiment_2_latency(criterion: &mut Criterion) {
    // Setup
    let mut tree = Tree::<Value, BinOp>::new();
    let timestamps = (2 as Time).pow(22);
    for timestamp in 0..timestamps {
        tree.insert(timestamp, Value(0));
    }
    let mut g = criterion.benchmark_group("bfinger2");
    let g = g.throughput(Throughput::Elements(1));
    // Experiment
    for &exponent in [0, 20].iter() {
        let d = (2 as Time).pow(exponent);
        g.bench_with_input(format!("2^{}", exponent), &d, |bench, d| {
            bench.iter(|| {
                tree.insert(black_box(*d), black_box(Value(0)));
                tree.evict(black_box(*d));
                tree.query();
            })
        });
    }
}

// A special case for FiBA is when d = 0; with in-order data, the theoretical results show that
// FiBA enjoys amortized constant time performance. Figure 13 compares the B-tree-based SWAGs
// against the state-of-the-art SWAGs optimized for first-in, first-out, completely in-order data.
// TwoStacks only works on in-order data and is amortized O(1) with worst-case O(n) [3]. DABA also
// only works on in-order data and is worst-case O(1) [32].
//
// The Reactive Aggregator supports out-of-order evict but requires in-order insert, and is
// amortized O(log n) with worst-case O(n) [33]. The x-axis represents the window size n.
//
// TwoStacks and DABA perform as seen in prior work: for most window sizes, TwoStacks with
// amortized O(1) time has the best throughput. DABA is generally the second best, as it does a
// little more work on each operation to maintain worst-case constant performance.
//
// The finger B-tree variants demonstrate constant performance as the window size increases. The
// best finger B-tree variants stay within 30% of DABA for sum and geomean, but are about 60% off
// of DABA with a more expensive operator like bloom. In general, finger B-trees are able to
// maintain constant performance with completely in-order data, but the extra work of maintaining a
// tree means that SWAGs specialized for in-order data consistently outperform them.
//
// The classic B-trees clearly demonstrate O(log n) behavior as the window size increases. Reactive
// does demonstrate O(log n) behavior, but it is only obvious with bloom. For sum and geomean, the
// fixed costs dominate. Reactive was designed to avoid using pointer-based data structures under
// the premise that the extra memory accesses would harm performance. To our surprise, this is not
// true: on our hardware, the extra computation required to avoid pointers ends up costing more.
// For bloom, Reactive outperforms B-tree based SWAGs because it is essentially a min-arity 1,
// max-arity 2 tree, thus requiring fewer aggregation operations per node.
//
// Summary:
// * Compare against in-order aggregators
// * Window size 2^0 .. 2^22
// * two_stacks, daba, reactive
fn experiment_3_fifo_in_order_data(criterion: &mut Criterion) {}

// One of the benefits of FiBA is that it supports range queries while maintaining logarithmic
// performance for queries over that range. Range queries enable window sharing: a single window
// can support multiple queries over different ranges. An obvious benefit from window sharing is
// reduced space usage, but we also wanted to investigate its time usage. Figure 14 shows that
// window sharing did not consistently improve runtime performance.
//
// The experiments maintain two queries: a big window fixed to size 222, and a small window whose
// size nsmall varies from 1 to 222, shown on the x-axis. The workload consists of out-of-order
// data items where the out-of-order distance d is half of the small window size, i.e., d =
// nsmall/2. The _twin experiments maintain two separate trees, one for each window size. The
// _range experiments maintain a single tree, using a standard query for the big window and a range
// query for the small window.
//
// Our experiment performs out-of-order insert and in-order evict, so insert costs O(log d) and
// evict costs O(1). Hence, on average, each round of the _range experiment costs O(log d) for
// insert, O(1) for evict, and O(1) + O(log nsmall) for query on the big window and the small
// window. On average, each round of the _twin experiment costs 2 · O(log d) for insert, 2 · O(1)
// for evict, and 2 · O(1) for query on the big and small window. Since we chose d = nsmall/2, this
// works out to a total of O(log d) per round in both the _range and the _twin experiments. There
// is no fundamental reason why window sharing is slightly more expensive in practice. A more
// optimized code path might make range queries slightly less expensive, but we would still expect
// them to remain in the same ballpark.
//
// By picking d = nsmall/2, our experiments demonstrate the case where window sharing is the most
// likely to outperform the twin experiment. We could have increased the number of shared win- dows
// to the point where maintaining multiple non-shared windows performed worse because of the memory
// hierarchy, but that is the same benefit as reduced space usage. We conclude that the primary
// benefits of window sharing in this context are reduced space usage and the ability to query
// against arbitrarily-sized windows on the fly.
//
// Summary:
// * Maintain two queries
// * one big window (2^22)
// * one small window (2^0 .. 2^22)
// * d = half of small window size
// * twin: two trees
// * range: single tree
// * measure throughput
// * Out-of-order insert, in-order evict
fn experiment_4_window_sharing(criterion: &mut Criterion) {
    //     let mut tree = Tree::new();
}

// Our real data experiments, Figure 15, use the NYC Citi Bike data [1] for two purposes: to show
// that our techniques work well with real out-of-order data and to showcase time-based windows. We
// use data from August 2018 to December 2018, for a total of about 8 million events. Each event
// includes trip duration, start and stop time, and start and stop location. We use start time as
// the event timestamp and consider events with earlier start time than any prior event to be
// out-of-order. The experimental environment is the same as in Section 5.1 except that it uses
// time-based windows. We vary the window size from 1/4 of a day to 32 days. We calculate the sum
// and geomean over trip duration and bloom over start location.
//
// The real data experiments mirror the trends with synthetic data: the finger B-trees consistently
// outperform their classic counterparts and lower arity trees perform better with more expensive
// operators. The characteristics of the real data experiments are subranges within the spectrum
// explored with synthetic data: the actual size of the
//
// window ranges from about 11, 000 elements for a time window of 1/4 of a day up to about 991, 000
// elements for 32 days.
//
// In this mostly in-order dataset, out-of-order arrivals are generally mild and sporadic, but
// there are bursts of severely out-of-order items, concentrated in about two weeks in November.
// The mean out-of-order distance is d = 56.47 (≈ 85.9 seconds). However, up to 99% of events have
// d ≤ 9 (≈ 149 seconds). The severely out-of-order bursts show up in the last 0.01%, with d ≥ 150,
// 000. The most severe has d ≈ 1 million (17.7 days late).
fn experiment_5_real_data(criterion: &mut Criterion) {}

// How does FiBA perform relative to the state-of-the-art open- source counterparts? To answer this
// question as well as to understand FiBA’s performance characteristics in a different environment,
// we reimplemented both the classic augmented B-tree and FiBA variants in Java inside Apache Flink
// [14]. Apache Flink was chosen because at the time of writing, it is one of the most popular
// open-source streaming platforms and has been the testing ground for many research projects. Our
// Java implementation observes idiomatic Java patterns but is otherwise the same as the C++
// version. All Flink-based experiments were run on a 2-core virtual machine with Intel Xeon
// Platinum 8168 CPU at 2.70GHz, running Ubuntu 18.04.2 LTS with a 4.15 kernel. We compiled and ran
// all experiments with 64-bit OpenJDK 1.8.0_191, using Apache Flink version 1.7.1.
//
// We repeated the distance- varying and FIFO experiments using as baseline Flink’s built-in
// sliding-window aggregation (.aggregate(<AggregateFunction>)). The distance-varying experiment,
// Figure 16, uses window size 213 = 8, 192 items. Though smaller than before, it is enough to
// study the behavior of all the algorithms without choking the baseline. FiBA and the classic
// augmented B-tree perform as seen previously. The throughput of Flink’s built-in algorithm
// remains constant indepen- dent of the out-of-order distance; however, it is orders of magnitude
// slower than the other algorithms due to asymptotical differences.
//
// The FIFO experiment in Figure 17 exhibits the same general trends as before, except that in this
// environment, the FiBA algorithms (bfinger4 and bfinger8, both O(1) time for FIFO input)
// outperform TwoStacks (a specialized O(1)-time algorithm for FIFO), reversing the ranking in the
// C++ experiments. The throughput of Flink’s built-in algorithm decreases linearly with the window
// size and is never competitive. We stopped the Flink experiment at window size n = 213 = 8, 192,
// after which point each run became too expensive.
fn experiment_6_distance_varying_and_fifo(criterion: &mut Criterion) {
    //     let mut tree = Tree::new();
}

// Coarse-grained windows intelli- gently combine items together to reduce the effective window
// size. The coarse-grained window experiment, Figure 18, studies how throughput (y-axis) changes
// with slide granularity (x-axis: how often queries are requested). The window size is 213 = 8,
// 192, and the workload is FIFO. We present two variations on FiBA: vanilla bfinger is the
// standard FiBA algorithm except that queries are only made at the specified granularity, whereas
// cg_bfinger (coarse-grained bfinger) uses FiBA together with slicing, so items that will be
// evicted together are combined into one. This helps reduce the effective window size. We also
// include (New) Scotty, a recent work by Traub et al. [35], which improved upon Scotty [34]. The
// numbers reported are from their flink-connector v.0.3 code on GitHub. Flink’s built-in
// aggregation, though never competitive, is included for reference and shows throughput improving
// linearly with the slide granularity.
//
// As expected, vanilla FiBA algorithms see practically no improve- ment as the slide granularity
// increases: although queries are less frequent, the dominant cost stems from insert/evict
// operations, which remain the same. But Scotty’s throughput improves as the window becomes
// coarser, ultimately outperforming vanilla FiBA (bfinger4, bfinger8) for coarse windows. However,
// FiBA with coarse-grained sliding (cg_bfinger) not only has the best throughput for the whole
// range of granularities but also exhibits performance improvement with coarser sliding
// granularity. This may seem counterintuitive as FiBA is already an O(1)-time algorithm; however,
// because coarse- grained sliding combines items, insert creates a new entry less often and evict
// occurs less frequently—hence, less total work overall.
fn experiment_7_coarse_grained_window(criterion: &mut Criterion) {
    //     let mut tree = Tree::new();
}
