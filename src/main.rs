use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
};

use bevy::log;
use bevy::prelude::*;
use twox_hash::XxHash64;

fn main() {
    let mut app = App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(run)
        .run();
}

fn run() {
    let mut h = XxHash64::with_seed(1337);
    let map: BTreeMap<u32, String> = BTreeMap::new();
    map.hash(&mut h);
    let val = h.finish();

    log::info!("{val:016X}");
}
