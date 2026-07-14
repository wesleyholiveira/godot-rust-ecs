use bevy_ecs::{entity::EntityHashMap, prelude::*};
use presentation_derive::PresentOutput;

use crate::model::components::SimTransform2D;

/// Tipo lógico de view. O mapeamento para `PackedScene` fica no lado Godot.
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

/// Lote ordenado de criações de views.
///
/// O `Vec` é drenado após a apresentação, preservando sua capacidade para os
/// próximos ticks.
#[derive(Default, Debug)]
pub(crate) struct SpawnCommands {
    requests: Vec<SpawnView>,
}

impl SpawnCommands {
    fn push(&mut self, request: SpawnView) {
        self.requests.push(request);
    }

    pub(crate) fn drain(
        &mut self,
    ) -> impl Iterator<Item = SpawnView> + '_ {
        self.requests.drain(..)
    }
}

/// Parte espacial do estado final de uma única entidade naquele tick.
///
/// Campos opcionais distinguem "não houve patch" de "aplique este valor".
#[derive(Default, Debug)]
pub(crate) struct SpatialPatch {
    pub(crate) transform: Option<SimTransform2D>,
}

/// Guarda-chuva de patches de apresentação de uma entidade.
///
/// Novos domínios de estado podem ser adicionados aqui no futuro, por exemplo:
/// `visual`, `animation_state` ou `ui`, sem criar um novo mapa por domínio.
#[derive(Default, Debug)]
pub(crate) struct EntityPatch {
    pub(crate) spatial: SpatialPatch,
}

/// Estado final agregado por entidade.
///
/// Existe um único `EntityHashMap` para todos os patches orientados a entidade.
/// Cada nova escrita atualiza o mesmo `EntityPatch`, e o último valor vence.
#[derive(Default, Debug)]
pub(crate) struct EntityPatches {
    patches: EntityHashMap<EntityPatch>,
}

impl EntityPatches {
    fn patch_mut(&mut self, entity: Entity) -> &mut EntityPatch {
        self.patches.entry(entity).or_default()
    }

    fn set_transform(
        &mut self,
        entity: Entity,
        transform: SimTransform2D,
    ) {
        self.patch_mut(entity).spatial.transform = Some(transform);
    }

    /// Esvazia o mapa, mas conserva sua alocação para reutilização.
    pub(crate) fn drain(
        &mut self,
    ) -> impl Iterator<Item = (Entity, EntityPatch)> + '_ {
        self.patches.drain()
    }
}

/// Lote ordenado de remoções de views.
///
/// Assim como os spawns, é drenado após a apresentação para reutilizar a
/// capacidade do `Vec` nos ticks seguintes.
#[derive(Default, Debug)]
pub(crate) struct DespawnCommands {
    entities: Vec<Entity>,
}

impl DespawnCommands {
    fn push(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub(crate) fn drain(
        &mut self,
    ) -> impl Iterator<Item = Entity> + '_ {
        self.entities.drain(..)
    }
}

/// Produto dos resultados independentes de apresentação de um tick.
///
/// `#[present(order = N)]` é lido pelo derive procedural. O número não é uma
/// espera temporal nem um frame: é apenas prioridade crescente de aplicação.
///
/// O mesmo resource permanece no `World`; seus campos são drenados, e não
/// substituídos por estruturas novas a cada tick.
#[derive(Resource, Default, Debug, PresentOutput)]
pub(crate) struct PresentationOutput {
    /// Views precisam existir antes de receber patches.
    #[present(order = 10)]
    spawns: SpawnCommands,

    /// Um único mapa reúne o estado final de cada entidade alterada.
    #[present(order = 20)]
    entities: EntityPatches,

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
        self.entities.set_transform(entity, transform);
    }

    pub(crate) fn despawn_view(&mut self, entity: Entity) {
        self.despawns.push(entity);
    }
}
