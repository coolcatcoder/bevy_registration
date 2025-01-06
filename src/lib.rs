//! # Bevy Registration
//! A way of running code on bevy's app from anywhere. This uses [Inventory](https://crates.io/crates/inventory) internally, so it may not work on all targets.
//! ## Example:
//! ```rs
//! use bevy::{app::{App, Startup}, prelude::{Res, Resource}};
//! 
//! // Initiates the resource on the app.
//! // This does not need to be in the same module as the app. It can be anywhere.
//! #[init]
//! #[derive(Resource, Default)]
//! pub struct TestResource;
//! 
//! fn main() {
//!   App::new()
//!     // Add the registration plugin that will collect the far-away app code.
//!     .add_plugins(RegistrationPlugin)
//!     // This will not panic.
//!     .add_systems(Startup, |resource: Option<Res<TestResource>>|{resource.unwrap();})        
//!     .run();
//! }
//! ```

use bevy::app::{App, Plugin};
use inventory::collect;
#[doc(hidden)]
pub use inventory::submit;

pub mod prelude {
    pub use super::{RegistrationPlugin, app};
    pub use procedural_macros::init;
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

// No tests, because I couldn't work them out.
// #[cfg(test)]
// mod tests {
//     use bevy::{app::{App, Startup}, prelude::{Res, Resource}};

//     #[init]
//     #[derive(Resource, Default)]
//     pub struct TestResource;

//     #[test]
//     fn resource() {
//         App::new()
//         .add_plugins(RegistrationPlugin)
//         .add_systems(Startup, |resource: Option<Res<TestResource>>|{resource.unwrap();})        
//         .run();
//     }
// }
