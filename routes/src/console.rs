use app::console::commands::Foo;
use soph::prelude::*;
use soph_console::{support::Command, Console};

pub fn commands<A: ApplicationTrait>() -> Console {
    // register custom command here
    Console::new().register(Command::new::<A, Foo>())
}
