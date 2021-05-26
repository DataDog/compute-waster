use std::time::Instant;

#[derive(Debug)]
pub struct Regulator {
    last_checked: Instant,
    // Number of iterations to do in a lapse
    pub lap_ops: f64,
    // Number of iterations since last adjustement
    pub ops_counter: f64,
    // Goal of iterations per s
    pub target_ops_per_s: f64,
}

impl Regulator {
    // Proportional correction factor
    const KP: f64 = 0.0001;

    pub fn new(target_ops_per_s: u64, laps_iters: u64) -> Self {
        Self {
            last_checked: Instant::now(),
            ops_counter: 0.0,
            lap_ops: laps_iters as f64,
            target_ops_per_s: target_ops_per_s as f64,
        }
    }

    pub fn add_lap(&mut self) {
        self.ops_counter += self.lap_ops;
    }

    pub fn should_adjust(&self) -> bool {
        self.ops_counter >= (self.target_ops_per_s / 100.0)
    }

    pub fn adjust_lap(&mut self) {
        let elapsed = self.last_checked.elapsed().as_secs_f64();

        // Proportionnal correction
        let correction = Self::KP * ((self.ops_counter / elapsed) - self.target_ops_per_s);
        self.lap_ops = f64::max(self.lap_ops - correction, 1.0).floor();

        self.ops_counter = 0.0;
        self.last_checked = Instant::now();
    }
}
