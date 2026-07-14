use bevy_ecs::prelude::*;

/// Posição lógica do modelo, em pixels.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub(crate) struct SimPosition2D {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

/// Velocidade em pixels por segundo.
#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct MoveSpeed(pub(crate) f32);

/// Marca a entidade controlada pelo jogador.
#[derive(Component, Debug)]
pub(crate) struct Player;
