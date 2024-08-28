use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand_chacha::{
    rand_core::{RndCore, SeedableRng},
    ChaCha8Rng,
};

use crate::{general::play_sfx, Flags};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (setup_pellets_system, setup_sfx_system))
        .add_systems(
            Update,
            (
                create_pellets_system,
                move_pellets_system,
                perish_perishables_system,
            ),
        )
}
