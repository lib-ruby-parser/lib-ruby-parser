#[cfg(feature = "pprof")]
mod profiler {
    extern crate pprof;

    pub(crate) struct Profiler {
        enabled: bool,
        guard: Option<pprof::ProfilerGuard<'static>>,
    }

    impl Profiler {
        pub(crate) fn new(enabled: bool) -> Self {
            Self {
                enabled,
                guard: None,
            }
        }

        pub(crate) fn start(&mut self) {
            if self.enabled {
                self.guard = Some(pprof::ProfilerGuard::new(100).unwrap())
            } else {
                self.guard = None
            }
        }

        pub(crate) fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            if self.enabled {
                println!("Creating flamegraph.svg");
                let report = self.guard.take().unwrap().report().build()?;
                let file = std::fs::File::create("flamegraph.svg").unwrap();
                report.flamegraph(file)?;
            }
            Ok(())
        }
    }

    impl Default for Profiler {
        fn default() -> Self {
            Self::new(false)
        }
    }

    impl std::fmt::Debug for Profiler {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Profiler")
                .field("enabled", &self.enabled)
                .finish()
        }
    }

    impl std::str::FromStr for Profiler {
        type Err = String;

        fn from_str(_: &str) -> Result<Self, Self::Err> {
            Ok(Self::new(true))
        }
    }
}

#[cfg(not(feature = "pprof"))]
mod profiler {
    pub(crate) struct Profiler {}

    impl Profiler {
        pub(crate) fn new(_: bool) -> Self {
            Self {}
        }

        pub(crate) fn start(&mut self) {}

        pub(crate) fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
    }
}

pub(crate) use profiler::Profiler;
