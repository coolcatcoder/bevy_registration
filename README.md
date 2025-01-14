# Bevy Registration
Annotate systems, resources, and events with macros and automatically add them to your app.
This uses [Inventory](https://crates.io/crates/inventory) internally, so it may not work on all targets.
## Bevy Versions
| registration version | bevy version |
| -------------------- | ------------ |
| 0.1.0 - 0.2.0        | 0.15         |
## Example:
```rs
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
```
## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
