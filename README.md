# cargo-profiler

[![Build Status](https://travis-ci.org/svenstaro/cargo-profiler.svg?branch=master)](https://travis-ci.org/svenstaro/cargo-profiler)
[![Crates.io](https://img.shields.io/crates/v/cargo-profiler.svg)](https://crates.io/crates/cargo-profiler)
[![dependency status](https://deps.rs/repo/github/svenstaro/cargo-profiler/status.svg)](https://deps.rs/repo/github/svenstaro/cargo-profiler)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/svenstaro/cargo-profiler/blob/master/LICENSE)

**Cargo subcommand to profile binaries**

## To install

NOTE: This subcommand can only be used on Linux machines.

First install valgrind:

```
$ sudo apt-get install valgrind
```

Then you can install `cargo-profiler` via `cargo install`.

Alternatively you can install directly from the git repository:

```
$ cargo install --git https://github.com/svenstaro/cargo-profiler.git
```

Or clone this repository and build from source:

```
$ cargo install --path .
```

## To run

Cargo profiler currently supports callgrind and cachegrind.

You can call cargo profiler anywhere in a rust project directory with a `Cargo.toml`.

```
$ cargo profiler callgrind
$ cargo profiler cachegrind --release
```

You can also specify a binary directly:

```
$ cargo profiler callgrind --bin $PATH_TO_BINARY
```

To specify command line arguments to the executable being profiled, append them
after a `--`:

```
$ cargo profiler callgrind --bin $PATH_TO_BINARY -- -a 3 --like this
```

You can chose to keep the callgrind/cachegrind output files using the `--keep` option

```
$ cargo profiler callgrind --keep
```

You can limit the number of functions you'd like to look at:

```
$ cargo profiler callgrind --bin ./target/debug/rsmat -n 10

Profiling rsmat with callgrind...

Total Instructions...198,466,456

78,346,775 (39.5%) dgemm_kernel.rs:matrixmultiply::gemm::masked_kernel
-----------------------------------------------------------------------
23,528,320 (11.9%) iter.rs:_..std..ops..Range..A....as..std..iter..Iterator..::next
-----------------------------------------------------------------------
16,824,925 (8.5%) loopmacros.rs:matrixmultiply::gemm::masked_kernel
-----------------------------------------------------------------------
10,236,864 (5.2%) mem.rs:core::mem::swap
-----------------------------------------------------------------------
7,712,846 (3.9%) memset.S:memset
-----------------------------------------------------------------------
7,197,344 (3.6%) ???:core::cmp::impls::_..impl..cmp..PartialOrd..for..usize..::lt
-----------------------------------------------------------------------
6,979,680 (3.5%) ops.rs:_..usize..as..ops..Add..::add
-----------------------------------------------------------------------

```

With cachegrind, you can also sort the data by a particular metric column:

```
$ cargo profiler cachegrind --bin ./target/debug/rsmat -n 10 --sort dr

Profiling rsmat with cachegrind...

Total Memory Accesses...320,385,356

Total L1 I-Cache Misses...371 (0%)
Total LL I-Cache Misses...308 (0%)
Total L1 D-Cache Misses...58,549 (0%)
Total LL D-Cache Misses...8,451 (0%)

 Ir  I1mr ILmr  Dr  D1mr DLmr  Dw  D1mw DLmw
0.40 0.18 0.21 0.35 0.93 1.00 0.38 0.00 0.00 dgemm_kernel.rs:matrixmultiply::gemm::masked_kernel
-----------------------------------------------------------------------
0.08 0.04 0.05 0.12 0.00 0.00 0.02 0.00 0.00 loopmacros.rs:matrixmultiply::gemm::masked_kernel
-----------------------------------------------------------------------
0.12 0.02 0.02 0.10 0.00 0.00 0.15 0.00 0.00 iter.rs:_std..ops..RangeAasstd..iter..Iterator::next
-----------------------------------------------------------------------
0.05 0.01 0.01 0.07 0.00 0.00 0.08 0.00 0.00 mem.rs:core::mem::swap
-----------------------------------------------------------------------
0.03 0.00 0.00 0.05 0.00 0.00 0.00 0.00 0.00 ???:core::cmp::impls::_implcmp..PartialOrdforusize::lt
-----------------------------------------------------------------------
0.03 0.01 0.01 0.04 0.00 0.00 0.03 0.00 0.00 ops.rs:_busizeasops..Addausize::add
-----------------------------------------------------------------------
0.04 0.01 0.01 0.04 0.00 0.00 0.03 0.00 0.00 ptr.rs:core::ptr::_implconstT::offset
-----------------------------------------------------------------------
0.02 0.01 0.00 0.03 0.00 0.00 0.01 0.00 0.00 ???:_usizeasops..Add::add
-----------------------------------------------------------------------
0.01 0.01 0.01 0.02 0.00 0.00 0.01 0.00 0.00 mem.rs:core::mem::uninitialized
-----------------------------------------------------------------------
0.02 0.01 0.01 0.02 0.00 0.00 0.04 0.00 0.00 wrapping.rs:_XorShiftRngasRng::next_u32
-----------------------------------------------------------------------


```

## What are the cachegrind metrics?

* Ir -> Total Instructions
* I1mr -> Level 1 I-Cache misses
* ILmr -> Last Level I-Cache misses
* Dr -> Total Memory Reads
* D1mr -> Level 1 D-Cache read misses
* DLmr -> Last Level D-cache read misses
* Dw -> Total Memory Writes
* D1mw -> Level 1 D-Cache write misses
* DLmw -> Last Level D-cache write misses

## TODO

* cmp subcommand - compare binary profiles
* profiler macros
* better context around expensive functions
* support for more profiling tools
