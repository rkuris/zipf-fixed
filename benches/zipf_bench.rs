#![feature(test)]

use rand::distr::Distribution;
extern crate test;

use rand_distr::Zipf as Theirs;
use rand::Rng;

use test::Bencher;

use zipf::Zipf;

#[bench]
fn bench_ours(b: &mut Bencher) {
    let mut rng = rand::rng();
    let d = Zipf::<10>::new(0.5);
    b.iter(|| d.sample(&mut rng));
}

#[bench]
fn bench_theirs(b: &mut Bencher) {
    let mut rng = rand::rng();
    let d = Theirs::new(10., 0.5).unwrap();
    b.iter(|| d.sample(&mut rng));
}

#[bench]
fn bench_rand(b: &mut Bencher) {
    let mut rng = rand::rng();
    b.iter(|| rng.random::<f64>());
}
