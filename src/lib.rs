//! # Bevy Registration
//! Annotate systems, resources, and events with macros and automatically add them to your app.
//! This uses [Inventory](https://crates.io/crates/inventory) internally, so it may not work on all targets.
//! ## Example:
//! ```
//! use bevy::prelude::*;
//! use bevy_registration::prelude::*;
//!
//! // Initiates the resource on the app.
//! #[init]
//! #[derive(Resource, Default)]
//! pub struct TestResource;
//!
//! // Adds the system to the Update schedule.
//! #[system(Update)]
//! fn resource_tester(resource: Option<Res<TestResource>>) {
//!     // This will not panic.
//!     resource.unwrap();
//! }
//!
//! fn main() {
//!     App::new()
//!         // Add bevy's default plugins, to start up the update loop.
//!         .add_plugins(DefaultPlugins)
//!         // Add the registration plugin that will collect the far-away app code.
//!         .add_plugins(RegistrationPlugin)
//!         .run();
//! }
//! ```

use bevy::{
    app::{App, Plugin},
    ecs::schedule::ScheduleLabel,
    prelude::World,
};
use inventory::collect;
#[doc(hidden)]
pub use inventory::submit;
#[doc(hidden)]
pub use paste::paste;

pub mod prelude {
    pub use super::{RegistrationPlugin, app};
    pub use bevy_registration_procedural_macros::{init, schedule, system};
}

/// Iterates through the collected [app functions](AppFunction) and runs each of them.
pub struct RegistrationPlugin;

impl Plugin for RegistrationPlugin {
    fn build(&self, app: &mut App) {
        inventory::iter::<AppFunction>
            .into_iter()
            .for_each(|app_function| {
                (app_function.0)(app);
            });
    }
}

/// A function that gets run on the app.
/// While you can use inventory::collect with this struct, you should instead use the convenient [app macro](app).
#[doc(hidden)]
pub struct AppFunction(pub fn(&mut App));
collect!(AppFunction);

/// Runs a function on the app from anywhere.
/// Accepts a closure or a function's ident. Expects an input of &mut app, with no output.
/// Example:
/// ```
/// app!(|app| {
///     app.add_systems(Startup, || info!("Fun!"));
/// });
/// ```
#[macro_export]
macro_rules! app {
    ($function: expr) => {
        bevy_registration::submit! {
            bevy_registration::AppFunction($function)
        }
    };
}

/// A system that will run a schedule.
/// This is used internally.
pub fn run_schedule<T: ScheduleLabel + Default>(world: &mut World) {
    world.run_schedule(T::default());
}
