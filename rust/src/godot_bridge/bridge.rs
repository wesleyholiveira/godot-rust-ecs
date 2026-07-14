use std::collections::HashMap;

use bevy_ecs::prelude::Entity;
use godot::{
    classes::{Node, Node2D, PackedScene},
    prelude::*,
    tools::try_load,
};

use crate::presentation::{SpawnView, TransformUpdate, ViewCommand, ViewKind};

/// Adapter concreto entre o contrato de apresentação e a API do Godot.
///
/// O bridge é o único componente da arquitetura que conhece simultaneamente
/// `Entity` do Bevy e os tipos concretos do Godot.
#[derive(Default)]
pub(crate) struct GodotBridge {
    /// Pai comum de todas as views criadas pelo ECS.
    root: Option<Gd<Node2D>>,

    /// Associação dinâmica: Entity do Bevy -> Node do Godot.
    views: HashMap<Entity, Gd<Node2D>>,

    /// Cache das cenas para evitar carregamento a cada spawn.
    scenes: HashMap<ViewKind, Gd<PackedScene>>,
}

impl GodotBridge {
    pub(crate) fn initialize(&mut self, root: Gd<Node2D>) {
        self.root = Some(root);
        self.load_scene(ViewKind::Player);
        self.load_scene(ViewKind::Enemy);
    }

    /// Aplica os comandos produzidos pelo proxy depois do `Schedule`.
    ///
    /// `PresentationCommands::take_ordered` já garante a ordem de ciclo de
    /// vida: Spawn -> SetTransform -> Despawn.
    pub(crate) fn apply(
        &mut self,
        commands: impl IntoIterator<Item = ViewCommand>,
    ) {
        for command in commands {
            match command {
                ViewCommand::Spawn(request) => self.spawn_view(request),
                ViewCommand::SetTransform(update) => {
                    self.update_transform(update)
                }
                ViewCommand::Despawn(entity) => self.despawn_view(entity),
            }
        }
    }

    fn scene_path(kind: ViewKind) -> &'static str {
        match kind {
            ViewKind::Player => "res://views/player.tscn",
            ViewKind::Enemy => "res://views/enemy.tscn",
        }
    }

    fn load_scene(&mut self, kind: ViewKind) {
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

    fn spawn_view(&mut self, request: SpawnView) {
        if self.views.contains_key(&request.entity) {
            godot_warn!("A Entity {:?} já possui uma view", request.entity);
            return;
        }

        let Some(scene) = self.scenes.get(&request.kind).cloned() else {
            godot_error!("Cena não carregada para {:?}", request.kind);
            return;
        };

        let Some(root) = self.root.as_mut() else {
            godot_error!("GodotBridge ainda não foi inicializado");
            return;
        };

        let Some(mut node) = scene.try_instantiate_as::<Node2D>() else {
            godot_error!(
                "A raiz de {} precisa herdar Node2D",
                Self::scene_path(request.kind)
            );
            return;
        };

        node.set_position(Vector2::new(
            request.transform.x,
            request.transform.y,
        ));
        node.set_rotation(request.transform.rotation);

        let node_as_base: Gd<Node> = node.clone().upcast();
        root.add_child(&node_as_base);
        self.views.insert(request.entity, node);
    }

    fn update_transform(&mut self, update: TransformUpdate) {
        let Some(node) = self.views.get_mut(&update.entity) else {
            // Entidades puramente lógicas podem existir sem uma view.
            return;
        };

        node.set_position(Vector2::new(
            update.transform.x,
            update.transform.y,
        ));
        node.set_rotation(update.transform.rotation);
    }

    fn despawn_view(&mut self, entity: Entity) {
        let Some(mut node) = self.views.remove(&entity) else {
            return;
        };

        node.queue_free();
    }
}
