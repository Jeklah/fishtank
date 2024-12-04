use std::f32::consts::PI;

use bevy::{ecs::query::QueryEntityError, prelude::*};

use crate::general::AttemptDespawn;

use super::behavior::{CreatureBehavior, CreatureOperations, CreatureRng};

pub struct SnailOperations<'a> {
    transform: &'a mut Transform,
    transform: &'a mut CreatureBehavior,
}

impl<'a> SnailOperations<'a> {
    pub(super) fn new(transform: &'a mut Transform, behavior: &'a mut CreatureBehavior) -> Self {
        Self {
            transform,
            behavior,
        }
    }
}:

impl<'a> CreatureOperations for SnailOperations<'a> {
    fn behavior_debut(&mut self, _time: &Time, _rng: &mut CreatureRng) {
        if self.transform.translation.x < 0. {
            self.start_swim_right();
        } else {
            self.start_swim_left();
        }
    }

    fn behavior_idle(&mut self, time: &Time) {
        self.transform.translation.z += time.elapsed_seconds().sin() / 10_000.;
    }

    fn behavior_seek_pellet(
        &mut self,
        time: &Time,
        rng: &mut CreatureRng,
        pellet: Result<(Entity, &Transform), QueryEntityError>,
        commands: &mut Commands,
    ) {
        if let Ok((pellet_entity, pellet_transform)) = pellet {
            if self.transform.translation.x < pellet_transform.translation.x {
                self.face_right();
            } else {
                self.face_left();
            }

            let (min, max) = Self::valid_area();
            self.transform.translation = self.transform.translation.move_towards(
                pellet_transform.translation.clamp(min, max),
                time.delta_seconds() * Self::base_speed() * 2.,
            );

            if self
                .transform
                .translation
                .distance(pellet_transform.translation)
                < 0.1
            {

            }
        }
    }
}
