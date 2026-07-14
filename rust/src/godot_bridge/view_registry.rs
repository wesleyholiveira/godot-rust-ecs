use bevy_ecs::{entity::EntityHashMap, prelude::Entity};
use godot::{classes::Node2D, prelude::*};

/// Associação ativa entre uma Entity do Bevy e seu Node no Godot.
#[derive(Default)]
pub(crate) struct ViewRegistry {
    views: EntityHashMap<Gd<Node2D>>,
}

impl ViewRegistry {
    pub(crate) fn contains(&self, entity: Entity) -> bool {
        self.views.contains_key(&entity)
    }

    pub(crate) fn insert(&mut self, entity: Entity, view: Gd<Node2D>) {
        self.views.insert(entity, view);
    }

    pub(crate) fn get_mut(
        &mut self,
        entity: Entity,
    ) -> Option<&mut Gd<Node2D>> {
        self.views.get_mut(&entity)
    }
}
