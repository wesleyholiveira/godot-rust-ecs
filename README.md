# Godot + godot-rust + Bevy ECS — derive procedural de apresentação

Projeto mínimo funcional com:

- `godot-rust 0.5.4`;
- `bevy_ecs 0.19.0`;
- modelo ECS agnóstico ao Godot;
- systems de simulação e extração separados;
- `PresentationOutput` como produto dos resultados do tick;
- presenters especializados por domínio;
- `GodotBridge` como fachada fina, sem loops por comando;
- derive procedural local `PresentOutput`;
- atributos `#[present(order = N)]` para ordenar presenters em compilação.

## Estrutura

```text
godot_bevy_ecs_derive_demo/
├── rust/
│   ├── Cargo.toml                 # crate principal + workspace
│   ├── presentation_derive/       # crate proc-macro
│   │   ├── Cargo.toml
│   │   ├── src/lib.rs
│   │   └── tests/derive.rs
│   └── src/
│       ├── model/
│       ├── presentation/
│       ├── godot_bridge/
│       │   └── presenters/
│       ├── schedule.rs
│       └── lib.rs
└── godot/
```

## O que significa `#[present(order = N)]`

`order` é uma **prioridade de aplicação**, não um atraso e não um número de
frame. O derive ordena os campos pelo inteiro em ordem crescente e gera as
chamadas a `Present::present`.

```rust
#[derive(Resource, Default, PresentOutput)]
struct PresentationOutput {
    #[present(order = 10)]
    spawns: SpawnCommands,

    #[present(order = 20)]
    spatial: SpatialPatches,

    #[present(order = 90)]
    despawns: DespawnCommands,
}
```

A expansão conceitual é:

```rust
impl<C> Present<C> for PresentationOutput
where
    SpawnCommands: Present<C>,
    SpatialPatches: Present<C>,
    DespawnCommands: Present<C>,
{
    fn present(self, context: &mut C) {
        self.spawns.present(context);   // 10
        self.spatial.present(context);  // 20
        self.despawns.present(context); // 90
    }
}
```

Os intervalos 10, 20 e 90 permitem inserir novos domínios depois:

```text
10 spawn
20 spatial
30 animation
40 audio
50 UI
90 despawn
```

Sem precisar renumerar tudo. Ordens duplicadas geram erro de compilação.
Campos sem `#[present(order = N)]` também geram erro.

## Por que isso evita o bridge inchado

O bridge agora só coordena:

```rust
pub fn apply(&mut self, output: PresentationOutput) {
    output.present(&mut self.context);
}
```

Os loops ficam nos presenters especializados:

```text
godot_bridge/presenters/
├── lifecycle.rs   # SpawnCommands e DespawnCommands
└── spatial.rs     # SpatialPatches
```

Adicionar animações exigiria:

1. criar `AnimationCommands` (normalmente contendo um enum `AnimationCommand`);
2. implementar `Present<GodotPresentationContext>` para esse tipo;
3. adicionar um campo com `#[present(order = 30)]`.

O `GodotBridge::apply` não muda.

## ADTs: soma dentro de produto

`PresentationOutput` é um ADT de produto porque, no mesmo tick, podem existir
simultaneamente spawns, patches espaciais e despawns.

Dentro de um domínio com operações alternativas, use um ADT de soma. Exemplo
para uma futura camada de animação:

```rust
pub enum AnimationCommand {
    Play { entity: Entity, animation: AnimationId },
    Stop { entity: Entity },
    SetSpeed { entity: Entity, speed: f32 },
}

pub struct AnimationCommands(Vec<AnimationCommand>);
```

Transform é diferente: ele representa estado final, então `SpatialPatches` usa
`HashMap<Entity, SimTransform2D>` e a última escrita do tick vence.

## Fluxo do tick

```text
Godot Input
  -> PlayerInput / DeltaTime
  -> Simulation Systems
  -> Extraction Systems
  -> PresentationOutput
  -> derive PresentOutput (ordena campos)
  -> Spawn presenter
  -> Spatial presenter
  -> Despawn presenter
  -> Nodes Godot
```

## Executar

Linux/macOS:

```bash
./build.sh
```

Windows PowerShell:

```powershell
./build.ps1
```

Ou:

```bash
cd rust
cargo build
cargo test --workspace
```

Depois abra `godot/` no Godot 4.6+ e execute.

## Controles

- WASD: mover o player;
- Espaço: criar inimigo;
- Delete: remover inimigos.

## Observação de validação

O teste em `rust/presentation_derive/tests/derive.rs` declara os campos fora de
ordem física (`90`, `10`, `40`) e verifica que o derive os executa como
`10 -> 40 -> 90`.

<img width="1536" height="1024" alt="image" src="https://github.com/user-attachments/assets/94b50b31-8d4a-47ff-a658-74f7ba8fc72c" />

