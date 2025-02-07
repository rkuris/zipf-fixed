#![feature(test)]

use rand::distr::Distribution;
extern crate test;

use rand::Rng;
use rand_distr::Zipf as Theirs;

use test::Bencher;

use zipf_fixed::{ZipfFast, ZipfFixed};

#[bench]
fn bench_fixed(b: &mut Bencher) {
    let mut rng = rand::rng();
    let d = ZipfFixed::<100>::new(1.2);
    b.iter(|| d.sample(&mut rng));
}

#[bench]
fn bench_fast(b: &mut Bencher) {
    let mut rng = rand::rng();
    let d = ZipfFast::new(1.2, 1.0, 100).unwrap();
    b.iter(|| d.sample(&mut rng));
}

#[bench]
fn bench_theirs(b: &mut Bencher) {
    let mut rng = rand::rng();
    let d = Theirs::new(100., 1.2).unwrap();
    b.iter(|| d.sample(&mut rng));
}

#[bench]
fn bench_rand(b: &mut Bencher) {
    let mut rng = rand::rng();
    b.iter(|| rng.random::<f64>());
}
