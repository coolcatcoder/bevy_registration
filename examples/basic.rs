use bevy::prelude::*;
use bevy_registration::prelude::*;

// Initiates the resource on the app.
#[init]
#[derive(Resource, Default)]
pub struct TestResource;

// Adds the system to the Update schedule.
#[system(Update)]
fn resource_tester(resource: Option<Res<TestResource>>) {
    // This will not panic.
    resource.unwrap();
}

fn main() {
    App::new()
        // Add bevy's default plugins, to start up the update loop.
        .add_plugins(DefaultPlugins)
        // Add the registration plugin that will collect the far-away app code.
        .add_plugins(RegistrationPlugin)
        .run();
}
