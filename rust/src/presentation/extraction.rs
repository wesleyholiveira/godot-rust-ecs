use bevy_ecs::prelude::*;

use crate::{
    model::components::{DespawnRequested, SimTransform2D},
    presentation::{PresentationOutput, ViewSpec},
};

/// Entidades com ViewSpec recém-adicionado geram pedidos de criação.
pub(crate) fn extract_added_views(
    added_views: Query<(Entity, &ViewSpec, &SimTransform2D), Added<ViewSpec>>,
    mut output: ResMut<PresentationOutput>,
) {
    for (entity, view, transform) in &added_views {
        output.spawn_view(entity, view.kind, *transform);
    }
}

/// Transforms alterados viram patches espaciais; a última escrita por Entity
/// substitui as anteriores no mesmo tick.
pub(crate) fn extract_changed_transforms(
    transforms: Query<
        (Entity, &SimTransform2D),
        (
            With<ViewSpec>,
            Changed<SimTransform2D>,
            Without<DespawnRequested>,
        ),
    >,
    mut output: ResMut<PresentationOutput>,
) {
    for (entity, transform) in &transforms {
        output.set_transform(entity, *transform);
    }
}

/// Registra a remoção visual antes do cleanup destruir a Entity.
pub(crate) fn extract_despawn_requests(
    entities: Query<Entity, Added<DespawnRequested>>,
    mut output: ResMut<PresentationOutput>,
) {
    for entity in &entities {
        output.despawn_view(entity);
    }
}
