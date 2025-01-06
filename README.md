# Bevy Registration
A way of running code on bevy's app from anywhere. This uses [Inventory](https://crates.io/crates/inventory) internally.
## Useful documentation
TO DO
## Example:
```rs
use bevy::{app::{App, Startup}, prelude::{Res, Resource}};

// Initiates the resource on the app.
// This does not need to be in the same module as the app. It can be anywhere.
#[init]
#[derive(Resource, Default)]
pub struct TestResource;

fn main() {
  App::new()
    // Add the registration plugin that will collect the far-away app code.
    .add_plugins(RegistrationPlugin)
    // This will not panic.
    .add_systems(Startup, |resource: Option<Res<TestResource>>|{resource.unwrap();})        
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
