//! Exemplo mínimo de Godot + godot-rust + bevy_ecs.
//!
//! Fluxo por physics tick:
//! Godot Input -> Resources -> Simulation -> Extraction serial
//! -> PresentationOutput -> Present<Context> -> Nodes Godot.

mod extension;
mod godot_bridge;
mod model;
mod presentation;
mod schedule;
