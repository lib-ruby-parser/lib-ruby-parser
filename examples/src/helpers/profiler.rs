#[cfg(not(windows))]
mod implementation {
    extern crate pprof;

    pub struct Profiler {
        enabled: bool,
        guard: Option<pprof::ProfilerGuard<'static>>,
    }

    impl Profiler {
        pub fn new(enabled: bool) -> Self {
            Self {
                enabled,
                guard: None,
            }
        }

        pub fn start(&mut self) {
            if self.enabled {
                self.guard = Some(pprof::ProfilerGuard::new(100).unwrap())
            } else {
                self.guard = None
            }
        }

        pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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

#[cfg(windows)]
mod implementation {
    #[derive(Default, Debug)]
    pub struct Profiler;

    impl Profiler {
        pub fn start(&mut self) {}
        pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
    }

    impl std::str::FromStr for Profiler {
        type Err = String;

        fn from_str(_: &str) -> Result<Self, Self::Err> {
            Ok(Self)
        }
    }
}

pub use implementation::Profiler;
