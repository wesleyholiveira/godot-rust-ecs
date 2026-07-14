use bevy_ecs::{prelude::*, schedule::Schedule};

use crate::{
    model::systems::{
        cleanup_despawned_entities_system, player_movement_system, request_clear_enemies_system,
        spawn_enemy_system,
    },
    presentation::{extract_added_views, extract_changed_transforms, extract_despawn_requests},
};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameSet {
    Simulation,
    PresentationExtraction,
    Cleanup,
}

pub(crate) fn build_schedule() -> Schedule {
    let mut schedule = Schedule::default();

    schedule.configure_sets(
        (
            GameSet::Simulation,
            GameSet::PresentationExtraction,
            GameSet::Cleanup,
        )
            .chain(),
    );

    // Systems de simulação continuam livres para o executor do Bevy organizar
    // conforme seus acessos a componentes e resources.
    schedule.add_systems(
        (
            player_movement_system,
            spawn_enemy_system,
            request_clear_enemies_system,
        )
            .in_set(GameSet::Simulation),
    );

    // Todos os extractors globais escrevem no mesmo `PresentationOutput`.
    // O `.chain()` torna a serialização e a ordem intencionais e visíveis:
    //
    // added views -> patches finais -> pedidos de despawn.
    schedule.add_systems(
        (
            extract_added_views,
            extract_changed_transforms,
            extract_despawn_requests,
        )
            .chain()
            .in_set(GameSet::PresentationExtraction),
    );

    schedule.add_systems(cleanup_despawned_entities_system.in_set(GameSet::Cleanup));

    schedule
}
