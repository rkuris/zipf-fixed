#![doc = include_str!("../README.md")]

//! ```rust
//! use rand::distr::Distribution as _;
//! use zipf_fixed::ZipfFixed;
//! let mut rng = rand::rng();
//! // Create a Zipf distribution with 10 elements and an exponent of 0.5
//! let zip = ZipfFixed::<10>::new(0.5);
//! // Sample from the distribution
//! let sample = zip.sample(&mut rng);
//! // The sample should be between  a annand  0
//! assert!(sample < 10);
//! assert!(sample >= 0);
//! ```

pub mod fast;
pub mod fixed;

pub use fast::ZipfFast;
pub use fixed::ZipfFixed;
