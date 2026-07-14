use std::collections::HashMap;

use godot::{
    classes::{Node2D, PackedScene},
    prelude::*,
    tools::try_load,
};

use crate::presentation::ViewKind;

/// Cache e fábrica das cenas visuais.
#[derive(Default)]
pub(crate) struct SceneFactory {
    scenes: HashMap<ViewKind, Gd<PackedScene>>,
}

impl SceneFactory {
    pub(crate) fn preload_defaults(&mut self) {
        self.load(ViewKind::Player);
        self.load(ViewKind::Enemy);
    }

    pub(crate) fn instantiate(
        &self,
        kind: ViewKind,
    ) -> Option<Gd<Node2D>> {
        let scene = self.scenes.get(&kind)?.clone();
        scene.try_instantiate_as::<Node2D>()
    }

    pub(crate) fn scene_path(kind: ViewKind) -> &'static str {
        match kind {
            ViewKind::Player => "res://views/player.tscn",
            ViewKind::Enemy => "res://views/enemy.tscn",
        }
    }

    fn load(&mut self, kind: ViewKind) {
        let path = Self::scene_path(kind);

        match try_load::<PackedScene>(path) {
            Ok(scene) => {
                self.scenes.insert(kind, scene);
            }
            Err(error) => {
                godot_error!("Falha ao carregar {path}: {error:?}");
            }
        }
    }
}
