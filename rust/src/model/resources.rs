use bevy_ecs::prelude::*;

/// Snapshot do input capturado pelo Godot para o tick atual.
#[derive(Resource, Default, Debug)]
pub(crate) struct PlayerInput {
    pub(crate) direction_x: f32,
    pub(crate) direction_y: f32,
    pub(crate) spawn_enemy_just_pressed: bool,
    pub(crate) clear_enemies_just_pressed: bool,
}

/// Delta do `_physics_process`, em segundos.
#[derive(Resource, Default, Debug)]
pub(crate) struct DeltaTime {
    pub(crate) seconds: f32,
}

/// Sequência simples para posicionar inimigos numa grade.
#[derive(Resource, Default, Debug)]
pub(crate) struct EnemySpawnSequence {
    pub(crate) next: u32,
}
