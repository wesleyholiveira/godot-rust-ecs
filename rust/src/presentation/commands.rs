use bevy_ecs::prelude::*;

use crate::model::components::SimTransform2D;

/// Tipo lógico da representação visual.
///
/// O modelo conhece apenas este identificador. O mapeamento para uma
/// `PackedScene` concreta pertence ao `GodotBridge`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ViewKind {
    Player,
    Enemy,
}

/// Dados necessários para criar a view de uma entidade.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SpawnView {
    pub(crate) entity: Entity,
    pub(crate) kind: ViewKind,
    pub(crate) transform: SimTransform2D,
}

/// Estado final de transform que deve ser aplicado a uma view existente.
#[derive(Clone, Copy, Debug)]
pub(crate) struct TransformUpdate {
    pub(crate) entity: Entity,
    pub(crate) transform: SimTransform2D,
}

/// Contrato uniforme de saída entre o ECS e a camada de apresentação.
///
/// Cada variante descreve uma intenção visual. Ela não chama o Godot e não
/// guarda `Gd<Node>`, portanto continua independente da engine de apresentação.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ViewCommand {
    Spawn(SpawnView),
    SetTransform(TransformUpdate),
    Despawn(Entity),
}

impl ViewCommand {
    /// Prioridade de aplicação usada para garantir um ciclo de vida seguro:
    ///
    /// 1. cria a view;
    /// 2. atualiza a view;
    /// 3. remove a view.
    fn phase(&self) -> u8 {
        match self {
            Self::Spawn(_) => 0,
            Self::SetTransform(_) => 1,
            Self::Despawn(_) => 2,
        }
    }
}

/// Proxy/fachada de apresentação armazenado como `Resource` do ECS.
///
/// Os systems não manipulam vetores públicos nem conhecem o `GodotBridge`.
/// Eles chamam uma API padronizada (`spawn_view`, `set_transform`,
/// `despawn_view`) e o proxy apenas acumula `ViewCommand`s para execução
/// posterior, depois que o `Schedule` terminar.
///
/// É importante que este proxy seja *deferred*: ele não chama a API do Godot.
#[derive(Resource, Default, Debug)]
pub(crate) struct PresentationCommands {
    queue: Vec<ViewCommand>,
}

impl PresentationCommands {
    /// Registra a intenção de criar uma view para a entidade.
    pub(crate) fn spawn_view(&mut self, entity: Entity, kind: ViewKind, transform: SimTransform2D) {
        self.queue.push(ViewCommand::Spawn(SpawnView {
            entity,
            kind,
            transform,
        }));
    }

    /// Registra o estado final do transform que deve chegar à view.
    pub(crate) fn set_transform(&mut self, entity: Entity, transform: SimTransform2D) {
        self.queue.push(ViewCommand::SetTransform(TransformUpdate {
            entity,
            transform,
        }));
    }

    /// Registra a intenção de remover a view associada à entidade.
    pub(crate) fn despawn_view(&mut self, entity: Entity) {
        self.queue.push(ViewCommand::Despawn(entity));
    }

    /// Move todos os comandos para fora do `World` e deixa a fila vazia.
    ///
    /// A ordenação é estável e garante `Spawn -> SetTransform -> Despawn`,
    /// independentemente da ordem em que os systems de extração executaram.
    pub(crate) fn take_ordered(&mut self) -> Vec<ViewCommand> {
        let mut commands = std::mem::take(&mut self.queue);
        commands.sort_by_key(ViewCommand::phase);
        commands
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.queue.len()
    }
}
