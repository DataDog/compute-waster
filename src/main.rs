mod rng;

fn main() {
    let l3_size: usize = std::env::var("L3_SIZE")
        .expect("Env variable L3_SIZE is missing")
        .parse()
        .expect("Env variable L3_SIZE should be an integer");
    l3_cache(l3_size)
}

fn l3_cache(l3_size: usize) {
    // Allocating 4 times the l2 cache to trigger cache miss at least 50% of the time
    let mut slab: Vec<u8> = vec![0; l3_size];
    println!("Allocating slab of size {} bytes", l3_size);

    let mut rng = rng::Rng::seed_from_u64(0);

    loop {
        let mut last_val = 0;
        for i in 0..u64::MAX {
            let idx = rng.next_idx(slab.len());
            slab[idx] = slab[idx].wrapping_add(1).wrapping_add(last_val);
            last_val = slab[idx];

            if i % u64::pow(10, 9) == 0 {
                println!("Iteration {}", i)
            }
        }
        // Use result so that it doesn't get discarded by optimizations
        println!("{}", slab.iter().copied().fold(0_u8, u8::wrapping_add));
    }
}
