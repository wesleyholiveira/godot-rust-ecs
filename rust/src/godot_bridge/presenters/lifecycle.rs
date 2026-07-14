use godot::{classes::Node, prelude::*};

use crate::presentation::{Present, SpawnCommands};

use super::super::context::GodotPresentationContext;

impl Present<GodotPresentationContext> for SpawnCommands {
    fn present(&mut self, context: &mut GodotPresentationContext) {
        for request in self.drain() {
            if context.views.contains(request.entity) {
                continue;
            }

            let Some(node) = context.scenes.instantiate_player() else {
                godot_error!("Não foi possível instanciar player.tscn");
                continue;
            };

            let Some(root) = context.root.as_mut() else {
                godot_error!("GodotPresentationContext sem root");
                return;
            };

            let node_as_base: Gd<Node> = node.clone().upcast();
            root.add_child(&node_as_base);
            context.views.insert(request.entity, node);
        }
    }
}
