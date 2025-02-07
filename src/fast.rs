use std::ops::Deref;

use thiserror::Error;

/// A fast Zipf distribution generator.
///
/// This generator takes an `exponent` and a `start` value.
/// The `start` is typically set to 1, and the exponent is
/// typically set to something slightly larger than 1 (like
/// 1.1)
///
/// Increasing `exponent` reduces the likelihood of larger
/// return values. Increasing `start` increases the likelihood
/// of larger return values.
///
/// Algorithm from:
/// "Rejection-Inversion to Generate Variates from Monotone Discrete Distributions"
/// http://eeyore.wu-wien.ac.at/papers/96-04-04.wh-der.ps.gz

#[derive(Debug)]
pub struct ZipfFast {
    hxm: f64,
    hx0_minus_hxm: f64,
    pre: Precomputes,
}

impl Deref for ZipfFast {
    type Target = Precomputes;
    fn deref(&self) -> &Self::Target {
        &self.pre
    }
}

// Precomputed values from the parameters.
// These are used in each iteration and are
// precomputed to save time when generating.
#[derive(Debug)]
pub struct Precomputes {
    // the exponent
    q: f64,
    one_minus_q: f64,
    one_minus_q_inv: f64,

    // the start
    v: f64,
}

impl Precomputes {
    fn new(q: f64, v: f64) -> Self {
        let one_minus_q = 1.0 - q;
        let one_minus_q_inv = 1.0 / one_minus_q;
        Self {
            q,
            one_minus_q,
            one_minus_q_inv,
            v,
        }
    }

    // helper methods
    #[inline]
    fn h(&self, x: f64) -> f64 {
        (self.one_minus_q * (self.v + x).ln()).exp() * self.one_minus_q_inv
    }

    #[inline]
    fn hinv(&self, x: f64) -> f64 {
        ((self.one_minus_q * x).ln() * self.one_minus_q_inv).exp() - self.v
    }
}

#[derive(Debug, Error)]
pub enum ZipfError {
    #[error("Exponent must be greater than 1")]
    InvalidExponent,
    #[error("Start must be >= 1")]
    InvalidStart,
    #[error("Invalid max")]
    InvalidMax,
}

impl ZipfFast {
    pub fn new(exponent: f64, start: f64, imax: u64) -> Result<Self, ZipfError> {
        if exponent <= 1.0 {
            return Err(ZipfError::InvalidExponent);
        }
        if start < 1.0 {
            return Err(ZipfError::InvalidStart);
        }
        let pre = Precomputes::new(exponent, start);
        let imax: f64 = imax as f64;
        let hxm = pre.h(imax + 0.5);
        let hx0_minus_hxm = pre.h(0.5) - (start.ln() * -exponent).exp() - hxm;
        Ok(Self {
            hxm,
            hx0_minus_hxm,
            pre,
        })
    }
}
impl rand::distr::Distribution<u64> for ZipfFast {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> u64 {
        loop {
            let r: f64 = rng.random();
            let ur = self.hxm + r * self.hx0_minus_hxm;
            let x = self.hinv(ur);
            let k = (x + 0.5).floor();
            if k - x < self.q {
                return k as u64;
            }
            if ur >= self.h(k + 0.5) - (-(k + self.v)).ln() {
                return k as u64;
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use rand::distr::Distribution as _;

    #[test]
    fn test_zipf() {
        const N: usize = 100;
        let zipf = ZipfFast::new(1.2, 1.0, (N - 1) as u64).unwrap();
        let mut rng = rand::rng();
        let mut counts = [0; N];
        for _ in 0..N * 10 {
            counts[zipf.sample(&mut rng) as usize] += 1;
        }
        println!("{counts:?}");
        let total = counts.iter().sum::<usize>();
        assert_eq!(total, N * 10);

        // with 1000 samples, we expect the first count to be non-zero.
        assert_ne!(counts[0], 0);
    }
}
