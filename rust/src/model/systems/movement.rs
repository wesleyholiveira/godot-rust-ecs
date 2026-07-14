use bevy_ecs::prelude::*;

use crate::model::{
    components::{MoveSpeed, Player, SimPosition2D},
    resources::{DeltaTime, PlayerInput},
};

/// Atualiza somente o modelo ECS. Este system não conhece Godot.
pub(crate) fn player_movement_system(
    input: Res<PlayerInput>,
    delta: Res<DeltaTime>,
    mut players: Query<(&mut SimPosition2D, &MoveSpeed), With<Player>>,
) {
    if input.direction_x == 0.0 && input.direction_y == 0.0 {
        return;
    }

    for (mut position, speed) in &mut players {
        position.x += input.direction_x * speed.0 * delta.seconds;
        position.y += input.direction_y * speed.0 * delta.seconds;
    }
}
