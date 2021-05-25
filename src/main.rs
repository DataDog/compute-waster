mod rng;

fn main() {
    let l2_size: usize = std::env::var("L2_SIZE")
        .expect("Env variable L2_SIZE is missing")
        .parse()
        .expect("Env variable L2_SIZE should be an integer");
    l3_cache(l2_size)
}

fn l3_cache(l2_size: usize) {
    // Allocating 2 times the l2 cache to trigger cache miss at least 50% of the time
    let mut slab: Vec<u8> = vec![0; 2 * l2_size];
    println!("Allocating slab of size {} bytes", 2 * l2_size);

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
