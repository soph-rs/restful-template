use soph_schedule::support::Task;

pub fn every_second() -> Task {
    Task::foreground("0/1 * * * * *", || {
        tracing::info!("[task] foreground: passed 1 second");

        Ok(())
    })
}

pub fn every_five_seconds() -> Task {
    Task::background("0/5 * * * * *", || {
        Box::pin(async {
            tracing::info!("[task] background: passed 5 seconds");

            Ok(())
        })
    })
}
