use std::collections::HashMap;

use bevy_ecs::prelude::*;
use presentation_derive::PresentOutput;

use crate::model::components::SimTransform2D;

/// Tipo lógico de view. O mapeamento para PackedScene fica no lado Godot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ViewKind {
    Player,
    Enemy,
}

/// Pedido de criação de uma view.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SpawnView {
    pub(crate) entity: Entity,
    pub(crate) kind: ViewKind,
    pub(crate) transform: SimTransform2D,
}

/// Lote de criações. A ordem dos itens é preservada.
#[derive(Default, Debug)]
pub(crate) struct SpawnCommands {
    requests: Vec<SpawnView>,
}

impl SpawnCommands {
    fn push(&mut self, request: SpawnView) {
        self.requests.push(request);
    }

    pub(crate) fn into_requests(self) -> Vec<SpawnView> {
        self.requests
    }
}

/// Patches de estado espacial.
///
/// Um `HashMap` é usado porque somente o transform final de cada entidade
/// interessa ao Godot naquele tick: a última escrita vence.
#[derive(Default, Debug)]
pub(crate) struct SpatialPatches {
    transforms: HashMap<Entity, SimTransform2D>,
}

impl SpatialPatches {
    fn set_transform(
        &mut self,
        entity: Entity,
        transform: SimTransform2D,
    ) {
        self.transforms.insert(entity, transform);
    }

    pub(crate) fn into_transforms(
        self,
    ) -> HashMap<Entity, SimTransform2D> {
        self.transforms
    }
}

/// Lote de remoções de views.
#[derive(Default, Debug)]
pub(crate) struct DespawnCommands {
    entities: Vec<Entity>,
}

impl DespawnCommands {
    fn push(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub(crate) fn into_entities(self) -> Vec<Entity> {
        self.entities
    }
}

/// Produto dos resultados independentes de apresentação de um tick.
///
/// `#[present(order = N)]` é lido pelo derive procedural. O número não é uma
/// espera temporal nem um frame: é apenas prioridade crescente de aplicação.
/// Deixamos intervalos (10, 20, 90) para inserir novos domínios depois sem
/// renumerar os existentes, por exemplo animação em 30 e áudio em 40.
#[derive(Resource, Default, Debug, PresentOutput)]
pub(crate) struct PresentationOutput {
    /// Views precisam existir antes de receber patches.
    #[present(order = 10)]
    spawns: SpawnCommands,

    /// Atualizações espaciais ocorrem depois dos spawns.
    #[present(order = 20)]
    spatial: SpatialPatches,

    /// Remoções são aplicadas por último.
    #[present(order = 90)]
    despawns: DespawnCommands,
}

impl PresentationOutput {
    pub(crate) fn spawn_view(
        &mut self,
        entity: Entity,
        kind: ViewKind,
        transform: SimTransform2D,
    ) {
        self.spawns.push(SpawnView {
            entity,
            kind,
            transform,
        });
    }

    pub(crate) fn set_transform(
        &mut self,
        entity: Entity,
        transform: SimTransform2D,
    ) {
        self.spatial.set_transform(entity, transform);
    }

    pub(crate) fn despawn_view(&mut self, entity: Entity) {
        self.despawns.push(entity);
    }
}
