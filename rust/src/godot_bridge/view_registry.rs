use bevy_ecs::{entity::EntityHashMap, prelude::Entity};
use godot::{classes::Node2D, prelude::*};

/// Mantém somente a associação lógica -> visual.
///
/// `EntityHashMap` usa o hasher especializado do Bevy para chaves `Entity`.
#[derive(Default)]
pub(crate) struct ViewRegistry {
    views: EntityHashMap<Gd<Node2D>>,
}

impl ViewRegistry {
    pub(crate) fn contains(&self, entity: Entity) -> bool {
        self.views.contains_key(&entity)
    }

    pub(crate) fn insert(&mut self, entity: Entity, node: Gd<Node2D>) {
        self.views.insert(entity, node);
    }

    pub(crate) fn get_mut(
        &mut self,
        entity: Entity,
    ) -> Option<&mut Gd<Node2D>> {
        self.views.get_mut(&entity)
    }

    pub(crate) fn remove(
        &mut self,
        entity: Entity,
    ) -> Option<Gd<Node2D>> {
        self.views.remove(&entity)
    }
}
