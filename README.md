Researching of applying different performance optimizations while implementing a genetic algorithm. 
Optimizations will be applied step by step and each improvement will be measured and compared to initial exec time.
The initial version is written to follow functional programming idioms as much as possible. 
One of the goals of the researching is to compare it to scala implementation of the same algorithm. 

CPU of the computer used to run measurement: Intel Core i5-4460 @ 3.2 GHz.
OS: Linux Ubuntu 17, kernel 4.13.0-17

To try it yourself run: cargo build --release && target/release/genetic-algorithm

Optimization steps and measurements: 

1. Without optimization it runs 100000 generations for about 14368590 ms
2. Dynamic dispatching is replaced by static one: 14304906 ms, mostly the same speed
3. Hash map is replaced by function with pattern matching: 11891678 ms, ~ 20% faster than initial one
4. Instead of parent pairs their position pairs are returned thus reducing cloning: 10931510 ms, ~ 30% faster than initial one
5. Vec of genes is replaced by vec of u64s: 92167 ms, ~ 155 times faster than initial one
6. Second generation is added to incubator to use it as cache thus reducing cloning: 32686 ms, ~ 439 times faster than initial one
7. Cache for decoded values is added thus reducing allocating memory on each decoding genotype: 28774, ~ 499 times faster than initial one