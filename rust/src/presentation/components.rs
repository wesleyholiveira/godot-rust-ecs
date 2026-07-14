use bevy_ecs::prelude::*;

use super::output::ViewKind;

/// Indica que a entidade precisa de uma representação visual.
#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct ViewSpec {
    pub(crate) kind: ViewKind,
}
