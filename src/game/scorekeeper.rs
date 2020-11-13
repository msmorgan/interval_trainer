use std::time::{Instant, Duration};

pub struct Scorekeeper {
    attempt_start: Instant,
    results: Vec<(bool, Duration)>,
}

impl Scorekeeper {
    pub fn new() -> Self {
        Scorekeeper {
            attempt_start: Instant::now(),
            results: vec![],
        }
    }

    pub fn add_result(&mut self, success: bool) -> Duration {
        let duration = self.attempt_start.elapsed();
        self.results.push((success, duration));
        self.attempt_start = Instant::now();
        duration
    }

    pub fn report(&self) {
        let mut correct = 0;
        let mut total_time = Duration::new(0, 0);

        let count = self.results.len();

        for result in self.results.iter() {
            if result.0 {
                correct += 1;
                total_time += result.1;
            }
        }

        println!();
        println!("Final results:");
        println!("  {}% correct.", correct as f32 / count as f32 * 100.0);
        println!(
            "  {:.2} sec. average.",
            total_time.as_secs_f32() / count as f32
        );
    }

    pub fn report_and_exit(&self) -> ! {
        self.report();
        std::process::exit(0);
    }
}
