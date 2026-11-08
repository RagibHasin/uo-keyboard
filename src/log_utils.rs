use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use tracing_flight_recorder::Trigger;

const DUMP_INTERVAL: Duration = Duration::from_secs(2);

static SHOULD_DUMP: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub(crate) struct TimedTrigger<O>(pub O);

impl<O: Trigger> Trigger for TimedTrigger<O> {
    fn should_dump(&self, event: &tracing_flight_recorder::CapturedEvent) -> bool {
        self.0.should_dump(event) || SHOULD_DUMP.swap(false, Ordering::Relaxed)
    }

    fn name(&self) -> &str {
        "TimedTrigger"
    }
}

static TIMER_ACTIVE: AtomicBool = AtomicBool::new(true);

pub(crate) fn timed_subscriber() -> tracing_flight_recorder::FlightRecorderLayer {
    init_timer();

    tracing_flight_recorder::FlightRecorderLayer::new(
        tracing_flight_recorder::FlightRecorder::with_default_capacity(),
    )
    .with_dump_on(
        TimedTrigger(tracing_flight_recorder::LevelTrigger::new(
            tracing::Level::ERROR,
        )),
        std::env::temp_dir(),
        "UoBanglaKeyboard",
        10,
    )
}

fn init_timer() {
    thread::spawn(|| {
        while TIMER_ACTIVE.load(Ordering::Relaxed) {
            thread::sleep(DUMP_INTERVAL);
            SHOULD_DUMP.store(true, Ordering::Relaxed);
        }
    });
}

pub(crate) fn shutdown_timer() {
    TIMER_ACTIVE.store(false, Ordering::Relaxed);
    thread::sleep(DUMP_INTERVAL);
}
