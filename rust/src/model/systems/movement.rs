use bevy_ecs::prelude::*;

use crate::model::{
    components::{MoveSpeed, Player, SimTransform2D},
    resources::{DeltaTime, PlayerInput},
};

/// Aplica o input ao transform lógico do jogador.
pub(crate) fn player_movement_system(
    input: Res<PlayerInput>,
    delta: Res<DeltaTime>,
    mut players: Query<(&mut SimTransform2D, &MoveSpeed), With<Player>>,
) {
    if input.direction_x == 0.0 && input.direction_y == 0.0 {
        return;
    }

    for (mut transform, speed) in &mut players {
        transform.x += input.direction_x * speed.0 * delta.seconds;
        transform.y += input.direction_y * speed.0 * delta.seconds;
    }
}
