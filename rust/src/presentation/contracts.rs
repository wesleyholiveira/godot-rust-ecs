use bevy_ecs::prelude::*;

use crate::model::components::SimTransform2D;

/// Tipo lógico da representação visual.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ViewKind {
    Player,
    Enemy,
}

/// Pedido para criar um Node que represente uma entidade.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SpawnView {
    pub(crate) entity: Entity,
    pub(crate) kind: ViewKind,
    pub(crate) transform: SimTransform2D,
}

/// Estado final de transform a ser aplicado a um Node existente.
#[derive(Clone, Copy, Debug)]
pub(crate) struct TransformUpdate {
    pub(crate) entity: Entity,
    pub(crate) transform: SimTransform2D,
}

/// Buffer de saída de um único physics tick.
///
/// Não é um tipo do Bevy: é o contrato do projeto entre ECS e Godot. Os systems
/// de extração preenchem os vetores; o EcsRuntime retira o frame e o entrega ao
/// GodotBridge depois que o Schedule termina.
#[derive(Resource, Default, Debug)]
pub(crate) struct PresentationFrame {
    pub(crate) spawns: Vec<SpawnView>,
    pub(crate) transforms: Vec<TransformUpdate>,
    pub(crate) despawns: Vec<Entity>,
}
