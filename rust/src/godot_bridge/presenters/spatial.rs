use godot::prelude::*;

use crate::presentation::{Present, SpatialPatches};

use super::super::context::GodotPresentationContext;

/// Presenter especializado somente em posição e rotação.
impl Present<GodotPresentationContext> for SpatialPatches {
    fn present(self, context: &mut GodotPresentationContext) {
        for (entity, transform) in self.into_transforms() {
            let Some(node) = context.views.get_mut(entity) else {
                // Entidades puramente lógicas podem não possuir view.
                continue;
            };

            node.set_position(Vector2::new(transform.x, transform.y));
            node.set_rotation(transform.rotation);
        }
    }
}
