use bevy::prelude::*;
use lifecycler::AppPlugin;

fn main() {
    App::new().add_plugin(AppPlugin).run();
}
