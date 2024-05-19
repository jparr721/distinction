use rand::{
    self,
    distributions::{Distribution, Standard},
    rngs::SmallRng,
    Rng, SeedableRng,
};

#[cfg(test)]
use rand::distributions::uniform::{SampleRange, SampleUniform};

pub struct Gen {
    rng: SmallRng,
}

impl Gen {
    pub fn new(seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(seed) => SmallRng::seed_from_u64(seed),
            None => SmallRng::from_entropy(),
        };

        Self { rng }
    }

    /// Generates a random number from the optionally provided seed.
    fn gen<T>(&mut self) -> T
    where
        Standard: Distribution<T>,
    {
        self.rng.gen()
    }

    /// Generates random values in a provided range. Only enabled during testing.
    #[cfg(test)]
    fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.rng.gen_range(range)
    }
}

/// Returns the number of distinct entries in an arbitrary stream. Specifically, this function does
/// not take ownership over any of your data, it operates exclusively on references to it and, therefore,
/// your data does not even need to implement clone. The algorithm is based on the work of
/// [Chakraborty, Vinodchandran, and Meel](https://arxiv.org/pdf/2301.10191) for a simple, sample-based
/// algorithm for finding the number of distinct elements in a stream.
///
/// # Examples
/// ```rust
/// use distinction::{Gen, find_n_distinct};
/// let mut gen = Gen::new(None);
/// let stream = vec![1, 10, 20, 10, 10, 30, 20, 10, 20, 20, 1, 1, 1];
/// let eps = 0.1;
/// let delta = 0.005;
/// let n_distinct = find_n_distinct(&stream, eps, delta, Some(gen));
/// assert_eq!(n_distinct, 4);
/// ```
pub fn find_n_distinct<T>(stream: &Vec<T>, eps: f64, delta: f64, gen: Option<Gen>) -> usize
where
    T: Eq + PartialEq,
{
    #[cfg(feature = "use_logging")]
    #[cfg(not(test))]
    env_logger::init();

    if stream.is_empty() {
        return 0;
    }

    // Get a random number generator for probability checks.
    let mut gen = gen.unwrap_or(Gen::new(None));

    let m = stream.len();
    let mut p = 1.0;
    let mut chi: Vec<&T> = Vec::new();
    let thresh: usize = (12.0 / eps.powf(2.0) * f64::log2((8 * m) as f64 / delta)).ceil() as usize;

    log::info!("Initializing; p = {} m = {} thresh = {}", p, m, thresh);

    for i in 0..m {
        let ai = &stream[i];

        // If a_i exists in \chi, remove it.
        if let Some(pos_i) = chi.iter().position(|x| *x == ai) {
            chi.swap_remove(pos_i);
        }

        // With probability p, \chi \leftarrow \chi \union \{a_i\}
        if gen.gen::<f64>() < p {
            // Add the element to \chi
            chi.push(ai);
        }

        if chi.len() == thresh {
            // Throw away each element of \chi with probability 1/2
            chi.retain(|_| gen.gen::<f64>() >= 0.5);

            p /= 2.0;

            if chi.len() == thresh {
                log::warn!("Exiting due to small threshold after removal of elements.");
                return 0;
            }
        }
    }

    log::info!("Finished calculating; p = {}", p);

    return (chi.len() as f64 / p) as usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    fn ground_truth_unique_naive(stream: Vec<i32>) -> usize {
        let mut stream = stream;
        stream.sort();
        stream.dedup();
        stream.len()
    }

    #[test]
    fn it_works() {
        let _ = env_logger::try_init();
        let mut gen = Gen::new(None);

        let stream: Vec<i32> = (0..100000).map(|_| gen.gen_range(0..2000)).collect();
        let eps = 0.1;
        let delta = 0.005;

        assert_eq!(
            ground_truth_unique_naive(stream.clone()),
            find_n_distinct(&stream, eps, delta, Some(gen))
        );
    }

    quickcheck! {
        fn qc_prop_static_eps_delta(stream: Vec<i32>) -> bool {
            let eps = 0.1;
            let delta = 0.005;
            find_n_distinct(&stream, eps, delta, None) == ground_truth_unique_naive(stream)
        }
    }
}
