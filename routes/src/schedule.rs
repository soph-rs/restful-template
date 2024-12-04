use app::console::schedule;
use soph::prelude::*;
use soph_schedule::Schedule;

pub fn tasks<A: ApplicationTrait>() -> Schedule {
    Schedule::new()
        .register(schedule::every_second())
        .register(schedule::every_five_seconds())
}
