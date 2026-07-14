use godot::{
    classes::{Node2D, PackedScene},
    prelude::*,
    tools::try_load,
};

const PLAYER_SCENE: &str = "res://views/player.tscn";

/// Mantém a única PackedScene do exemplo carregada.
#[derive(Default)]
pub(crate) struct SceneFactory {
    player: Option<Gd<PackedScene>>,
}

impl SceneFactory {
    pub(crate) fn preload(&mut self) {
        match try_load::<PackedScene>(PLAYER_SCENE) {
            Ok(scene) => self.player = Some(scene),
            Err(error) => {
                godot_error!("Falha ao carregar {PLAYER_SCENE}: {error:?}");
            }
        }
    }

    pub(crate) fn instantiate_player(&self) -> Option<Gd<Node2D>> {
        self.player
            .as_ref()?
            .clone()
            .try_instantiate_as::<Node2D>()
    }
}
