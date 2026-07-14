use bevy_ecs::prelude::*;

use crate::{
    model::components::{DespawnRequested, SimTransform2D},
    presentation::{PresentationFrame, SpawnView, TransformUpdate, ViewSpec},
};

/// Converte entidades com `ViewSpec` recém-adicionado em pedidos de spawn.
pub(crate) fn extract_added_views(
    added_views: Query<(Entity, &ViewSpec, &SimTransform2D), Added<ViewSpec>>,
    mut frame: ResMut<PresentationFrame>,
) {
    for (entity, view, transform) in &added_views {
        frame.spawns.push(SpawnView {
            entity,
            kind: view.kind,
            transform: *transform,
        });
    }
}

/// Copia para o frame somente transforms inseridos ou alterados.
pub(crate) fn extract_changed_transforms(
    transforms: Query<
        (Entity, &SimTransform2D),
        (
            With<ViewSpec>,
            Changed<SimTransform2D>,
            Without<DespawnRequested>,
        ),
    >,
    mut frame: ResMut<PresentationFrame>,
) {
    for (entity, transform) in &transforms {
        frame.transforms.push(TransformUpdate {
            entity,
            transform: *transform,
        });
    }
}

/// Registra quais views devem ser removidas antes do cleanup destruir a Entity.
pub(crate) fn extract_despawn_requests(
    entities: Query<Entity, Added<DespawnRequested>>,
    mut frame: ResMut<PresentationFrame>,
) {
    frame.despawns.extend(entities.iter());
}
