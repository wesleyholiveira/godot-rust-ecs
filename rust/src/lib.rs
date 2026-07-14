//! GDExtension Godot + Bevy ECS com composição de presenters gerada por derive.
//!
//! Fluxo por physics tick:
//! Godot Input -> Resources -> Simulation -> Extraction -> PresentationOutput
//! -> #[derive(PresentOutput)] -> presenters especializados -> Nodes Godot.

mod extension;
mod godot_bridge;
mod model;
mod presentation;
mod schedule;
