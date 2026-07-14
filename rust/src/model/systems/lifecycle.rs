use bevy_ecs::prelude::*;

use crate::{
    model::{
        components::{DespawnRequested, Enemy, SimTransform2D},
        resources::{EnemySpawnSequence, PlayerInput},
    },
    presentation::{ViewKind, ViewSpec},
};

/// Cria uma entidade inimiga usando a fila estrutural `Commands` do Bevy.
pub(crate) fn spawn_enemy_system(
    input: Res<PlayerInput>,
    mut sequence: ResMut<EnemySpawnSequence>,
    mut commands: Commands,
) {
    if !input.spawn_enemy_just_pressed {
        return;
    }

    let index = sequence.next;
    sequence.next += 1;

    let column = index % 8;
    let row = (index / 8) % 4;

    commands.spawn((
        Enemy,
        SimTransform2D {
            x: 100.0 + column as f32 * 70.0,
            y: 310.0 + row as f32 * 60.0,
            rotation: 0.0,
        },
        ViewSpec {
            kind: ViewKind::Enemy,
        },
    ));
}

/// Marca todos os inimigos para remoção.
pub(crate) fn request_clear_enemies_system(
    input: Res<PlayerInput>,
    enemies: Query<Entity, (With<Enemy>, Without<DespawnRequested>)>,
    mut commands: Commands,
) {
    if !input.clear_enemies_just_pressed {
        return;
    }

    for entity in &enemies {
        commands.entity(entity).insert(DespawnRequested);
    }
}

/// Remove definitivamente as entidades já extraídas para apresentação.
pub(crate) fn cleanup_despawned_entities_system(
    entities: Query<Entity, With<DespawnRequested>>,
    mut commands: Commands,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
