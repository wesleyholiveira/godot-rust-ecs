# Godot + godot-rust + Bevy ECS — proxy de apresentação

Exemplo mínimo funcional com a estrutura recomendada:

```text
godot_bevy_ecs_proxy_demo/
├── rust/   # cdylib Rust: modelo ECS, proxy e bridge
└── godot/  # projeto Godot, cenas e assets
```

## Requisitos

- Rust 1.95 ou mais recente.
- Godot 4.6 ou mais recente.

O projeto fixa `godot = 0.5.4` e `bevy_ecs = 0.19.0`.

## Executar

Linux/macOS:

```bash
./build.sh
```

Windows PowerShell:

```powershell
./build.ps1
```

Alternativamente:

```bash
cd rust
cargo build
```

Depois abra a pasta `godot/` no Godot e execute a cena principal.

## Controles

- **WASD**: mover o player.
- **Espaço**: criar um inimigo.
- **Delete**: remover todos os inimigos.

## Fluxo da arquitetura

```text
Godot Input
    ↓
PlayerInput / DeltaTime (Resources)
    ↓
Simulation Systems
    ↓
Presentation Extraction Systems
    ↓
PresentationCommands (proxy deferred)
    ↓
Vec<ViewCommand>
    ↓
GodotBridge (adapter)
    ↓
Node2D / PackedScene / queue_free
```

## O que mudou nesta refatoração

A versão anterior expunha os vetores de um `PresentationFrame` aos systems:

```rust
frame.spawns.push(...);
frame.transforms.push(...);
frame.despawns.push(...);
```

Agora existe uma fachada padronizada:

```rust
presentation.spawn_view(entity, kind, transform);
presentation.set_transform(entity, transform);
presentation.despawn_view(entity);
```

Os systems não conhecem os vetores internos nem o `GodotBridge`. O proxy converte as chamadas em um contrato único:

```rust
enum ViewCommand {
    Spawn(SpawnView),
    SetTransform(TransformUpdate),
    Despawn(Entity),
}
```

O proxy continua **deferred**: ele apenas acumula comandos. A API do Godot só é chamada depois que o `Schedule` termina.

## Responsabilidades

### `World`

Contém Entities, Components e Resources.

### Components do modelo

- `SimTransform2D`: posição/rotação lógica, fonte da verdade.
- `MoveSpeed`: velocidade por entidade.
- `Player` e `Enemy`: marcadores.
- `DespawnRequested`: marca remoção no fim do tick.
- `ViewSpec`: informa que a entidade precisa de representação visual.

### Resources

- `PlayerInput`: snapshot do input do tick.
- `DeltaTime`: delta do `_physics_process`.
- `EnemySpawnSequence`: estado global do spawner de exemplo.
- `PresentationCommands`: proxy/buffer de saída do tick.

### Systems de simulação

Alteram apenas o modelo ECS. `Commands` enfileira mudanças estruturais no `World`, como spawn, inserção de componente e despawn.

### Systems de extração

Observam `Added<T>` e `Changed<T>` e chamam a API do proxy. Eles não chamam o Godot:

```text
extract_added_views       -> spawn_view(...)
extract_changed_transforms -> set_transform(...)
extract_despawn_requests  -> despawn_view(...)
```

### `PresentationCommands`

É a fachada/proxy entre os extractors e o adapter:

- esconde a estrutura interna da fila;
- fornece métodos uniformes e tipados;
- acumula `ViewCommand`s;
- não conhece `Gd<Node>` nem chama o Godot;
- ao ser drenado, ordena `Spawn -> SetTransform -> Despawn`.

### `ViewCommand`

É a unidade do contrato de apresentação. Cada variante representa uma intenção que o backend pode aplicar.

### `GodotBridge`

É o adapter concreto. Mantém:

```text
HashMap<Entity, Gd<Node2D>>
```

Ele recebe os `ViewCommand`s, instancia cenas, aplica transforms e chama `queue_free()`.

## As duas filas da arquitetura

Há dois mecanismos diferentes:

1. **`Commands` do Bevy**: fila interna para modificar estruturalmente o ECS (`spawn`, `insert`, `remove`, `despawn`).
2. **`PresentationCommands` do projeto**: proxy/buffer para transportar intenções visuais até o `GodotBridge`.

O segundo não chama o bridge imediatamente. Isso preserva:

- desacoplamento do modelo em relação ao Godot;
- execução da API Godot somente fora do `Schedule`;
- ordem determinística do ciclo de vida;
- possibilidade de testar o ECS sem iniciar a engine.

Não são usados `MessageReader`/`MessageWriter` neste exemplo.

## Organização Rust

```text
rust/src/
├── lib.rs
├── extension.rs
├── schedule.rs
├── model/
│   ├── components.rs
│   ├── resources.rs
│   └── systems/
│       ├── movement.rs
│       └── lifecycle.rs
├── presentation/
│   ├── commands.rs       # ViewCommand + PresentationCommands
│   ├── components.rs     # ViewSpec
│   └── extraction.rs     # modelo -> proxy
└── godot_bridge/
    ├── bridge.rs         # ViewCommand -> API Godot
    └── runtime.rs        # orquestra o tick
```

## Fluxos concretos

### Movimento

```text
WASD
-> PlayerInput
-> player_movement_system
-> SimTransform2D alterado
-> extract_changed_transforms
-> PresentationCommands::set_transform
-> ViewCommand::SetTransform
-> GodotBridge
-> Node2D.set_position
```

### Spawn

```text
Commands::spawn
-> Entity + ViewSpec adicionados ao World
-> extract_added_views
-> PresentationCommands::spawn_view
-> ViewCommand::Spawn
-> GodotBridge instancia PackedScene
-> registra Entity -> Node2D
```

### Despawn

```text
DespawnRequested
-> extract_despawn_requests
-> PresentationCommands::despawn_view
-> ViewCommand::Despawn
-> cleanup remove a Entity
-> GodotBridge remove o HashMap e chama queue_free
```

## Nota sobre versões do Godot

O `godot` 0.5 usa a API Godot 4.6 por padrão. Para compilar deliberadamente contra uma versão anterior suportada, ajuste a dependência, por exemplo:

```toml
godot = { version = "0.5.4", features = ["api-4-5"] }
```

E ajuste `compatibility_minimum` no arquivo `.gdextension`.
