use godot::prelude::*;

use crate::presentation::{EntityPatches, Present};

use super::super::context::GodotPresentationContext;

impl Present<GodotPresentationContext> for EntityPatches {
    fn present(&mut self, context: &mut GodotPresentationContext) {
        for (entity, patch) in self.drain() {
            let Some(position) = patch.position else {
                continue;
            };

            let Some(view) = context.views.get_mut(entity) else {
                continue;
            };

            view.set_position(Vector2::new(position.x, position.y));
        }
    }
}
