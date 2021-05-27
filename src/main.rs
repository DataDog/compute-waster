mod config;
mod regulator;
mod rng;

use std::{
    thread,
    time::{Duration, Instant},
};

use config::Config;
use regulator::Regulator;
use rng::Rng;

fn main() -> Result<(), String> {
    let cfg = Config::from_env()?;
    l3_cache(&cfg);
    Ok(())
}

fn l3_cache(cfg: &Config) {
    let mut slab = allocate_slab(cfg);
    let mut rng = rng::Rng::seed_from_u64(0);
    let mut reg = Regulator::new(cfg.cache_hits_per_s, 1_000_000);

    // Cache warmup
    let cache_len = slab.len() as u64 / 2;
    poke_slab(&mut slab, &mut rng, cache_len);
    println!("Finished cache warmups");

    loop {
        let mut now = Instant::now();
        let mut counter = 0.0;
        for _ in 0..u64::MAX {
            while !reg.should_adjust() {
                poke_slab(&mut slab, &mut rng, reg.lap_ops as u64);
                reg.add_lap();
                thread::sleep(Duration::from_micros(cfg.sleep_duration));
                if cfg.debug {
                    counter += reg.lap_ops;
                }
            }
            if cfg.debug && counter > reg.target_ops_per_s {
                println!("{:?}", reg);
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
fn allocate_slab(cfg: &Config) -> Box<[u8]> {
    let slab_size = (cfg.slab_to_cache_ration * cfg.l2_size as f32) as usize;
    println!("Allocating slab of size {} bytes", slab_size);
    vec![0; slab_size].into_boxed_slice()
}

fn poke_slab(slab: &mut [u8], rng: &mut Rng, iterations: u64) {
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
