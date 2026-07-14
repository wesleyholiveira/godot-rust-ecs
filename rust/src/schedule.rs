use bevy_ecs::{prelude::*, schedule::Schedule};

use crate::{
    model::systems::player_movement_system,
    presentation::{extract_added_views, extract_changed_positions},
};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameSet {
    Simulation,
    PresentationExtraction,
}

pub(crate) fn build_schedule() -> Schedule {
    let mut schedule = Schedule::default();

    schedule.configure_sets(
        (GameSet::Simulation, GameSet::PresentationExtraction).chain(),
    );

    schedule.add_systems(
        player_movement_system.in_set(GameSet::Simulation),
    );

    // Ambos escrevem no mesmo PresentationOutput; a ordem serial é explícita.
    schedule.add_systems(
        (extract_added_views, extract_changed_positions)
            .chain()
            .in_set(GameSet::PresentationExtraction),
    );

    schedule
}
