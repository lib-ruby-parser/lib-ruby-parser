extern crate pprof;
use std::fs::File;

type MaybeProfiler = Option<pprof::ProfilerGuard<'static>>;

pub fn start(enabled: bool) -> MaybeProfiler {
    if enabled {
        Some(pprof::ProfilerGuard::new(100).unwrap())
    } else {
        None
    }
}

pub fn stop(enabled: bool, guard: MaybeProfiler) -> Result<(), pprof::Error> {
    if enabled {
        println!("Creating flamegraph.svg");
        let report = guard.unwrap().report().build()?;
        let file = File::create("flamegraph.svg").unwrap();
        report.flamegraph(file)
    } else {
        Ok(())
    }
}
