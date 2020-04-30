// use quickcheck::quickcheck;
// use swag::*;
//
// fn is_sum(pairs: Vec<(Time, Agg)>) -> bool {
//     let sum = pairs.iter().fold(0, |sum, (time, value)| sum + value);
//     let mut tree = Tree::new();
//     for (time, value) in pairs {
//         tree.insert(time, value);
//     }
//     tree.query() == sum
// }
//
// #[test]
// fn test_sum() {
//     //     quickcheck(is_sum as fn(Vec<(Time, Agg)>) -> bool);
// }
