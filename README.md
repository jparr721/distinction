# Distinction
Distinction is a simple rust package which exports a single function which solves the Distinct Elements problem in optimal space and time complexity from the work of [Chakraborty, Vinodchandran, and Meel](https://arxiv.org/pdf/2301.10191). The F0-Estimator algorithm efficiently estimates the number of distinct elements in a data stream using probabilistic techniques. It initializes a probability `p` and an empty set `𝒳`, then iterates over the stream, probabilistically adding elements to `𝒳` with probability `p`. If the size of `𝒳` reaches a predefined threshold, elements are discarded with a 50% probability, and `p` is halved, this ensures that `𝒳` remains small, and we use the calculated value `thresh` as our upper bound on the size of the set. This process continues until the end of the stream, at which point the estimated number of distinct elements is calculated as the size of `𝒳` divided by `p`. This approach ensures space efficiency by keeping `𝒳` small, leveraging probabilistic sampling to handle large data streams effectively.

## Example

```rust
use distinction::{Gen, find_n_distinct};
let mut gen = Gen::new(None);
let stream = vec![1, 10, 20, 10, 10, 30, 20, 10, 20, 20, 1, 1, 1];
let eps = 0.1;
let delta = 0.005;
let n_distinct = find_n_distinct(&stream, eps, delta, Some(gen));
assert_eq!(n_distinct, 4);
```