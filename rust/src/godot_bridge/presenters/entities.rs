use godot::prelude::*;

use crate::presentation::{EntityPatch, EntityPatches, Present};

use super::super::context::GodotPresentationContext;

/// Aplica todos os patches finais de uma entidade usando um único lookup no
/// `ViewRegistry` por entidade alterada.
impl Present<GodotPresentationContext> for EntityPatches {
    fn present(&mut self, context: &mut GodotPresentationContext) {
        for (entity, patch) in self.drain() {
            let Some(node) = context.views.get_mut(entity) else {
                // Entidades puramente lógicas podem não possuir view.
                continue;
            };

            apply_entity_patch(patch, node);
        }
    }
}

fn apply_entity_patch(
    patch: EntityPatch,
    node: &mut Gd<godot::classes::Node2D>,
) {
    if let Some(transform) = patch.spatial.transform {
        node.set_position(Vector2::new(transform.x, transform.y));
        node.set_rotation(transform.rotation);
    }
}
