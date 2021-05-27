# Compute-waster

Contrary to what the name might make you think, this is not a blockchain!

## Purpose

This program runs useless computations to generate load on components of the CPU.

### L3 data cache

This code allocates a slab a multipe of the size of the l2 cache and pokes it at random indices, trying to trigger a certain number of L3 cache hits per second.

For instance on an Intel Xeon 8259CL, L2 cache 1MiB/core. If we want to hit the LLC 1/3 we have to use these parameters.

```
L2_SIZE=1000000 SLAB_CACHE_RATIO=1.5 L3_HITS=25000000 ./target/release/compute-waster
```

### Full usage of a single core

TODO

