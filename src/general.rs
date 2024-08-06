use bevy::prelude::*;

use crate::Flags;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PostUpdate, deferred_despawn_system);
}
