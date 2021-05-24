mod rng;

const SIZE: usize = usize::pow(2, 20);

fn main() {
    let mut slab: Vec<u8> = vec![0; SIZE];
    println!("Allocating slab of size {} bytes", SIZE);

    let mut rng = rng::Rng::seed_from_u64(0);
    let mut last_val = 0;
    for i in 0..u64::MAX {
        let idx = rng.next_idx(slab.len());
        slab[idx] = slab[idx].wrapping_add(1).wrapping_add(last_val);
        last_val = slab[idx];
        if i % u64::pow(10, 9) == 0 {
            println!("Iteration {}", i)
        }
    }

    // Use value so that it doesn't get discarded by optimizations
    println!(
        "{}",
        (&slab).into_iter().fold(0_u8, |a, &b| a.wrapping_add(b))
    )
}
