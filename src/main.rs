mod regulator;
mod rng;

use std::{
    env, thread,
    time::{Duration, Instant},
};

use regulator::Regulator;
use rng::Rng;

fn main() {
    let l2_size: usize = std::env::var("L2_SIZE")
        .expect("Env variable L2_SIZE is missing")
        .parse()
        .expect("Env variable L3_SIZE should be an unsigned integer");
    let cache_hits_per_s: u64 = match env::var("L3_HITS") {
        Err(env::VarError::NotPresent) => {
            println!("Env variable L3_HITS is missing, defaults to 100_000_000");
            100_000_000
        }
        Err(env::VarError::NotUnicode(_)) => {
            panic!("Env variable L3_SIZE should be an unsigned integer")
        }
        Ok(v) => v
            .parse()
            .expect("Env variable L3_SIZE should be an unsigned integer"),
    };
    let debug = env::var("DEBUG").map(|v| v == "true").unwrap_or(false);

    l3_cache(cache_hits_per_s, l2_size, debug)
}

fn l3_cache(ops_per_s: u64, l2_size: usize, debug: bool) {
    let mut slab = allocate_slab(l2_size);
    let mut rng = rng::Rng::seed_from_u64(0);
    let mut reg = Regulator::new(ops_per_s, 10_000_000);
    loop {
        let mut now = Instant::now();
        let mut counter = 0.0;
        for _ in 0..u64::MAX {
            while !reg.should_adjust() {
                poke_laps(&mut slab, &mut rng, reg.lap_ops as u64);
                reg.add_lap();
                thread::sleep(Duration::from_micros(200));
                if debug {
                    counter += reg.lap_ops;
                }
            }
            if debug && counter > reg.target_ops_per_s {
                println!("{} in {}ms", counter, now.elapsed().as_millis());
                counter = 0.0;
                now = Instant::now();
            }
            reg.adjust_lap();
        }
        use_slab(&slab)
    }
}

/// Allocates a slab 4 times the l2 cache to trigger L2 miss/L3 hit ~75% of the time
fn allocate_slab(l2_size: usize) -> Box<[u8]> {
    let slab_size = 4 * l2_size;
    println!("Allocating slab of size {} bytes", slab_size);
    vec![0; slab_size].into_boxed_slice()
}

fn poke_laps(slab: &mut [u8], rng: &mut Rng, iterations: u64) {
    let mut last_val = 0;
    for _ in 0..iterations {
        let idx = rng.next_idx(slab.len());
        slab[idx] = slab[idx].wrapping_add(1).wrapping_add(last_val);
        last_val = slab[idx];
    }
}

/// Use the slab so it doesn't get discarded by optimizations
#[inline(never)]
fn use_slab(slab: &[u8]) {
    println!("{}", slab.iter().copied().fold(0_u8, u8::wrapping_add));
}
