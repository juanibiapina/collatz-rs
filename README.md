# The Collatz Conjecture in Rust

source: http://en.wikipedia.org/wiki/Collatz_conjecture

Take any positive integer n. If n is even, divide it by 2 to get n / 2. If n is
odd, multiply it by 3 and add 1 to obtain 3n + 1. Repeat the process
indefinitely. The conjecture is that no matter what number you start with, you
will always eventually reach 1

Which positive integer below 1 million produces the longest sequence?


# Result

```
time ./target/release/collatz-rs
longest sequence: 837799
./target/release/collatz-rs  0.26s user 0.00s system 98% cpu 0.264 total
```
