use bevy_ecs::prelude::*;
use godot::{
    classes::{INode, Input, Node, Node2D},
    prelude::*,
};

use crate::{
    godot_bridge::GodotBridge,
    model::{
        components::{MoveSpeed, Player, SimTransform2D},
        resources::{DeltaTime, EnemySpawnSequence, PlayerInput},
    },
    presentation::{PresentationOutput, ViewKind, ViewSpec},
    schedule::build_schedule,
};

/// Node Godot que coordena entrada, simulação, extração e apresentação.
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
        world.insert_resource(EnemySpawnSequence::default());
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
        let views_root_as_base: Gd<Node> = views_root.clone().upcast();
        self.base_mut().add_child(&views_root_as_base);
        self.bridge.context_mut().initialize(views_root);

        let player_entity = self
            .world
            .spawn((
                Player,
                SimTransform2D {
                    x: 480.0,
                    y: 220.0,
                    rotation: 0.0,
                },
                MoveSpeed(260.0),
                ViewSpec {
                    kind: ViewKind::Player,
                },
            ))
            .id();

        godot_print!("Player ECS criado: {player_entity:?}");
    }

    fn physics_process(&mut self, delta: f64) {
        // 1) Godot -> Resources.
        let input = Input::singleton();
        let direction = input.get_vector(
            "move_left",
            "move_right",
            "move_up",
            "move_down",
        );

        *self.world.resource_mut::<PlayerInput>() = PlayerInput {
            direction_x: direction.x,
            direction_y: direction.y,
            spawn_enemy_just_pressed: input
                .is_action_just_pressed("spawn_enemy"),
            clear_enemies_just_pressed: input
                .is_action_just_pressed("clear_enemies"),
        };
        self.world.resource_mut::<DeltaTime>().seconds = delta as f32;

        // 2) Simulation -> Extraction -> Cleanup.
        self.schedule.run(&mut self.world);

        // 3) Move o produto de apresentação para fora do World e deixa outro
        // `default()` vazio para o tick seguinte.
        let output = {
            let mut output =
                self.world.resource_mut::<PresentationOutput>();
            std::mem::take(&mut *output)
        };

        // 4) O derive chama cada presenter em ordem crescente de `order`.
        self.bridge.apply(output);

        self.world.clear_trackers();
    }
}
