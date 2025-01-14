use bevy::{
    log::info,
    prelude::{Res, Resource},
    time::Time,
};
use bevy_registration::prelude::{init, system};

#[system(Startup)]
fn hello() {
    info!("Hello world!");
}

#[system(Update::Test::First)]
fn first() {
    info!("1");
}

#[system(Update::Test::Second)]
fn second() {
    info!("2");
}

#[system(Update::Test::Third)]
fn third() {
    info!("3");
}

#[system(Update::Test)]
fn random() {
    info!("random");
}

#[system(Update::Test)]
fn fixed_time_step(time: Res<Time>) {
    info!("Time's delta is {} seconds!", time.delta_secs());
}

#[init]
#[derive(Resource, Default)]
pub struct TestResource;

#[system(Update)]
fn resource_tester(resource: Option<Res<TestResource>>) {
    resource.unwrap();
}
