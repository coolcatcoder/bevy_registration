use bevy::app::App;

pub fn app() {
    App::new()
        .add_plugins((bevy::DefaultPlugins, bevy_registration::RegistrationPlugin))
        .run();
}
