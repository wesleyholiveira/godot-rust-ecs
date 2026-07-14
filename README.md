# Godot + godot-rust + bevy_ecs — base mínima e guia de evolução

Este projeto mantém somente o necessário para mover uma única `Sprite2D` com WASD, mas preserva a arquitetura que deve continuar válida quando o jogo crescer.

A intenção não é ser um framework genérico. É uma base pequena, legível e mensurável para evoluir sem misturar gameplay, ECS e `SceneTree`.

```text
Godot Input
  → Resources do ECS
  → systems de simulação
  → extractors seriais
  → PresentationOutput
  → Present<GodotPresentationContext>
  → GodotBridge serial
  → cache da GodotView
  → Sprite2D
```

## Sumário

- [Executar](#executar)
- [Arquitetura atual](#arquitetura-atual)
- [Por onde começar](#por-onde-começar)
- [Mapa dos arquivos](#mapa-dos-arquivos)
- [Responsabilidades e limites](#responsabilidades-e-limites)
- [Fluxo completo de um tick](#fluxo-completo-de-um-tick)
- [Como evoluir o projeto](#como-evoluir-o-projeto)
- [Gerenciamento de memória](#gerenciamento-de-memória)
- [Paralelismo e schedule](#paralelismo-e-schedule)
- [Performance da ponte com o Godot](#performance-da-ponte-com-o-godot)
- [Pooling de Nodes](#pooling-de-nodes)
- [Testes, profiling e qualidade](#testes-profiling-e-qualidade)
- [Checklist de uma nova feature](#checklist-de-uma-nova-feature)
- [Troubleshooting](#troubleshooting)
- [Roadmap recomendado](#roadmap-recomendado)

---

## Executar

### Requisitos

- Godot 4.6 ou compatível com o arquivo `.gdextension`;
- Rust compatível com `edition = "2024"`;
- `cargo` disponível no terminal.

### Compilar

```bash
cd rust
cargo build
```

Abra a pasta `godot/` no editor e execute `main.tscn`.

Controles:

```text
WASD  move o jogador
```

### Verificações recomendadas

```bash
cd rust
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

---

## Arquitetura atual

O ECS é a fonte da verdade da posição lógica. O Godot apresenta essa posição.

```text
┌────────────────────────────────────────────────────────────┐
│ Godot                                                     │
│ Input Map + _physics_process                              │
└───────────────────────────┬────────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────────┐
│ EcsRuntime                                                │
│ escreve PlayerInput e DeltaTime no World                  │
└───────────────────────────┬────────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────────┐
│ Schedule                                                  │
│ Simulation → PresentationExtraction                       │
└───────────────────────────┬────────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────────┐
│ PresentationOutput                                       │
│ SpawnCommands + EntityHashMap<EntityPatch>                │
└───────────────────────────┬────────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────────┐
│ GodotBridge — serial                                     │
│ Present<GodotPresentationContext>                         │
└───────────────────────────┬────────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────────┐
│ GodotView                                                 │
│ cache → set_position somente quando necessário            │
└───────────────────────────┬────────────────────────────────┘
                            │
                            ▼
                         Sprite2D
```

### O que foi mantido

- `World`, `Schedule`, Components e Resources do Bevy ECS;
- movimentação lógica independente do Godot;
- extração explícita do modelo para apresentação;
- `EntityHashMap<EntityPatch>` para consolidar alterações por entidade;
- `Present<Context>` e derive procedural para ordenar os domínios;
- `ViewRegistry` para associar `Entity` a uma view;
- `GodotView` com cache de setters;
- métricas do último tick de apresentação;
- `SceneFactory` com `PackedScene` pré-carregada;
- bridge serial e curto.

### O que não existe no exemplo mínimo

- despawn de entidades;
- pool de Nodes;
- múltiplos tipos de view;
- física e colisão;
- animação, UI ou áudio;
- troca de mapa e teardown completo.

Esses itens não foram mantidos como código decorativo. As receitas abaixo mostram exatamente como adicioná-los quando houver um caminho real de execução.

---

## Por onde começar

Leia nesta ordem:

1. `rust/src/godot_bridge/runtime.rs`  
   Mostra o ciclo inteiro de um `_physics_process`.

2. `rust/src/schedule.rs`  
   Mostra as fases e a serialização explícita dos extractors.

3. `rust/src/model/components.rs`  
   Dados pertencentes às entidades.

4. `rust/src/model/resources.rs`  
   Dados globais do `World` atualizados pelo Godot.

5. `rust/src/model/systems/movement.rs`  
   Regra de gameplay sem dependência do Godot.

6. `rust/src/presentation/extraction.rs`  
   Detecta mudanças no modelo e escreve no output.

7. `rust/src/presentation/output.rs`  
   Define commands, patches e APIs de escrita.

8. `rust/src/godot_bridge/presenters/`  
   Converte o output em operações concretas no Godot.

9. `rust/src/godot_bridge/godot_view.rs`  
   Faz cache dos valores já aplicados.

10. `rust/presentation_derive/src/lib.rs`  
    Gera a composição ordenada de `Present<C>`.

A primeira extensão real do projeto deve começar no **modelo**, não no bridge.

---

## Mapa dos arquivos

```text
<projeto>/
├── README.md
├── rust/
│   ├── Cargo.toml
│   ├── presentation_derive/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── src/
│       ├── lib.rs
│       ├── extension.rs
│       ├── schedule.rs
│       ├── model/
│       │   ├── components.rs
│       │   ├── resources.rs
│       │   └── systems/
│       │       ├── mod.rs
│       │       └── movement.rs
│       ├── presentation/
│       │   ├── mod.rs
│       │   ├── components.rs
│       │   ├── extraction.rs
│       │   ├── output.rs
│       │   └── present.rs
│       └── godot_bridge/
│           ├── mod.rs
│           ├── runtime.rs
│           ├── bridge.rs
│           ├── context.rs
│           ├── godot_view.rs
│           ├── view_registry.rs
│           ├── scene_factory.rs
│           ├── stats.rs
│           └── presenters/
│               ├── mod.rs
│               ├── lifecycle.rs
│               └── entities.rs
└── godot/
    ├── project.godot
    ├── main.tscn
    ├── godot_bevy_ecs_demo.gdextension
    ├── assets/player.svg
    └── views/player.tscn
```

---

## Responsabilidades e limites

| Arquivo ou módulo | Deve fazer | Não deve fazer |
|---|---|---|
| `model/components.rs` | armazenar estado por Entity | importar `Gd<T>`, `Node`, `PackedScene` |
| `model/resources.rs` | armazenar estado global do `World` | guardar referências para Nodes |
| `model/systems/` | executar regras de gameplay | chamar API do Godot |
| `schedule.rs` | organizar fases e dependências | conter regra de gameplay |
| `presentation/extraction.rs` | traduzir mudanças do modelo | modificar `SceneTree` |
| `presentation/output.rs` | definir patches e commands | carregar cenas ou criar Nodes |
| `godot_bridge/presenters/` | aplicar apresentação no Godot | decidir dano, IA ou movimentação |
| `godot_view.rs` | cachear e aplicar setters | guardar estado lógico autoritativo |
| `view_registry.rs` | mapear Entity → view ativa | criar regras de lifecycle |
| `scene_factory.rs` | carregar e instanciar cenas | conhecer Components de gameplay |
| `runtime.rs` | orquestrar Godot ↔ ECS | concentrar todas as regras do jogo |
| `presentation_derive` | gerar wiring de `Present<C>` | conhecer Godot ou gameplay |

### Invariantes que devem continuar verdadeiras

1. O ECS é a fonte da verdade do gameplay.
2. O modelo não depende da API do Godot.
3. Apenas `godot_bridge` modifica a `SceneTree`.
4. Spawn é aplicado antes dos patches.
5. Despawn, quando existir, é aplicado por último.
6. Toda view ativa pertence ao `ViewRegistry`.
7. Todo buffer temporário é drenado depois da aplicação.
8. Extractors que usam o mesmo `ResMut<PresentationOutput>` são seriais.
9. O bridge aplica resultados prontos; não executa cálculo pesado.
10. Otimização deve ser validada com métricas, não por suposição.

---

## Fluxo completo de um tick

### 1. Godot captura input

`runtime.rs` chama:

```rust
Input::singleton().get_vector(
    "move_left",
    "move_right",
    "move_up",
    "move_down",
);
```

O resultado é copiado para:

```rust
PlayerInput
DeltaTime
```

### 2. O schedule executa a simulação

```rust
player_movement_system
```

O system altera somente:

```rust
SimPosition2D
```

### 3. A extração detecta mudanças

```rust
Changed<SimPosition2D>
```

O extractor grava a posição final no mapa:

```rust
EntityHashMap<EntityPatch>
```

Uma entidade recebe no máximo um `EntityPatch` consolidado por tick.

### 4. O bridge aplica o output

O derive gera a ordem:

```text
10  SpawnCommands
20  EntityPatches
```

O bridge mantém um único `&mut GodotPresentationContext`, portanto a aplicação é serial.

### 5. A view evita setter redundante

`GodotView::apply_position` compara a nova posição com `applied_position`.

```text
igual      → não chama o Godot
 diferente → chama set_position e atualiza o cache
```

### 6. Os buffers são drenados

`Vec::drain(..)` e `EntityHashMap::drain()` deixam os containers vazios, preservando a capacidade para o próximo tick.

### 7. Change detection é encerrada

```rust
world.clear_trackers();
```

Isso encerra o ciclo manual de change detection usado com `bevy_ecs` diretamente.

---

# Como evoluir o projeto

## Receita 1 — adicionar um componente puramente lógico

Exemplo: vida.

### Arquivos

```text
alterar  rust/src/model/components.rs
criar    rust/src/model/systems/health.rs
alterar  rust/src/model/systems/mod.rs
alterar  rust/src/schedule.rs
```

### Componente

```rust
#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct Health {
    pub(crate) current: i32,
    pub(crate) maximum: i32,
}
```

### System

```rust
use bevy_ecs::prelude::*;
use crate::model::components::Health;

pub(crate) fn regeneration_system(
    mut query: Query<&mut Health>,
) {
    for mut health in &mut query {
        health.current = (health.current + 1).min(health.maximum);
    }
}
```

### Registro

Em `model/systems/mod.rs`:

```rust
mod health;
pub(crate) use health::regeneration_system;
```

Em `schedule.rs`:

```rust
schedule.add_systems(
    regeneration_system.in_set(GameSet::Simulation),
);
```

Se a vida não tiver representação visual, pare aqui. Não crie extractor ou presenter sem necessidade.

---

## Receita 2 — adicionar uma propriedade persistente à view

Exemplo: rotação.

Use patch quando a semântica for:

> deixe a view neste estado final.

### Arquivos

```text
alterar  rust/src/model/components.rs
alterar  rust/src/presentation/output.rs
alterar  rust/src/presentation/extraction.rs
alterar  rust/src/presentation/mod.rs
alterar  rust/src/schedule.rs
alterar  rust/src/godot_bridge/godot_view.rs
alterar  rust/src/godot_bridge/presenters/entities.rs
alterar  rust/src/godot_bridge/stats.rs
```

### 1. Adicione o componente

```rust
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub(crate) struct SimRotation2D(pub(crate) f32);
```

### 2. Amplie `EntityPatch`

```rust
#[derive(Default, Debug)]
pub(crate) struct EntityPatch {
    pub(crate) position: Option<SimPosition2D>,
    pub(crate) rotation: Option<SimRotation2D>,
}
```

Adicione uma escrita consolidada:

```rust
fn set_rotation(
    &mut self,
    entity: Entity,
    rotation: SimRotation2D,
) {
    self.patches
        .entry(entity)
        .or_default()
        .rotation = Some(rotation);
}
```

Exponha pela fachada:

```rust
pub(crate) fn set_rotation(
    &mut self,
    entity: Entity,
    rotation: SimRotation2D,
) {
    self.entities.set_rotation(entity, rotation);
}
```

### 3. Crie o extractor

```rust
pub(crate) fn extract_changed_rotations(
    query: Query<
        (Entity, &SimRotation2D),
        (With<ViewSpec>, Changed<SimRotation2D>),
    >,
    mut output: ResMut<PresentationOutput>,
) {
    for (entity, rotation) in &query {
        output.set_rotation(entity, *rotation);
    }
}
```

### 4. Registre o extractor serialmente

```rust
(
    extract_added_views,
    extract_changed_positions,
    extract_changed_rotations,
)
    .chain()
    .in_set(GameSet::PresentationExtraction)
```

### 5. Adicione cache e setter

Em `GodotView`:

```rust
applied_rotation: Option<SimRotation2D>
```

Crie `apply_rotation` com a mesma regra de `apply_position`: só chamar o Godot quando o valor realmente mudar.

### 6. Amplie o presenter e as métricas

O presenter deve aplicar todos os campos de um `EntityPatch` depois de um único lookup no registry.

```rust
if let Some(position) = patch.position {
    // apply_position
}

if let Some(rotation) = patch.rotation {
    // apply_rotation
}
```

Não crie um `HashMap` separado para cada propriedade sem uma necessidade medida.

---

## Receita 3 — adicionar uma ação de input

Exemplo: ataque.

### Arquivos

```text
alterar  godot/project.godot ou Input Map do editor
alterar  rust/src/model/resources.rs
alterar  rust/src/godot_bridge/runtime.rs
criar    rust/src/model/systems/combat.rs
alterar  rust/src/model/systems/mod.rs
alterar  rust/src/schedule.rs
```

### Resource

```rust
#[derive(Resource, Default, Debug)]
pub(crate) struct PlayerInput {
    pub(crate) direction_x: f32,
    pub(crate) direction_y: f32,
    pub(crate) attack_just_pressed: bool,
}
```

### Captura no runtime

```rust
attack_just_pressed: Input::singleton()
    .is_action_just_pressed("attack"),
```

### Consumo

O system lê `Res<PlayerInput>`. Ele não chama `Input::singleton()`.

Isso mantém input, replay, IA e testes desacoplados da engine.

---

## Receita 4 — adicionar um command pontual

Exemplo: tocar som ou emitir partícula.

Use command quando:

- cada ocorrência importa;
- a ordem importa;
- a operação não representa apenas estado final.

### Arquivos

```text
criar    rust/src/presentation/audio.rs
alterar  rust/src/presentation/mod.rs
alterar  rust/src/presentation/output.rs
criar    rust/src/godot_bridge/presenters/audio.rs
alterar  rust/src/godot_bridge/presenters/mod.rs
```

### Domínio

```rust
#[derive(Clone, Copy, Debug)]
pub(crate) enum AudioCommand {
    Play {
        sound: SoundId,
        entity: Option<Entity>,
    },
}

#[derive(Default, Debug)]
pub(crate) struct AudioCommands {
    commands: Vec<AudioCommand>,
}
```

Forneça somente APIs controladas:

```rust
fn push(&mut self, command: AudioCommand)
pub(crate) fn drain(&mut self) -> impl Iterator<Item = AudioCommand> + '_
```

### Adição ao output

```rust
#[present(order = 40)]
audio: AudioCommands,
```

O derive passará a chamar `audio.present(context)` automaticamente.

O derive **não** deve gerar métodos como `play_sound`; essa API pertence ao `impl PresentationOutput`.

---

## Receita 5 — adicionar outro tipo de view

Exemplo: projétil.

O marcador atual:

```rust
struct ViewSpec;
```

precisa evoluir para:

```rust
#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct ViewSpec {
    pub(crate) kind: ViewKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ViewKind {
    Player,
    Projectile,
}
```

### Arquivos

```text
criar    godot/views/projectile.tscn
alterar  rust/src/presentation/components.rs
alterar  rust/src/presentation/output.rs
alterar  rust/src/presentation/extraction.rs
alterar  rust/src/godot_bridge/scene_factory.rs
alterar  rust/src/godot_bridge/presenters/lifecycle.rs
alterar  system que cria a Entity
```

O output de spawn deve carregar o `ViewKind`:

```rust
pub(crate) struct SpawnView {
    pub(crate) entity: Entity,
    pub(crate) kind: ViewKind,
}
```

`SceneFactory` faz o mapeamento concreto:

```text
ViewKind::Player     → res://views/player.tscn
ViewKind::Projectile → res://views/projectile.tscn
```

Não coloque caminhos `res://` em Components. O ECS conhece o tipo lógico; o bridge conhece a cena concreta.

---

## Receita 6 — adicionar despawn corretamente

Despawn exige duas ações diferentes:

1. remover a view no Godot;
2. remover a Entity no ECS.

A extração precisa enxergar a intenção antes da Entity desaparecer.

### Arquivos

```text
alterar  rust/src/presentation/components.rs
alterar  rust/src/presentation/output.rs
alterar  rust/src/presentation/extraction.rs
alterar  rust/src/presentation/mod.rs
alterar  rust/src/godot_bridge/view_registry.rs
alterar  rust/src/godot_bridge/presenters/lifecycle.rs
alterar  rust/src/schedule.rs
criar    rust/src/model/systems/cleanup.rs
alterar  rust/src/model/systems/mod.rs
```

### Marque antes de remover

```rust
#[derive(Component, Debug)]
pub(crate) struct DespawnRequested;
```

### Extraia o command

```rust
pub(crate) fn extract_despawn_requests(
    query: Query<Entity, Added<DespawnRequested>>,
    mut output: ResMut<PresentationOutput>,
) {
    for entity in &query {
        output.despawn_view(entity);
    }
}
```

### Adicione o domínio final

```rust
#[present(order = 90)]
despawns: DespawnCommands,
```

A ordem deve permanecer:

```text
spawn → patches/events → despawn
```

### Limpe a Entity depois da extração

Crie um `GameSet::Cleanup` depois de `PresentationExtraction`.

O cleanup usa `Commands` para despawn estrutural. Garanta que as operações deferred sejam aplicadas na ordem esperada pelo schedule.

### Remova do registry

Adicione:

```rust
pub(crate) fn remove(
    &mut self,
    entity: Entity,
) -> Option<GodotView>
```

A view removida deve:

- ir para um pool limitado; ou
- receber `queue_free()`.

Nunca deixe `Gd<Node2D>` de uma Entity morta no registry.

---

## Receita 7 — adicionar estado global de apresentação

Câmera, HUD global e iluminação não precisam ser forçados dentro de uma Entity artificial.

```rust
#[derive(Default, Debug)]
pub(crate) struct GlobalPatch {
    pub(crate) camera_position: Option<SimPosition2D>,
    pub(crate) hud_visible: Option<bool>,
}
```

Adicione um domínio separado:

```rust
#[present(order = 30)]
global: GlobalPatch,
```

Implemente:

```rust
impl Present<GodotPresentationContext> for GlobalPatch
```

Use o mapa por `Entity` somente para estado realmente pertencente a entidades.

---

## Receita 8 — dividir um módulo que ficou grande

Não divida cedo demais. Quando `output.rs` ou `extraction.rs` ficar difícil de navegar, uma estrutura segura é:

```text
presentation/
├── output/
│   ├── mod.rs
│   ├── lifecycle.rs
│   ├── entity_patch.rs
│   ├── audio.rs
│   └── global.rs
└── extraction/
    ├── mod.rs
    ├── lifecycle.rs
    ├── spatial.rs
    └── ui.rs
```

Mantenha uma fachada pequena em `mod.rs` e evite reexports públicos que não são usados.

---

# Gerenciamento de memória

## 1. Reutilize os buffers

O `PresentationOutput` vive durante todo o runtime.

```rust
Vec::drain(..)
EntityHashMap::drain()
```

Esvaziam os dados sem descartar a capacidade alocada.

Evite este padrão a cada tick:

```rust
let output = std::mem::take(&mut resource);
```

Ele pode mover a capacidade para um valor temporário e provocar realocação no tick seguinte.

## 2. Entenda o high-water mark

`drain()` e `clear()` preservam capacidade. Um pico de dez mil patches pode deixar o mapa com capacidade alta mesmo depois do pico.

Isso não é vazamento.

Reduza a capacidade somente em transições raras:

```text
fim da partida
troca de mapa
retorno ao menu
```

Nesses pontos, pode ser válido usar `shrink_to_fit()` ou reconstruir o resource.

Não faça isso por frame.

## 3. Mantenha o registry coerente

Ciclo correto:

```text
spawn    → insert
patch    → get_mut
despawn  → remove
```

O tamanho do registry deve acompanhar o número de views ativas.

Sinais de bug:

- registry cresce após entidades morrerem;
- `missing_views` aumenta continuamente;
- Nodes ficam invisíveis, mas continuam vivos sem propósito;
- uma Entity nova recebe a view de outra Entity.

## 4. Use `Entity` completo como chave

Não use somente o índice numérico da Entity. A geração faz parte da identidade e impede que um slot reutilizado seja confundido com uma Entity antiga.

O projeto usa:

```rust
EntityHashMap<T>
```

para manter a chave correta e um hasher apropriado para `Entity`.

## 5. Evite dados pesados nos patches

`EntityPatch` deve carregar estado pequeno e final:

```text
posição
rotação
escala
visibilidade
IDs pequenos
```

Evite:

```text
Strings grandes
imagens
meshes
buffers extensos
PackedScene
Gd<T>
```

Use IDs ou handles para recursos cacheados no bridge.

## 6. Limite commands potencialmente explosivos

Áudio, partículas e logs podem crescer sem limite se um bug produzir milhares de eventos por tick.

Para cada fila de commands, defina:

- limite por tick;
- política de descarte;
- coalescência, quando possível;
- métrica de itens descartados.

Exemplo:

```text
máximo de 256 sons por tick
sons iguais e simultâneos podem ser agrupados
excedentes incrementam dropped_audio_commands
```

## 7. Faça teardown explícito

Quando houver troca de cena ou mapa, crie uma rotina que:

1. interrompa a produção de novos outputs;
2. drene o `PresentationOutput`;
3. remova todas as views do registry;
4. esvazie ou libere o pool;
5. descarte caches de cenas específicos da fase;
6. remova o root `EcsViews`;
7. limpe resources temporários do `World`.

Não dependa somente de drop implícito em uma arquitetura com Nodes e referências da engine.

---

# Paralelismo e schedule

## O que o Bevy pode paralelizar

Systems de simulação podem executar em paralelo quando seus acessos não conflitam.

Exemplo potencialmente paralelo:

```text
movement_system  escreve SimPosition2D
health_system    escreve Health
```

Exemplo necessariamente serial:

```text
system A escreve SimPosition2D
system B escreve SimPosition2D
```

O scheduler decide a compatibilidade a partir de `Query`, `Res` e `ResMut`.

## Não use `.chain()` sem dependência real

`.chain()` torna os systems ordenados e seriais.

Use na simulação somente quando a saída de um system precisa estar disponível para o próximo naquele mesmo tick.

Não encadeie tudo por conveniência. Isso transforma o schedule em uma sequência manual e elimina o principal benefício do ECS scheduler.

## Por que os extractors atuais são seriais

Os dois recebem:

```rust
ResMut<PresentationOutput>
```

Portanto, já possuem conflito de escrita.

O projeto também usa `.chain()` para tornar essa serialização visível:

```rust
(
    extract_added_views,
    extract_changed_positions,
)
    .chain()
    .in_set(GameSet::PresentationExtraction)
```

Isso evita a falsa impressão de que os extractors globais serão paralelos.

## O custo de um output global

Um único `PresentationOutput` oferece:

- agregação simples;
- uma API única;
- um mapa por entidade;
- ordem clara de aplicação.

A troca é:

- todos os extractors que usam `ResMut<PresentationOutput>` são incompatíveis entre si.

Para a maioria dos projetos, uma fase curta de extração serial é aceitável.

## Quando a extração serial ficar cara

Evolua em etapas:

### Etapa 1 — mantenha simples

```text
simulation paralela
extraction serial curta
bridge serial curto
```

### Etapa 2 — prepare dados em paralelo

Systems paralelos escrevem em Components ou Resources intermediários separados.

Depois, um único collector serial consolida no `PresentationOutput`.

### Etapa 3 — outputs por domínio

Somente se o profiler justificar, use:

```text
SpatialOutput
UiOutput
AudioOutput
```

Depois faça um merge ou aplicação ordenada.

Isso aumenta complexidade e quantidade de buffers; não é uma melhoria gratuita.

## `Commands` e operações deferred

`bevy_ecs::Commands` é usado para alterações estruturais:

```text
spawn de Entity
insert/remove de Component
despawn
```

Ao adicionar lifecycle dinâmico, garanta que as operações deferred sejam aplicadas antes da fase que depende delas.

Teste explicitamente:

```text
system cria Entity com ViewSpec
→ extração enxerga Added<ViewSpec>
→ SpawnCommands recebe a Entity
```

## Por que o bridge continua serial

O bridge:

- possui um único `&mut GodotPresentationContext`;
- modifica a `SceneTree` ativa;
- precisa respeitar `spawn → patches → despawn`;
- faz lookups e setters em views compartilhadas.

Trabalho pesado deve ocorrer antes dele.

Não use locks ao redor de Nodes para “paralelizar” uma etapa naturalmente serial. Reduza a quantidade de operações aplicadas.

---

# Performance da ponte com o Godot

## Barreiras contra trabalho redundante

O projeto usa duas barreiras:

### 1. Change detection

```rust
Changed<SimPosition2D>
```

Evita gerar patch quando o componente não foi marcado como alterado.

### 2. Cache da view

```rust
GodotView::apply_position
```

Evita chamar `set_position` quando o valor final já está aplicado.

Isso é importante porque `Changed<T>` indica acesso ou mudança observada pelo ECS, não garante por si só que a chamada concreta ao Godot seja necessária.

## Um lookup por entidade alterada

`EntityPatches` agrega propriedades por Entity.

O presenter deve fazer:

```text
1 lookup no ViewRegistry
→ aplica todos os campos presentes no EntityPatch
```

Evite um lookup separado para posição, rotação, visibilidade e animação da mesma entidade.

## Não chame setters sem necessidade

Toda nova propriedade deve possuir cache correspondente quando:

- o setter é frequente;
- o valor pode ser repetido;
- a chamada cruza a fronteira Rust ↔ Godot;
- o custo de comparação é muito menor que o setter.

Para floats, escolha conscientemente entre:

- igualdade exata para valores determinísticos;
- epsilon para valores sujeitos a ruído numérico.

Não use epsilon arbitrário sem relacioná-lo à unidade e escala do jogo.

## Métricas atuais

`PresentationStats` mede o último tick:

```text
spawn_requests
instantiated_views
entity_patches
applied_setters
skipped_setters
missing_views
```

Em build de debug, o runtime imprime uma amostra periódica.

Interpretação:

- `skipped_setters` alto: o cache está evitando trabalho;
- `missing_views` acima de zero: há erro de ordem ou lifecycle;
- `entity_patches` muito maior que `applied_setters`: muitos patches vazios ou repetidos;
- `spawn_requests` contínuo para uma view estática: bug de lifecycle.

Ao crescer, adicione também:

```text
tempo do schedule
tempo da extração
tempo do bridge
views ativas
capacidade dos buffers
views no pool
commands descartados
```

Não registre uma linha por Entity ou por setter em produção. Agregue contadores e publique em intervalos.

## Quando um Node por Entity deixa de escalar

Para milhares de objetos homogêneos, considere:

- `MultiMesh`;
- APIs de servidor de renderização;
- renderização em lote;
- uma view representando várias Entities;
- culling antes da extração;
- atualização visual em frequência menor que a simulação.

O próximo passo raramente é paralelizar milhares de `set_position()`. É reduzir Nodes e chamadas.

---

# Pooling de Nodes

O projeto mínimo não contém um pool porque existe somente um player criado uma vez e nunca removido. Sem despawn e reuso, o pool não teria um caminho de execução válido.

Adicione pooling quando houver entidades com ciclo frequente:

```text
spawn → uso → despawn → reutilização
```

Exemplos:

```text
projéteis
partículas customizadas
inimigos em ondas
itens temporários
```

## Arquivos para adicionar

```text
criar    rust/src/godot_bridge/view_pool.rs
alterar  rust/src/godot_bridge/mod.rs
alterar  rust/src/godot_bridge/context.rs
alterar  rust/src/godot_bridge/godot_view.rs
alterar  rust/src/godot_bridge/presenters/lifecycle.rs
alterar  rust/src/godot_bridge/stats.rs
```

## Estrutura recomendada

```rust
pub(crate) struct ViewPool {
    by_kind: HashMap<ViewKind, Vec<GodotView>>,
    limits: HashMap<ViewKind, usize>,
}
```

O pool precisa conhecer o tipo da view, portanto deve ser introduzido junto de `ViewKind`.

## Fluxo de spawn

```text
1. tenta retirar do pool
2. reseta completamente a view
3. adiciona ao root ativo
4. registra Entity → GodotView
5. se não houver item, instancia pela SceneFactory
```

## Fluxo de despawn

```text
1. remove do ViewRegistry
2. desconecta da apresentação ativa
3. reseta estado transitório
4. insere no pool se abaixo do limite
5. caso contrário, queue_free
```

## Reset obrigatório

Antes de reutilizar uma view, limpe:

```text
posição e transform transitório
visibilidade
animações
modulate
material overrides
sinais ou callbacks específicos
filhos temporários
cache de setters
estado de partículas
```

Pooling sem reset completo produz bugs difíceis de reproduzir.

## Limites

Todo pool deve ter limite por `ViewKind`.

Exemplo inicial:

```text
Player      1
Projectile  256
Enemy       64
```

Esses números não são universais. Ajuste com métricas de pico real.

Acima do limite, libere a view. Um pool ilimitado é apenas retenção de memória com outro nome.

---

# Testes, profiling e qualidade

## Testes do modelo sem Godot

A maior parte do gameplay pode ser testada com `World` e `Schedule` puros.

Exemplo:

```rust
#[test]
fn player_moves_to_the_right() {
    let mut world = World::new();
    world.insert_resource(PlayerInput {
        direction_x: 1.0,
        direction_y: 0.0,
    });
    world.insert_resource(DeltaTime { seconds: 1.0 });

    let entity = world
        .spawn((
            Player,
            SimPosition2D { x: 0.0, y: 0.0 },
            MoveSpeed(10.0),
        ))
        .id();

    let mut schedule = Schedule::default();
    schedule.add_systems(player_movement_system);
    schedule.run(&mut world);

    let position = world.get::<SimPosition2D>(entity).unwrap();
    assert_eq!(*position, SimPosition2D { x: 10.0, y: 0.0 });
}
```

## Testes recomendados

```text
movimento usa delta corretamente
input zero não muda posição
Added<ViewSpec> gera um spawn
última posição do tick vence
output fica vazio após present
cache ignora posição repetida
spawn é aplicado antes do primeiro patch
despawn remove registry antes de liberar Entity
pool respeita limite
```

## Teste do derive procedural

Adicione:

```text
rust/presentation_derive/tests/derive.rs
```

Verifique:

- ordem numérica independente da ordem física dos campos;
- erro para ordem duplicada;
- erro para campo sem `#[present(order = N)]`;
- suporte a `&mut self`;
- reutilização dos mesmos buffers após duas chamadas.

Para erros de compilação esperados, use testes de compilação dedicados, como `trybuild`, quando a crate crescer.

## Profiling

Meça separadamente:

```text
1. captura de input
2. Schedule completo
3. PresentationExtraction
4. GodotBridge
5. quantidade de patches
6. quantidade de setters
7. spawn/despawn por segundo
8. tamanho e capacidade dos buffers
9. views ativas e em pool
```

Compare debug e release. Não tire conclusão de performance apenas em build de debug.

## Critérios de regressão

Antes de aceitar uma mudança de arquitetura, compare o mesmo cenário:

```text
mesmo número de Entities
mesmo tempo de execução
mesmo padrão de input
mesma taxa de spawn/despawn
```

Uma abstração nova deve justificar seu custo em clareza, extensibilidade ou performance mensurada.

---

# Checklist de uma nova feature

## Modelo

- [ ] o dado pertence a Component ou Resource correto;
- [ ] não há tipos do Godot no modelo;
- [ ] o system possui o menor acesso possível;
- [ ] alterações estruturais usam `Commands`;
- [ ] dependências no schedule são explícitas somente quando necessárias.

## Apresentação

- [ ] decidiu entre patch e command;
- [ ] o output possui API de escrita encapsulada;
- [ ] o extractor usa filtros específicos;
- [ ] o extractor foi registrado na chain serial global;
- [ ] o presenter realiza um lookup por Entity;
- [ ] setters frequentes possuem cache;
- [ ] métricas foram atualizadas;
- [ ] a ordem `spawn → patches/events → despawn` foi preservada.

## Memória

- [ ] buffers são drenados;
- [ ] o registry remove views destruídas;
- [ ] o pool, se existir, possui limite;
- [ ] scenes são pré-carregadas ou cacheadas;
- [ ] filas de commands possuem orçamento;
- [ ] teardown foi definido para troca de fase.

## Paralelismo

- [ ] não adicionou `.chain()` sem dependência;
- [ ] não introduziu `ResMut` global em systems que deveriam ser paralelos;
- [ ] trabalho pesado continua fora do bridge;
- [ ] operações deferred foram consideradas.

## Qualidade

- [ ] `cargo fmt` passa;
- [ ] `cargo check` passa;
- [ ] testes passam;
- [ ] `clippy` passa;
- [ ] o cenário foi validado no Godot;
- [ ] métricas não mostram regressão inesperada.

---

# Troubleshooting

## `expected &mut SpawnCommands, found SpawnCommands`

O derive deve gerar:

```rust
Present::present(&mut self.spawns, context);
```

Confira:

```text
rust/presentation_derive/src/lib.rs
```

Depois:

```bash
cargo clean
cargo check --workspace
```

Reinicie o rust-analyzer. Use `cargo expand --lib` para inspecionar a expansão real.

## A classe `EcsRuntime` não aparece no Godot

Verifique:

1. `cargo build` terminou sem erro;
2. o caminho da biblioteca no `.gdextension` corresponde ao sistema operacional;
3. `extension.rs` contém o único `#[gdextension]`;
4. o editor foi reiniciado após falha de carregamento;
5. o console do Godot não mostra símbolo ou biblioteca ausente.

## A Entity existe, mas a Sprite não aparece

Verifique:

1. a Entity possui `ViewSpec`;
2. `extract_added_views` está registrado;
3. `PresentationOutput` está inserido como Resource;
4. `SceneFactory::preload` carregou `player.tscn`;
5. a raiz de `player.tscn` herda `Node2D`;
6. o `GodotPresentationContext` possui root;
7. o `ViewRegistry` recebeu a view.

## `missing_views` aumenta

Possíveis causas:

- patch aplicado antes do spawn;
- Entity recebeu `ViewSpec` depois da posição, mas a ordem foi alterada;
- view removida do registry cedo demais;
- Entity antiga permaneceu em um command;
- lifecycle e cleanup estão fora de ordem.

## Setters continuam altos

Verifique:

- o extractor usa `Changed<T>`;
- o system não solicita `&mut T` sem necessidade;
- o cache da `GodotView` foi atualizado junto com o setter;
- a comparação de floats é adequada;
- mais de uma origem está escrevendo a mesma propriedade;
- o mesmo estado está sendo reinserido todo tick.

## Memória cresce continuamente

Verifique:

- containers são drenados;
- registry remove despawns;
- pools possuem limite;
- commands têm orçamento;
- cenas não são carregadas repetidamente;
- Nodes não são criados fora do bridge;
- teardown ocorre na troca de mapa.

## O movimento funciona em debug, mas a biblioteca release não carrega

Compile explicitamente:

```bash
cargo build --release
```

Confira o caminho `release` no arquivo `.gdextension` e a extensão correta da biblioteca para o sistema operacional.

---

# Roadmap recomendado

Ordem segura para evoluir esta base:

1. adicionar testes puros do movimento;
2. adicionar `Health` e systems de gameplay sem apresentação;
3. adicionar rotação ou visibilidade como novos campos do `EntityPatch`;
4. adicionar um segundo `ViewKind`;
5. implementar `DespawnRequested` e cleanup ordenado;
6. adicionar pool limitado para um tipo realmente dinâmico;
7. adicionar `UiPatch` ou `GlobalPatch`;
8. adicionar commands de áudio/animação;
9. criar teardown de troca de mapa;
10. medir o limite de Node por Entity;
11. introduzir batch rendering somente onde o profiler justificar.

---

## Regra final de design

Ao adicionar qualquer feature, percorra sempre esta direção:

```text
entrada
→ Resource ou Component
→ system de modelo
→ mudança lógica
→ extractor
→ patch ou command
→ presenter
→ cache
→ chamada mínima ao Godot
→ métrica
```

Quando uma implementação pula essas etapas e um system de gameplay começa a chamar Nodes diretamente, a arquitetura deixa de ter uma fonte da verdade clara e fica mais difícil testar, paralelizar e otimizar.