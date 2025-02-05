#![doc = include_str!("../README.md")]

//! ```rust
//! use rand::distr::Distribution as _;
//! use zipf::Zipf;
//! let mut rng = rand::rng();
//! // Create a Zipf distribution with 10 elements and an exponent of 0.5
//! let zip = Zipf::<10>::new(0.5);
//! // Sample from the distribution
//! let sample = zip.sample(&mut rng);
//! // The sample should be between [0, 10)
//! assert!(sample < 10);
//! assert!(sample >= 0);
//! ```

use std::{array::from_fn, cmp::Ordering};
#[derive(Clone, Debug, PartialEq)]
pub struct Zipf<const N: usize> {
    sum_probs: [f64; N],
}

impl<const N: usize> Zipf<N> {
    pub fn new(exponent: f64) -> Self {
        let mut sum = 0.0;
        let sum_probs: [_; N] = from_fn(|i| match i {
            0 => 0.0,
            _ => {
                let exp = (i as f64).powf(exponent);
                sum += 1.0 / exp;
                exp
            }
        });
        sum = 1.0 / sum;
        let mut last = 0.0;
        let sum_probs = from_fn(|i| match i {
            0 => 0.0,
            _ => {
                last += sum / sum_probs[i];
                last
            }
        });

        Zipf { sum_probs }
    }
}

impl<const N: usize> rand::distr::Distribution<usize> for Zipf<N> {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> usize {
        let r = rng.random();
        self.sum_probs
            .binary_search_by(|f| match f.partial_cmp(&r).unwrap() {
                Ordering::Equal => Ordering::Greater,
                ord => ord,
            })
            .unwrap_err()
            - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::distr::Distribution as _;

    #[test]
    fn test_zipf() {
        const N: usize = 1000;
        let zipf = Zipf::<N>::new(2.0);
        let mut rng = rand::rng();
        let mut counts = [0; N];
        for _ in 0..1000 {
            counts[zipf.sample(&mut rng)] += 1;
        }
        let total = counts.iter().sum::<usize>();
        assert_eq!(total, N);
        
        // with 1000 samples, we expect the first count to be non-zero.
        assert_ne!(counts[0], 0);
    }
}
