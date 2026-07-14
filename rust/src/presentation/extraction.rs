use bevy_ecs::prelude::*;

use crate::{
    model::components::SimPosition2D,
    presentation::{PresentationOutput, ViewSpec},
};

/// Gera uma criação visual quando `ViewSpec` é adicionado.
pub(crate) fn extract_added_views(
    views: Query<Entity, Added<ViewSpec>>,
    mut output: ResMut<PresentationOutput>,
) {
    for entity in &views {
        output.spawn_view(entity);
    }
}

/// Copia para a apresentação somente posições alteradas.
pub(crate) fn extract_changed_positions(
    positions: Query<
        (Entity, &SimPosition2D),
        (With<ViewSpec>, Changed<SimPosition2D>),
    >,
    mut output: ResMut<PresentationOutput>,
) {
    for (entity, position) in &positions {
        output.set_position(entity, *position);
    }
}
