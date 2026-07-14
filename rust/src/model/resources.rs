use bevy_ecs::prelude::*;

/// Snapshot do input capturado pelo Godot no tick atual.
#[derive(Resource, Default, Debug)]
pub(crate) struct PlayerInput {
    pub(crate) direction_x: f32,
    pub(crate) direction_y: f32,
}

/// Delta do `_physics_process`, em segundos.
#[derive(Resource, Default, Debug)]
pub(crate) struct DeltaTime {
    pub(crate) seconds: f32,
}
