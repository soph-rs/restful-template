use soph::prelude::*;
use soph_console::Console;

pub struct App;

impl ApplicationTrait for App {
    type Service = Console;

    type Migrator = migration::Migrator;

    fn with_routing() -> impl ServiceTrait {
        routes::api::routes()
    }

    fn with_schedule() -> impl ServiceTrait {
        routes::schedule::tasks::<Self>()
    }

    fn with_console() -> impl ServiceTrait {
        routes::console::commands::<Self>()
    }

    fn with_worker() -> impl ServiceTrait {
        routes::queue::jobs()
    }
}
