use std::time::Instant;

#[derive(Debug)]
pub struct Timer {
    enabled: bool,
    started_at: Option<Instant>,
}

impl Timer {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            started_at: None,
        }
    }

    pub fn start(&mut self) {
        if !self.enabled {
            return;
        }
        self.started_at = Some(Instant::now());
    }

    pub fn stop(&mut self, files_count: usize) {
        if !self.enabled {
            return;
        }

        let started_at = self.started_at.take().unwrap();
        let diff = (Instant::now() - started_at).as_secs_f64();
        println!("Time taken: {:.10} (total files: {})", diff, files_count)
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new(false)
    }
}

impl std::str::FromStr for Timer {
    type Err = String;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Timer::new(true))
    }
}
