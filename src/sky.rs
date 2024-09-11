use bevy::prelude::*;
yse bevy_atmosphere::{
    model::Atmosphere,
    prelude::{AtmosphereMut, Nishita},
};
use light_consts::lux::AMBIENT_DAYLIGHT;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_sky_system)
        .add_systems(Update, daylight_cycle_system)
        .insert_resource(AtmosphereModel::default())
        .insert_resource(CycleTiimer(Timer::new(
                    bevy::utils::Duration::from_millis(50),
                    TimerMode::Repeating,
                )));
}

#[derive(Component)]
pub struct Sun;

#[derive(Resource, Deref, DerefMut)]
struct CycleTimer(Timer);

