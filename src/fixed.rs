use std::array::from_fn;
use std::cmp::Ordering;
#[derive(Clone, Debug, PartialEq)]
pub struct ZipfFixed<const N: usize> {
    sum_probs: [f64; N],
}

impl<const N: usize> ZipfFixed<N> {
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

        ZipfFixed { sum_probs }
    }
}

impl<const N: usize> rand::distr::Distribution<usize> for ZipfFixed<N> {
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
        const N: usize = 100;
        let zipf = ZipfFixed::<N>::new(1.2);
        let mut rng = rand::rng();
        let mut counts = [0; N];
        for _ in 0..N * 10 {
            counts[zipf.sample(&mut rng)] += 1;
        }
        println!("{counts:?}");
        let total = counts.iter().sum::<usize>();
        assert_eq!(total, N * 10);

        // with 1000 samples, we expect the first count to be non-zero.
        assert_ne!(counts[0], 0);
    }
}
