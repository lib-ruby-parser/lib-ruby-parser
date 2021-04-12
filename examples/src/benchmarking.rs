use std::time::Instant;

pub fn start(enabled: bool) -> Option<Instant> {
    if enabled {
        return Some(Instant::now());
    }
    None
}

pub fn stop(enabled: bool, started_at: Option<Instant>, files_count: usize) {
    if enabled {
        let started_at = started_at.unwrap();
        let diff = (Instant::now() - started_at).as_secs_f64();
        println!("Time taken: {:.10} (total files: {})", diff, files_count)
    }
}
