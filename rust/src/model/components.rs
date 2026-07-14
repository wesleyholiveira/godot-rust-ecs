use bevy_ecs::prelude::*;

/// Transform lógico da simulação.
///
/// Este componente é a fonte da verdade para posição e rotação. A posição do
/// Node2D no Godot é apenas uma representação desse estado.
#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct SimTransform2D {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) rotation: f32,
}

/// Velocidade de movimento em pixels por segundo.
#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct MoveSpeed(pub(crate) f32);

/// Marcador da entidade controlada pelo jogador.
#[derive(Component, Debug)]
pub(crate) struct Player;

/// Marcador das entidades inimigas do exemplo.
#[derive(Component, Debug)]
pub(crate) struct Enemy;

/// Marca uma entidade para remoção no fim do tick.
///
/// A entidade não é destruída imediatamente, porque a fase de apresentação
/// precisa primeiro registrar que o Node correspondente deve ser removido.
#[derive(Component, Debug)]
pub(crate) struct DespawnRequested;
