use godot::{classes::Node, prelude::*};

use crate::presentation::{DespawnCommands, Present, SpawnCommands};

use super::super::{
    context::GodotPresentationContext,
    scene_factory::SceneFactory,
};

/// Presenter especializado em criação de views.
impl Present<GodotPresentationContext> for SpawnCommands {
    fn present(self, context: &mut GodotPresentationContext) {
        for request in self.into_requests() {
            if context.views.contains(request.entity) {
                godot_warn!(
                    "A Entity {:?} já possui uma view",
                    request.entity,
                );
                continue;
            }

            let Some(mut node) = context.scenes.instantiate(request.kind) else {
                godot_error!(
                    "Não foi possível instanciar {} como Node2D",
                    SceneFactory::scene_path(request.kind),
                );
                continue;
            };

            let Some(root) = context.root.as_mut() else {
                godot_error!("GodotPresentationContext sem root");
                return;
            };

            node.set_position(Vector2::new(
                request.transform.x,
                request.transform.y,
            ));
            node.set_rotation(request.transform.rotation);

            let node_as_base: Gd<Node> = node.clone().upcast();
            root.add_child(&node_as_base);
            context.views.insert(request.entity, node);
        }
    }
}

/// Presenter especializado em remoção de views.
impl Present<GodotPresentationContext> for DespawnCommands {
    fn present(self, context: &mut GodotPresentationContext) {
        for entity in self.into_entities() {
            let Some(mut node) = context.views.remove(entity) else {
                continue;
            };

            node.queue_free();
        }
    }
}
