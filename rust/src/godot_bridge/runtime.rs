use bevy_ecs::prelude::*;
use godot::{
    classes::{INode, Input, Node, Node2D},
    prelude::*,
};

use crate::{
    godot_bridge::GodotBridge,
    model::{
        components::{MoveSpeed, Player, SimPosition2D},
        resources::{DeltaTime, PlayerInput},
    },
    presentation::{PresentationOutput, ViewSpec},
    schedule::build_schedule,
};

/// Node que coordena Godot -> ECS -> apresentação.
#[derive(GodotClass)]
#[class(base = Node)]
pub(crate) struct EcsRuntime {
    base: Base<Node>,
    world: World,
    schedule: Schedule,
    bridge: GodotBridge,
}

#[godot_api]
impl INode for EcsRuntime {
    fn init(base: Base<Node>) -> Self {
        let mut world = World::new();
        world.insert_resource(PlayerInput::default());
        world.insert_resource(DeltaTime::default());
        world.insert_resource(PresentationOutput::default());

        Self {
            base,
            world,
            schedule: build_schedule(),
            bridge: GodotBridge::default(),
        }
    }

    fn ready(&mut self) {
        let mut views_root = Node2D::new_alloc();
        views_root.set_name("EcsViews");
        let views_root_as_node: Gd<Node> = views_root.clone().upcast();
        self.base_mut().add_child(&views_root_as_node);
        self.bridge.context_mut().initialize(views_root);

        self.world.spawn((
            Player,
            SimPosition2D { x: 480.0, y: 270.0 },
            MoveSpeed(260.0),
            ViewSpec,
        ));
    }

    fn physics_process(&mut self, delta: f64) {
        let direction = Input::singleton().get_vector(
            "move_left",
            "move_right",
            "move_up",
            "move_down",
        );

        *self.world.resource_mut::<PlayerInput>() = PlayerInput {
            direction_x: direction.x,
            direction_y: direction.y,
        };
        self.world.resource_mut::<DeltaTime>().seconds = delta as f32;

        self.schedule.run(&mut self.world);

        {
            let world = &mut self.world;
            let bridge = &mut self.bridge;
            let mut output = world.resource_mut::<PresentationOutput>();
            bridge.apply(&mut *output);
        }

        self.world.clear_trackers();
    }
}
