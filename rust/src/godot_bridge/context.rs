use godot::{classes::Node2D, prelude::*};

use super::{scene_factory::SceneFactory, view_registry::ViewRegistry};

/// Estado compartilhado pelos presenters concretos do Godot.
///
/// O derive só recebe `&mut Context`; ele não sabe que este contexto contém
/// Nodes, cenas e o registry Entity -> Node2D.
#[derive(Default)]
pub(crate) struct GodotPresentationContext {
    pub(super) root: Option<Gd<Node2D>>,
    pub(super) views: ViewRegistry,
    pub(super) scenes: SceneFactory,
}

impl GodotPresentationContext {
    pub(crate) fn initialize(&mut self, root: Gd<Node2D>) {
        self.root = Some(root);
        self.scenes.preload_defaults();
    }
}
