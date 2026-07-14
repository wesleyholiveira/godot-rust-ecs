use bevy_ecs::{entity::EntityHashMap, prelude::*};
use presentation_derive::PresentOutput;

use crate::model::components::SimPosition2D;

/// Pedido de criação da view de uma entidade.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SpawnView {
    pub(crate) entity: Entity,
}

/// Operações ordenadas de criação.
#[derive(Default, Debug)]
pub(crate) struct SpawnCommands {
    requests: Vec<SpawnView>,
}

impl SpawnCommands {
    fn push(&mut self, entity: Entity) {
        self.requests.push(SpawnView { entity });
    }

    pub(crate) fn drain(
        &mut self,
    ) -> impl Iterator<Item = SpawnView> + '_ {
        self.requests.drain(..)
    }
}

/// Estado final de apresentação de uma entidade naquele tick.
#[derive(Default, Debug)]
pub(crate) struct EntityPatch {
    pub(crate) position: Option<SimPosition2D>,
}

/// Um único mapa agrega os patches das entidades alteradas.
#[derive(Default, Debug)]
pub(crate) struct EntityPatches {
    patches: EntityHashMap<EntityPatch>,
}

impl EntityPatches {
    fn set_position(&mut self, entity: Entity, position: SimPosition2D) {
        self.patches.entry(entity).or_default().position = Some(position);
    }

    pub(crate) fn drain(
        &mut self,
    ) -> impl Iterator<Item = (Entity, EntityPatch)> + '_ {
        self.patches.drain()
    }
}

/// Saída reutilizável produzida pelos extractors.
#[derive(Resource, Default, Debug, PresentOutput)]
pub(crate) struct PresentationOutput {
    /// A view precisa existir antes de receber seu primeiro patch.
    #[present(order = 10)]
    spawns: SpawnCommands,

    /// Posições finais são aplicadas depois dos spawns.
    #[present(order = 20)]
    entities: EntityPatches,
}

impl PresentationOutput {
    pub(crate) fn spawn_view(&mut self, entity: Entity) {
        self.spawns.push(entity);
    }

    pub(crate) fn set_position(
        &mut self,
        entity: Entity,
        position: SimPosition2D,
    ) {
        self.entities.set_position(entity, position);
    }
}
