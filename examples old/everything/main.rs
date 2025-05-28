use bevy::prelude::{Startup, Update};
use bevy_registration::prelude::schedule;
use std::time::Duration;

mod app;
mod far;

schedule! {
    Update(
        [run_every(Duration::from_secs_f32(1.5))]
        Test(
            First,
            Second,
            Third,
        ),
    )
}

fn main() {
    app::app();
}
