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
}
