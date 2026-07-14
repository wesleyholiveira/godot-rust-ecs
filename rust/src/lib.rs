//! GDExtension Godot + Bevy ECS.
//!
//! Fluxo de cada physics tick:
//! Godot Input -> Resources -> Systems de simulação -> Systems de extração
//! -> PresentationCommands/ViewCommand -> GodotBridge -> Nodes do Godot.

mod extension;
mod godot_bridge;
mod model;
mod presentation;
mod schedule;
