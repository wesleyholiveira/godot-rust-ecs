# Godot + godot-rust + Bevy ECS — output agregado por Entity

Projeto mínimo funcional com:

- `godot-rust 0.5.4`;
- `bevy_ecs 0.19.0`;
- modelo ECS agnóstico ao Godot;
- systems de simulação e extração separados;
- um único `EntityHashMap<EntityPatch>` para o estado final das entidades;
- buffers de spawn/despawn drenados e reutilizados;
- extractors globais explicitamente seriais com `.chain()`;
- `GodotBridge` genérico sobre `P: Present<GodotPresentationContext>`;
- derive procedural local `PresentOutput`;
- atributos `#[present(order = N)]` para ordenar os domínios em compilação.

## Estrutura

```text
godot_bevy_ecs_entity_patch_demo/
├── rust/
│   ├── Cargo.toml
│   ├── presentation_derive/
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

## Refactor principal: um guarda-chuva por Entity

O estado final de apresentação não usa mais um mapa separado por domínio,
como `transforms: HashMap<Entity, ...>` e `health: HashMap<Entity, ...>`.
Agora há um único mapa especializado para chaves `Entity`:

```rust
pub struct EntityPatches {
    patches: EntityHashMap<EntityPatch>,
}

pub struct EntityPatch {
    pub spatial: SpatialPatch,
}

pub struct SpatialPatch {
    pub transform: Option<SimTransform2D>,
}
```

Conceitualmente:

```text
PresentationOutput.entities
├── Entity A
│   └── spatial.transform = posição final
└── Entity B
    └── spatial.transform = posição final
```

Ao adicionar futuros estados como UI, animação persistente ou propriedades
visuais, eles podem entrar em `EntityPatch`, sem criar um novo mapa global para
cada tipo de patch.

## Commands versus patches

O output mantém duas categorias:

```text
SpawnCommands / DespawnCommands
→ operações ordenadas
→ armazenadas em Vec

EntityPatches
→ estado final declarativo por Entity
→ armazenado em um único EntityHashMap
→ última escrita vence
```

O transform pode ser escrito várias vezes no mesmo tick, mas somente seu valor
final fica armazenado:

```rust
output.set_transform(entity, first);
output.set_transform(entity, second);
output.set_transform(entity, final_value);
```

O mapa termina com apenas `final_value` para essa entidade.

## `Present` agora usa `&mut self`

```rust
pub trait Present<C> {
    fn present(&mut self, context: &mut C);
}
```

Essa alteração evita mover e destruir o `PresentationOutput` todo tick. Cada
domínio drena seu buffer:

```rust
for request in self.requests.drain(..) {
    // aplica spawn
}

for (entity, patch) in self.patches.drain() {
    // aplica patch
}
```

`drain()` esvazia a coleção, mas conserva a capacidade alocada. Isso reduz
realocações em ticks futuros.

O runtime agora apresenta diretamente o resource persistente:

```rust
let mut output = world.resource_mut::<PresentationOutput>();
bridge.apply(&mut *output);
```

Não há mais `std::mem::take()` no fluxo normal.

## `GodotBridge` genérico

```rust
pub fn apply<P>(&mut self, output: &mut P)
where
    P: Present<GodotPresentationContext> + ?Sized,
{
    output.present(&mut self.context);
}
```

O bridge não conhece `spawns`, `entities` ou `despawns`. Ele apenas inicia a
apresentação de qualquer tipo que implemente o contrato apropriado.

## Derive procedural e `#[present(order = N)]`

```rust
#[derive(Resource, Default, PresentOutput)]
struct PresentationOutput {
    #[present(order = 10)]
    spawns: SpawnCommands,

    #[present(order = 20)]
    entities: EntityPatches,

    #[present(order = 90)]
    despawns: DespawnCommands,
}
```

`order` é prioridade crescente de aplicação, não atraso nem número de frame.
O macro gera conceitualmente:

```rust
impl<C> Present<C> for PresentationOutput
where
    SpawnCommands: Present<C>,
    EntityPatches: Present<C>,
    DespawnCommands: Present<C>,
{
    fn present(&mut self, context: &mut C) {
        self.spawns.present(context);   // 10
        self.entities.present(context); // 20
        self.despawns.present(context); // 90
    }
}
```

A implementação gerada usa referências mutáveis, portanto os campos continuam
existindo e reutilizam suas alocações após serem drenados.

## Extractors globais explicitamente seriais

Todos os extractors escrevem no mesmo `PresentationOutput`. Em vez de deixá-los
numa tupla que poderia sugerir paralelismo, o schedule declara a ordem:

```rust
schedule.add_systems(
    (
        extract_added_views,
        extract_changed_transforms,
        extract_despawn_requests,
    )
        .chain()
        .in_set(GameSet::PresentationExtraction),
);
```

O fluxo fica explícito:

```text
extract_added_views
→ extract_changed_transforms
→ extract_despawn_requests
```

O executor global não é mais forçado a `SingleThreaded`. Os systems de
simulação continuam livres para serem organizados pelo Bevy conforme seus
acessos; somente a fase de extração global é deliberadamente encadeada.

## Ciclo completo do tick

```text
Godot captura input
→ EcsRuntime atualiza PlayerInput e DeltaTime
→ Schedule executa Simulation
→ Commands estruturais são aplicados
→ Extraction serial preenche PresentationOutput
→ Cleanup remove Entities marcadas
→ GodotBridge apresenta o mesmo output por &mut
→ SpawnCommands.drain()
→ EntityPatches.drain()
→ DespawnCommands.drain()
→ clear_trackers()
```

## Memória

- `PresentationOutput` não cresce por acumulação de ticks: seus buffers são
  drenados após cada apresentação.
- As coleções conservam a capacidade do maior pico recente para reutilização;
  isso é retenção de capacidade, não vazamento.
- `ViewRegistry` contém apenas views ativas e remove a entrada no despawn.
- Tanto os patches quanto o registry usam `EntityHashMap`, otimizado pelo Bevy
  para chaves `Entity`.

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

## Validação incluída

O teste em `rust/presentation_derive/tests/derive.rs` verifica:

- execução na ordem dos atributos (`10 -> 40 -> 90`);
- reutilização do mesmo output em duas chamadas consecutivas de `present`.

<img width="1536" height="1024" alt="image" src="https://github.com/user-attachments/assets/94b50b31-8d4a-47ff-a658-74f7ba8fc72c" />

