# Compute-waster

Contrary to what the name might make you think, this is not a blockchain!

## Purpose

This program runs useless computations to generate load on components of the CPU.

### L3 data cache

This code allocates a slab a multipe of the size of the l2 cache and pokes it at random indices, trying to trigger a certain number of L3 cache hits per second.

```sh
L2_SIZE=524288 L3_HITS=50000000 cargo run --release
```

### Full usage of a single core

TODO

