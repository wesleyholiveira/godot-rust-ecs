use bevy_ecs::prelude::*;

use crate::model::{
    components::{MoveSpeed, Player, SimTransform2D},
    resources::{DeltaTime, PlayerInput},
};

/// Aplica o input ao transform lógico do jogador.
///
/// Este system não conhece Input, Node2D ou qualquer outro tipo do Godot.
pub(crate) fn player_movement_system(
    input: Res<PlayerInput>,
    delta: Res<DeltaTime>,
    mut players: Query<(&mut SimTransform2D, &MoveSpeed), With<Player>>,
) {
    // Evita obter acesso mutável ao transform quando não há movimento.
    // Isso também evita marcar o componente como Changed sem necessidade.
    if input.direction_x == 0.0 && input.direction_y == 0.0 {
        return;
    }

    for (mut transform, speed) in &mut players {
        transform.x += input.direction_x * speed.0 * delta.seconds;
        transform.y += input.direction_y * speed.0 * delta.seconds;
    }
}
