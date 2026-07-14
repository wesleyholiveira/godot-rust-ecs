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
    presentation::{PresentationFrame, ViewKind, ViewSpec},
    schedule::build_schedule,
};

/// Node Godot que coordena a integração.
///
/// Ele é a fronteira de entrada e saída: captura input do Godot, executa o ECS
/// e entrega o PresentationFrame ao adapter.
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
        world.insert_resource(PresentationFrame::default());

        Self {
            base,
            world,
            schedule: build_schedule(),
            bridge: GodotBridge::default(),
        }
    }

    fn ready(&mut self) {
        // O ready faz apenas o bootstrap. Novas views são criadas/removidas
        // continuamente por PresentationFrame + GodotBridge nos ticks seguintes.
        let mut views_root = Node2D::new_alloc();
        views_root.set_name("EcsViews");
        let views_root_as_base: Gd<Node> = views_root.clone().upcast();
        self.base_mut().add_child(&views_root_as_base);
        self.bridge.initialize(views_root);

        // Cria somente a entidade lógica. No primeiro tick, Added<ViewSpec> será
        // extraído para PresentationFrame.spawns e o bridge instanciará player.tscn.
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
        // ------------------------------------------------------------------
        // 1) Captura do input pela engine Godot.
        // ------------------------------------------------------------------
        let input = Input::singleton();
        let direction = input.get_vector(
            "move_left",
            "move_right",
            "move_up",
            "move_down",
        );

        let spawn_enemy_just_pressed =
            input.is_action_just_pressed("spawn_enemy");
        let clear_enemies_just_pressed =
            input.is_action_just_pressed("clear_enemies");

        // ------------------------------------------------------------------
        // 2) Godot -> Resources do ECS.
        // ------------------------------------------------------------------
        *self.world.resource_mut::<PlayerInput>() = PlayerInput {
            direction_x: direction.x,
            direction_y: direction.y,
            spawn_enemy_just_pressed,
            clear_enemies_just_pressed,
        };
        self.world.resource_mut::<DeltaTime>().seconds = delta as f32;

        // ------------------------------------------------------------------
        // 3) Simulation -> PresentationExtraction -> Cleanup.
        // ------------------------------------------------------------------
        self.schedule.run(&mut self.world);

        // ------------------------------------------------------------------
        // 4) Retira o buffer do tick sem clonar seus Vecs.
        // ------------------------------------------------------------------
        let frame = {
            let mut resource = self.world.resource_mut::<PresentationFrame>();
            std::mem::take(&mut *resource)
        };

        // ------------------------------------------------------------------
        // 5) ECS -> Godot. Só o bridge toca nos Nodes.
        // ------------------------------------------------------------------
        self.bridge.apply(frame);

        // Delimita a rodada de change tracking no uso standalone de bevy_ecs.
        self.world.clear_trackers();
    }
}
