# Solution to 'Parallel Letter Frequency' excercise
I tried a couple approaches, with and without Arc/Mutex.  
Default benchmark (in orig folder) was replaced with [Criterion.rs](https://bheisler.github.io/criterion.rs/book/getting_started.html) crate. Below, the benchmark results and the original README from Excercism.  

```
bench_tiny_sequential   time:   [60.048 ns 60.183 ns 60.338 ns]                                  
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

bench_tiny_parallel     time:   [17.647 µs 18.095 µs 18.594 µs]                                 
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

bench_tiny_parallel_no_mutex                                                                             
                        time:   [17.114 µs 17.441 µs 17.777 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

bench_small_sequential  time:   [23.276 µs 23.811 µs 24.420 µs]                                    
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) high mild
  10 (10.00%) high severe

bench_small_parallel    time:   [51.240 µs 51.684 µs 52.099 µs]                                  
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) low severe
  3 (3.00%) low mild
  5 (5.00%) high mild
  5 (5.00%) high severe

bench_small_parallel_no_mutex                                                                             
                        time:   [50.953 µs 51.169 µs 51.385 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

bench_large_sequential  time:   [621.10 µs 630.46 µs 642.35 µs]                                   
Found 24 outliers among 100 measurements (24.00%)
  7 (7.00%) low severe
  6 (6.00%) low mild
  11 (11.00%) high severe

bench_large_parallel    time:   [159.12 µs 160.21 µs 161.48 µs]                                 
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

bench_large_parallel_no_mutex                                                                            
                        time:   [155.95 µs 156.59 µs 157.30 µs]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild
 ``` 

# Parallel Letter Frequency

Welcome to Parallel Letter Frequency on Exercism's Rust Track.
If you need help running the tests or submitting your code, check out `HELP.md`.

## Instructions

Count the frequency of letters in texts using parallel computation.

Parallelism is about doing things in parallel that can also be done
sequentially. A common example is counting the frequency of letters.
Create a function that returns the total frequency of each letter in a
list of texts and that employs parallelism.

Learn more about concurrency in Rust here:

- [Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

## Bonus

This exercise also includes a benchmark, with a sequential implementation as a
baseline. You can compare your solution to the benchmark. Observe the
effect different size inputs have on the performance of each. Can you
surpass the benchmark using concurrent programming techniques?

As of this writing, test::Bencher is unstable and only available on
*nightly* Rust. Run the benchmarks with Cargo:

```
cargo bench
```

If you are using rustup.rs:

```
rustup run nightly cargo bench
```

- [Benchmark tests](https://doc.rust-lang.org/stable/unstable-book/library-features/test.html)

Learn more about nightly Rust:

- [Nightly Rust](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html)
- [Installing Rust nightly](https://rust-lang.github.io/rustup/concepts/channels.html#working-with-nightly-rust)

## Source

### Created by

- @EduardoBautista

### Contributed to by

- @andrewclarkson
- @ashleygwilliams
- @ccouzens
- @ClashTheBunny
- @coriolinus
- @cwhakes
- @EduardoBautista
- @efx
- @ErikSchierboom
- @etrepum
- @glennpratt
- @IanWhitney
- @kytrinyx
- @lutostag
- @mkantor
- @nfiles
- @petertseng
- @rofrol
- @sjwarner-bp
- @stringparser
- @xakon
- @ZapAnton