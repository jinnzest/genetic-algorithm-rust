Researching of applying different performance optimizations while implementing a genetic algorithm. 
Optimizations will be applied step by step and each improvement will be measured and compared to initial exec time.
The initial version is written to follow functional programming idioms as much as possible. 
One of the goals of the researching is to compare it to scala implementation of the same algorithm. 

CPU of the computer used to run measurement: Intel(R) Core(TM) i9-10900K CPU @ 3.70GHz.

OS: headless Linux nixos 6.12.63

To try it yourself run: cargo build --release && target/release/genetic-algorithm

Optimization steps and measurements: 

1. Without optimization it runs 100000 generations for about 6313161 ms (~ 105 minutes)
2. Replace a hash map by a function with pattern matching, it runs for about 4929082 ms, ~ 1.28 times faster than the initial one (~ 82 minutes) 
3. Replace vec of genes by vec of u64s, it runs for about 99161 ms, ~ 64 times faster than the initial one (~ 99 seconds)
4. Optimize gray code, it runs for about 60674 ms, ~ 104 times faster than the initial one (~ 60 seconds)
5. Add a second generation pool to incubator to use it as a cache thus reducing cloning, it runs for about 43952 ms, ~ 144 times faster than the initial one (~ 44 seconds)
6. Replace dynamic dispatching by static one, it runs for about 41833 ms, ~ 151 times faster than the initial one (~ 42 seconds)
7. Return parent positions instead of themselfs to reduce cloning , it runs for about 18106 ms, ~ 359 times faster than the initial one (~ 18 seconds)