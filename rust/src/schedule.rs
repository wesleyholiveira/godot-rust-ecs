use bevy_ecs::{
    prelude::*,
    schedule::{Schedule, SingleThreadedExecutor},
};

use crate::{
    model::systems::{
        cleanup_despawned_entities_system, player_movement_system, request_clear_enemies_system,
        spawn_enemy_system,
    },
    presentation::{extract_added_views, extract_changed_transforms, extract_despawn_requests},
};

/// Fases conceituais do tick.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameSet {
    Simulation,
    PresentationExtraction,
    Cleanup,
}

pub(crate) fn build_schedule() -> Schedule {
    let mut schedule = Schedule::default();

    // Para este exemplo pequeno, single-thread reduz cerimônia e overhead.
    // Todos os acessos à API do Godot continuam fora do Schedule.
    schedule.set_executor(SingleThreadedExecutor::default());

    // O encadeamento garante a ordem das fases. O scheduler aplica os Commands
    // diferidos nos pontos de sincronização necessários entre fases ordenadas.
    schedule.configure_sets(
        (
            GameSet::Simulation,
            GameSet::PresentationExtraction,
            GameSet::Cleanup,
        )
            .chain(),
    );

    schedule.add_systems(
        (
            player_movement_system,
            spawn_enemy_system,
            request_clear_enemies_system,
        )
            .in_set(GameSet::Simulation),
    );

    schedule.add_systems(
        (
            extract_added_views,
            extract_changed_transforms,
            extract_despawn_requests,
        )
            .in_set(GameSet::PresentationExtraction),
    );

    schedule.add_systems(cleanup_despawned_entities_system.in_set(GameSet::Cleanup));

    schedule
}
