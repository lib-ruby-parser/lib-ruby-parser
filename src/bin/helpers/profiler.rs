#[cfg(not(windows))]
mod implementation {
    pub(crate) struct Profiler {
        enabled: bool,
        guard: Option<pprof::ProfilerGuard<'static>>,
    }

    impl Profiler {
        pub(crate) fn enabled() -> Self {
            Self {
                enabled: true,
                guard: None,
            }
        }

        pub(crate) fn disabled() -> Self {
            Self {
                enabled: false,
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

    impl std::fmt::Debug for Profiler {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Profiler")
                .field("enabled", &self.enabled)
                .finish()
        }
    }
}

#[cfg(windows)]
mod implementation {
    #[derive(Debug)]
    pub(crate) struct Profiler;

    impl Profiler {
        pub(crate) fn enabled() -> Self {
            Self
        }
        pub(crate) fn disabled() -> Self {
            Self
        }
        pub(crate) fn start(&mut self) {}
        pub(crate) fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
    }
}

pub(crate) use implementation::Profiler;
