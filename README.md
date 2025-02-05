# zipf-fixed

Zipf-fixed is an optimized implementation of the [zipf distribution](https://en.wikipedia.org/wiki/Zipf%27s_law) that is compute-heavy
up front in order to perform better each time a sample is retrieved.
It is about 10x faster than rand_distr::Zipf to compute the next number.
The cost for these gains is a longer setup time to create the distribution.

```text
     test bench_ours   ... bench:          21.83 ns/iter (+/- 1.20)
     test bench_theirs ... bench:         183.02 ns/iter (+/- 7.17)
     test bench_rand   ... bench:           9.21 ns/iter (+/- 0.32)
```

The benchmark `bench_theirs` uses `rand_distr::Zipf`.
The `bench_rand` test just indicates the speed in which this machine can produce random float
values.

This code is optimized for small values of N. Each possible value requires 64 bits of storage,
so for N possible values, the storage requirement is N*8 bytes. This is typically not a problem
unless N is very large.

## Running the benchmark

The code works fine on stable, but the benchmarks require the nightly benchmark framework.

```shell
cargo +nightly bench
```

## Examples and Usage
