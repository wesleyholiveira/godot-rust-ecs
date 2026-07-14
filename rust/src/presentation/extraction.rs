use bevy_ecs::prelude::*;

use crate::{
    model::components::{DespawnRequested, SimTransform2D},
    presentation::{PresentationCommands, ViewSpec},
};

/// Converte entidades com `ViewSpec` recém-adicionado em uma intenção de spawn.
///
/// O system usa a API do proxy; ele não manipula a fila diretamente e não
/// conhece o `GodotBridge`.
pub(crate) fn extract_added_views(
    added_views: Query<(Entity, &ViewSpec, &SimTransform2D), Added<ViewSpec>>,
    mut presentation: ResMut<PresentationCommands>,
) {
    for (entity, view, transform) in &added_views {
        presentation.spawn_view(entity, view.kind, *transform);
    }
}

/// Converte transforms inseridos ou alterados em comandos de apresentação.
pub(crate) fn extract_changed_transforms(
    transforms: Query<
        (Entity, &SimTransform2D),
        (
            With<ViewSpec>,
            Changed<SimTransform2D>,
            Without<DespawnRequested>,
        ),
    >,
    mut presentation: ResMut<PresentationCommands>,
) {
    for (entity, transform) in &transforms {
        presentation.set_transform(entity, *transform);
    }
}

/// Registra a remoção da view antes que o cleanup destrua a Entity.
pub(crate) fn extract_despawn_requests(
    entities: Query<Entity, Added<DespawnRequested>>,
    mut presentation: ResMut<PresentationCommands>,
) {
    for entity in &entities {
        presentation.despawn_view(entity);
    }
}
