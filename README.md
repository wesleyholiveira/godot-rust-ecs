# Godot + godot-rust + Bevy ECS

Exemplo mínimo funcional com a estrutura recomendada:

```text
godot_bevy_ecs_demo/
├── rust/   # cdylib Rust: modelo ECS, extração e bridge
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

## Arquitetura

```text
Godot Input
    ↓
PlayerInput / DeltaTime (Resources)
    ↓
Simulation Systems
    ↓
Presentation Extraction Systems
    ↓
PresentationFrame
    ↓
GodotBridge
    ↓
Node2D / PackedScene / queue_free
```

### `World`

Contém Entities, Components e Resources.

### Components

- `SimTransform2D`: posição/rotação lógica, fonte da verdade.
- `MoveSpeed`: velocidade por entidade.
- `Player` e `Enemy`: marcadores.
- `DespawnRequested`: marca remoção no fim do tick.
- `ViewSpec`: informa que a entidade precisa de representação visual.

### Resources

- `PlayerInput`: snapshot do input do tick.
- `DeltaTime`: delta do `_physics_process`.
- `EnemySpawnSequence`: estado global do spawner de exemplo.
- `PresentationFrame`: buffer de saída daquele tick.

### Systems de simulação

Alteram apenas o modelo ECS. `Commands` enfileira mudanças estruturais no `World`, como spawn, inserção de componente e despawn.

### Systems de extração

Observam `Added<T>` e `Changed<T>` e produzem um único `PresentationFrame` por tick. Eles não chamam a API do Godot.

### `GodotBridge`

É o adapter concreto entre os dois lados. Mantém:

```text
HashMap<Entity, Gd<Node2D>>
```

Ele instancia cenas, aplica transforms e chama `queue_free()`.

### Filas e buffers usados

Há dois mecanismos diferentes:

1. `Commands`: fila interna do Bevy para modificar estruturalmente o ECS depois que o system termina.
2. `PresentationFrame`: buffer criado pelo projeto para transportar os resultados do ECS até a camada Godot.

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
│   ├── components.rs
│   ├── contracts.rs
│   └── extraction.rs
└── godot_bridge/
    ├── bridge.rs
    └── runtime.rs
```

## Nota sobre versões do Godot

O `godot` 0.5 usa a API Godot 4.6 por padrão. Para compilar deliberadamente contra uma versão anterior suportada, ajuste a dependência, por exemplo:

```toml
godot = { version = "0.5.4", features = ["api-4-5"] }
```

E ajuste `compatibility_minimum` no arquivo `.gdextension`.
