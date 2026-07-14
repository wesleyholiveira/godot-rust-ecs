use bevy_ecs::prelude::*;

use super::contracts::ViewKind;

/// Componente de apresentação: indica que a entidade precisa de uma view.
///
/// Ele contém apenas um identificador lógico; não armazena Node ou PackedScene.
#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct ViewSpec {
    pub(crate) kind: ViewKind,
}
