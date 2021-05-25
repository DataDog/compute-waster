# Compute-waster

Contrary to what the name might make you think, this is not a blockchain!

## Purpose

This program runs useless but expensive computations to generate load on components of the CPU.

### L3 data cache

This code allocates a slab of memory of the size of the l3 and pokes it at random indices.
The aim is to fill the l3 cache, which is shared by multiple cores with garbage.

```sh
L3_SIZE=8388608 cargo run --release
```

### Full usage of a single core

TODO

