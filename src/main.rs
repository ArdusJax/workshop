mod stats {
    #[derive(Debug)]
    pub struct Stats {
        pub max: f64,
        pub mean: f64,
        pub min: f64,
        pub size: u64,
        pub sum: f64,
    }

    impl Stats {
        pub fn update(&mut self, val: f64) {
            if val > self.max {
                self.max = val;
            };
            if self.min == 0.0 {
                self.min = val
            } else if val < self.min {
                self.min = val;
            };
            self.sum += val;
            self.size += 1;
            self.mean = self.sum / self.size as f64;
        }
    }
}

fn main() {
    use crate::stats::Stats;

    let mut s = Stats {
        max: 0.0,
        mean: 0.0,
        min: 0.0,
        size: 0,
        sum: 0.0,
    };

    s.update(56.0);
    s.update(32.0);
    s.update(12.0);

    println!("{:?}", s);
}
