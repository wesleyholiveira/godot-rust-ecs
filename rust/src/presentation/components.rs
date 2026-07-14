use bevy_ecs::prelude::*;

/// Indica que a entidade precisa de uma view no Godot.
#[derive(Component, Debug)]
pub(crate) struct ViewSpec;
