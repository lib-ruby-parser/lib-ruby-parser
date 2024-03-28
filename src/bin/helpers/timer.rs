use std::time::Instant;

#[derive(Debug, Clone)]
pub(crate) enum Timer {
    Disabled,
    ReadyToStart,
    Running { started_at: Instant },
}

impl Default for Timer {
    fn default() -> Self {
        Self::Disabled
    }
}

impl Timer {
    pub(crate) fn enabled() -> Self {
        Self::ReadyToStart
    }

    pub(crate) fn start(&mut self) {
        match self {
            Timer::Disabled => {}
            Timer::ReadyToStart => {
                *self = Timer::Running {
                    started_at: Instant::now(),
                }
            }
            Timer::Running { .. } => panic!("Timer is already running"),
        }
    }

    pub(crate) fn stop(&mut self, files_count: usize) {
        match self {
            Timer::Disabled => {}
            Timer::ReadyToStart => panic!("Timer has not started yet"),
            Timer::Running { started_at } => {
                let diff = (Instant::now() - *started_at).as_secs_f64();
                println!("Time taken: {:.10} (total files: {})", diff, files_count);

                *self = Timer::ReadyToStart;
            }
        }
    }
}
