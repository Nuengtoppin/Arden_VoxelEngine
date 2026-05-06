**Document Status:** 🕯 archived  
**Version:** 0.1.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2025-12-04

---

# Arden Engine — MVP0 Technical Specification

MVP0 — это **сырой прототип**, цель которого не «красота», а демонстрация ключевой идеи:

* MVP не оптимизирован.
* Цель — проверить концепт.

> **Один DUN = один объект для физики, внутри которого живёт много вокселей (32×32×32).**  
> Физика и шарик взаимодействуют с **mesh-поверхностью**, а не с кубической границей DUN.

---

## 1. Цель MVP0

Показать на живой сцене, что:

1. Есть **Dynamic DUN** размером `32×32×32` вокселей.
2. Из него генерируется **surface mesh сложной формы** (горка/ступенька).
3. Шар (`RigidBody::Dynamic`) падает сверху и **катается по поверхности меша DUN**,  
   а не упирается в кубический `32×32×32` bounding box.
4. Можно заспавнить **десятки таких DUN** (20–50) и увидеть, что движок живёт (FPS ок).

**Сознательно НЕ делаем в MVP0:**

- Разрушение мира / отщипывание чанков в отдельные DUN.
- Воксельный ландшафт для пола.
- SVO/LOD, HAOS, стриминг регионов и т.п.

---

## 2. Технологический стек

- Язык: **Rust 2021**
- Игровой движок: **Bevy `=0.13.2`**
- UI: `bevy_egui`
- Инспектор: `bevy-inspector-egui`
- Сериализация: `serde`, `ron`
- Логирование: `tracing`, `tracing-subscriber`
- Физика: **Rapier 3D** (через `bevy_rapier3d` для Bevy 0.13.x)

---

## 3. Базовая модель мира для MVP0

### 3.1. Размерности

- **Размер вокселя:** `1.0` world unit.
- **1 Octochunk:**  
  `VoxelGrid.size = (32, 32, 32)`, всего `32³` вокселей. (размерность мира, страйды)
- **Chunk для MVP:** считаем, что `VoxelGrid.size = (64, 64, 64)`, всего `64³` вокселей.  
  `chunk_size_world = 6.0`.
- **DUN = 1 Octochunk:**
   `VoxelGrid.size = (32, 32, 32)`, всего `32³` вокселей. (коробка для наполнения вокселей для мвп используем один страйд окточнка) 
- **Block:** `8×8×8` чанков (логическая рамка, пока без реальной реализации).

### 3.2. Объекты в сцене

1. **Пол (Static Ground)**  
   - Обычный `Mesh` (plane или box), **не DUN**, без вокселей.  
   - `RigidBody::Fixed` + `Collider::cuboid(...)`.

2. **Dynamic DUN (герой MVP0)**  
   - Один `Entity` в ECS:
     - компонент `Dun { kind, chunk_coord, voxel_grid }`,
     - `Transform`,
     - `Mesh` + `Material`,
     - `RigidBody::Dynamic`,
     - `Collider` **по surface mesh (tri-mesh)**.
   - Размер воксельной решётки: `32×32×32`.
   - Внутри произвольная неровная форма (горка, ступеньки, блоки).

3. **Шар (Ball)**  
   - Отдельный `Entity`:
     - `Mesh` (UVSphere),
     - `RigidBody::Dynamic`,
     - `Collider::ball(radius)`.

4. **Topology Debug (Route-lite)**  
   - `chunk_coord = floor(position / 32.0)` (целочисленные координаты чанка).  
   - Debug grid: тонкие линии каждый 1 unit, жирные — каждые 32 (границы чанков).  
   - В инспекторе/egui отображать `chunk_coord` для DUN.

---

## 4. Структура проекта (`src/`)

```text
src/
  main.rs

  app/
    mod.rs
    setup.rs        // камера, свет, пол, запуск плагинов

  mvp0/
    mod.rs
    scene.rs        // сцена MVP0.0: один DUN + шар
    stress_test.rs  // MVP0.1: массовый спавн DUN (краш-тест)

  voxel/
    mod.rs
    grid.rs         // VoxelGrid 32x32x32
    mesher.rs       // кубический mesher

  dun/
    mod.rs
    dun.rs          // компонент Dun
    kind.rs         // DunKind (DynamicVoxel)
    route_lite.rs   // chunk_coord = floor(pos / 32.0)
    spawn.rs        // функции спавна одного/многих DUN

  render/
    mod.rs
    mesh_builder.rs // VoxelGrid -> bevy::Mesh

  physics/
    mod.rs
    collider_builder.rs // Mesh -> Collider::trimesh(...)

  tools/
    mod.rs
    debug_grid.rs   // grid + границы чанков
    egui_panel.rs   // UI: Spawn N, счетчики
```

Часть модулей (select, state, transform, integrator) можно добавить позже. Для MVP0 достаточно перечисленных.

---

## 5. Детальная спецификация по файлам

### 5.1. `src/main.rs`

**Назначение:** точка входа.

**Минимум:**

* Инициализация Bevy `App`.
* Подключение:

  * `DefaultPlugins`,
  * `EguiPlugin`,
  * `RapierPhysicsPlugin`,
  * `RapierDebugRenderPlugin` (опционально),
  * своего `Mvp0Plugins` из `app/plugins.rs`.

---

### 5.2. `app/mod.rs`

**Назначение:** реэкспорт подмодулей.

```rust
pub mod setup;
pub mod plugins;
```

---

### 5.3. `app/setup.rs`

**Назначение:** базовый сетап приложения и сцены.

**Функции:**

* `pub fn setup_camera_and_light(...)`

  * Спавнит камеру (`PerspectiveCameraBundle`) и источник света.

* `pub fn spawn_ground(...)`

  * Спавнит пол:

    * `PbrBundle` с `Mesh::from(shape::Plane { size: 512.0 })` или `Box`.
    * `RigidBody::Fixed`.
    * `Collider::cuboid(...)`.

---

### 5.4. `app/plugins.rs`

**Назначение:** регистрация плагинов.

```rust
pub struct Mvp0Plugins;

impl Plugin for Mvp0Plugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins)
            .add_plugins(EguiPlugin)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(Mvp0ScenePlugin); // из mvp0/scene.rs
    }
}
```

---

### 5.5. `voxel/mod.rs`

```rust
pub mod grid;
pub mod mesher;
```

---

### 5.6. `voxel/grid.rs`

**Назначение:** хранение вокселей для одного DUN (32×32×32).

```rust
pub const VOXELS_PER_AXIS: u32 = 32;

#[derive(Clone)]
pub struct VoxelGrid {
    pub size: UVec3,     // (32,32,32)
    pub data: Vec<u8>,   // len = 32^3, 0 = пусто, 1..N = материал
}

impl VoxelGrid {
    pub fn new(size: UVec3) -> Self;
    pub fn index(&self, x: u32, y: u32, z: u32) -> usize;
    pub fn get(&self, x: u32, y: u32, z: u32) -> u8;
    pub fn set(&mut self, x: u32, y: u32, z: u32, value: u8);
}
```

**На MVP0:** `size` всегда задаётся как `UVec3::splat(VOXELS_PER_AXIS)`.

---

### 5.7. `voxel/mesher.rs`

**Назначение:** генерация **кубического** mesh из `VoxelGrid`.

```rust
pub fn build_mesh(grid: &VoxelGrid) -> Mesh;
```

**Алгоритм (минимальная версия):**

* пройти все `(x, y, z)`,
* если `voxel != 0`:

  * добавить куб (6 граней) в mesh на позиции `(x, y, z)`.
* Можно без greedy-мешинга на MVP0.

---

### 5.8. `render/mod.rs`

```rust
pub mod mesh_builder;
```

---

### 5.9. `render/mesh_builder.rs`

**Назначение:** обвязка вокруг mesher.

```rust
use crate::voxel::grid::VoxelGrid;
use bevy::prelude::*;

pub fn build_bevy_mesh(grid: &VoxelGrid) -> Mesh {
    // внутри делегирует в voxel::mesher::build_mesh,
    // либо реализует mesh-генерацию напрямую.
}
```

---

### 5.10. `dun/mod.rs`

```rust
pub mod dun;
pub mod kind;
pub mod route_lite;
pub mod spawn;
```

---

### 5.11. `dun/kind.rs`

```rust
#[derive(Clone, Copy, Debug)]
pub enum DunKind {
    DynamicVoxel, // MVP0: один тип
}
```

---

### 5.12. `dun/dun.rs`

**Назначение:** ECS-компонент DUN.

```rust
use bevy::prelude::*;
use crate::voxel::grid::VoxelGrid;
use super::kind::DunKind;

#[derive(Component)]
pub struct Dun {
    pub kind: DunKind,
    pub chunk_coord: IVec3,   // floor(position / 32.0)
    pub voxel_grid: VoxelGrid,
}
```

> Mesh, Material, Transform, RigidBody, Collider — храним как отдельные компоненты (`PbrBundle` + Rapier).

---

### 5.13. `dun/route_lite.rs`

**Назначение:** “мини-Route” для привязки к чанку.

```rust
use bevy::prelude::*;

pub const CHUNK_SIZE: f32 = 32.0;

pub fn calc_chunk_coord(pos: Vec3) -> IVec3 {
    (pos / CHUNK_SIZE).floor().as_ivec3()
}
```

---

### 5.14. `dun/spawn.rs`

**Назначение:** функции создания DUN.

Минимум:

```rust
pub fn spawn_test_dun(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity;

pub fn spawn_random_dun(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) -> Entity;
```

**spawn_test_dun:**

* создаёт `VoxelGrid 32³` c формой “горка/ступенька”;
* генерирует `Mesh` через `render::mesh_builder`;
* создаёт `Dun` + `PbrBundle` + `RigidBody::Fixed` (для MVP0.0) + `Collider::trimesh(...)`.

**spawn_random_dun:**

* создаёт `VoxelGrid 32³` с простой формой (куб/столб/Г-блок),
* `RigidBody::Dynamic` (для краш-теста).

---

### 5.15. `physics/mod.rs`

```rust
pub mod collider_builder;
```

---

### 5.16. `physics/collider_builder.rs`

**Назначение:** `Mesh` → `Collider`.

```rust
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn mesh_to_collider(mesh: &Mesh) -> Collider {
    // берём вершины и индексы из mesh
    // строим Collider::trimesh(...)
}
```

На MVP0 достаточно поддержать только triangle-list.

---

### 5.17. `tools/mod.rs`

```rust
pub mod debug_grid;
pub mod egui_panel;
```

---

### 5.18. `tools/debug_grid.rs`

**Назначение:** визуализация топологии.

* Рисуем:

  * тонкие линии с шагом 1,
  * толстые линии каждые 32 (границы Chunk/DUN).

Можно через `Gizmos` Bevy 0.13:

```rust
pub fn draw_debug_grid(mut gizmos: Gizmos) {
    // линии по XZ
}
```

---

### 5.19. `tools/egui_panel.rs`

**Назначение:** простой UI для краш-теста.

Элементы EGUI:

* Кнопки:

  * `Spawn 1 DUN`
  * `Spawn 10 DUN`
  * `Spawn 50 DUN`
* Инфо:

  * `Total DUN: N`
  * `Voxels per DUN: 32^3`
  * `Total voxels: N * 32^3`

---

### 5.20. `mvp0/mod.rs`

```rust
pub mod scene;
pub mod stress_test;
```

---

### 5.21. `mvp0/scene.rs`

**Назначение:** сцена MVP0.0 — один DUN + один шар.

Плагин:

```rust
pub struct Mvp0ScenePlugin;

impl Plugin for Mvp0ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_mvp0_scene);
    }
}
```

`fn setup_mvp0_scene(...)`:

* вызывает:

  * `setup_camera_and_light`,
  * `spawn_ground`,
  * `spawn_test_dun`,
  * `spawn_test_ball`,
  * включает debug grid и egui панель (если готова).

---

### 5.22. `mvp0/stress_test.rs`

**Назначение:** MVP0.1 — массовый спавн DUN.

* Функция/система, которую вызывает `egui_panel`:

  * `spawn_batch(count: u32)` → N DUN’ов на случайных позициях над полом.

---

## 6. Критерии готовности MVP0

### MVP0.0 — один DUN + один шар

* Есть:

  * окно, камера, свет,
  * пол (mesh + Fixed collider),
  * один DUN (32³) с неровной воксельной формой,
  * шар (Dynamic, Collider::ball).
* Mesh DUN генерируется из `VoxelGrid`.
* **Collider для DUN строится по mesh (tri-mesh)**.
* Шар падает сверху и **катается по поверхности DUN**, а не отскакивает от невидимого куба.

### MVP0.1 — краш-тест

* Есть UI-кнопки `Spawn N DUN`.
* Спавнится 20–50 Dynamic DUN’ов над полом.
* Они падают, сталкиваются между собой и с полом.
* В UI видно:

  * количество DUN,
  * общее количество вокселей,
  * хотя бы примерный FPS.
* Debug grid показывает границы чанков (каждые 32 units).

---

#### Режим "постепенный стресс-тест" (continuous spawn)

Помимо разового спавна (кнопки `Spawn 100` и т.п.), рекомендуется ввести
режим "постепенного стресса", где новые объекты появляются по одному с течением времени.

Идея:

- Пользователь сам:
  - выбирает режим `StressMode`:
    - `VoxelsOnly` (каждый кубик — отдельный rigidbody),
    - `DUN` (все кубики внутри одного DUN);
  - включает / выключает "стресс-тест";
  - при необходимости очищает сцену (кнопка `Clear`).

##### Логика режима

Вводится простая структура для состояния стресс-теста:

- `mode: StressMode` — текущий режим (`VoxelsOnly` или `DUN`);
- `active: bool` — включён ли стресс-режим;
- `spawn_rate: f32` — сколько объектов (или DUN) спавнится в секунду (например, 5 шт/с);
- `spawn_accumulator: f32` — накопленный таймер для спавна.

Пока `active == true`, каждый кадр:

- накапливается `delta_time` в `spawn_accumulator`,
- когда `spawn_accumulator >= 1.0 / spawn_rate`:
  - спавнится 1 объект:
    - в режиме `VoxelsOnly` — один новый `VoxelBody` (кубик-entity);
    - в режиме `DUN` — либо:
      - новый DUN (если хотим множества DUN),
      - либо обновление/расширение одного DUN (если хотим один "растущий" DUN);
  - `spawn_accumulator` уменьшается на шаг.

Таким образом можно, например, получать 5 новых объектов в секунду, пока стресс-режим включён.

##### Управление через EGUI

В EGUI-панели добавляются элементы:

- Переключатель режима:

  - `Mode: [ VoxelsOnly | DUN ]`

- Блок стресс-теста:

  - чекбокс / кнопка:

    - `Stress test: [ ] active`

  - слайдер/поле:

    - `Spawn rate: 1..20 (objects/sec)` *(по умолчанию, например, 5)*

  - кнопка:

    - `Clear` — удалить все динамические объекты сцены (DUN / VoxelBody).

##### Логи и вывод на экран

Во время стресс-теста информация фиксируется:

1. **В лог (`tracing`):**

   При каждом спавне можно писать, например:

   - для режима без DUN:

     ```text
     [MVP0][VoxelsOnly] spawned 1 voxel body, total_entities = N, total_bodies = N
     ```

   - для режима с DUN:

     ```text
     [MVP0][DUN] spawned 1 DUN, voxels_per_dun = 32^3, total_duns = M
     ```

2. **В EGUI-панели:**

   Отображаются агрегированные значения:

   - `Mode: VoxelsOnly / DUN`
   - `Stress: ON / OFF`
   - `Spawn rate: X objects/sec`
   - `Entities: N`  *(общее количество entities в стресс-тесте)*
   - `Rigid bodies: M` *(приблизительно равно N для VoxelsOnly, мало для DUN)*
   - `DUN count: D`   *(для режима DUN)*
   - `Approx. voxels: D * 32^3` *(оценка общего числа вокселей при DUN)*

##### Зачем этот режим

Этот режим позволяет:

- видеть **динамический рост нагрузки**, а не только один разовый "залп" из 100 объектов;
- вручную включать / выключать стресс-тест и менять режим `VoxelsOnly` ↔ `DUN` в одной сессии;
- визуально и по цифрам сравнивать:
  - "вот так выглядят сотни отдельных физических кубиков",
  - "а вот так — те же воксели, собранные в DUN-структуру".

Это дополнение не меняет фундаментальных целей MVP0,
но делает демо более наглядной для сравнения подходов "без DUN" и "с DUN".


Этот документ можно давать другому разработчику как ТЗ:
структура папок, формат компонентов и ожидаемое поведение сцены прописаны достаточно жёстко, а детали реализации (конкретный mesher, UI, управление камерой) можно дорабатывать по ходу.

---

## 7. Добить “ощущение живого мира” (опционально) 

Это всё можно дописывать поверх уже работающей сцены:

### Редактирование DUN по клику
Raycast → определяем DUN + точку столкновения.
Перевод в локальные (x,y,z) и voxel = 0.
Пометка Dirty → ремешинг только этого DUN.
Визуально: дырки, выкусы, вмятины.

### Поворот и движение самого DUN
RigidBody::Dynamic + кручение/толкание.
Показать, что DUN крутится/движется, а воксели внутри остаются в своём grid.

### Несколько материалов / цветов
u8 материал → пару простых типов (камень, металл, стекло).
Разная визуалка/плотность, но всё ещё один DUN.

### Простые пресеты форм
В egui: кнопки типа Cube, Ramp, Stairs, Weird.
Быстрый спавн DUN с предзаданной воксельной формой — удобно для демо.


#### MVP1 — связка с настоящей топологией (цель на следующий этап)

- заменить `chunk_coord` на полноценный `Route` из Topology-дока (Region/Block/Chunk/Octo);
- сделать примитивный воксельный ландшафт из Static DUN по шуму;
- добавить простое включение/выключение DUN по расстоянию до камеры.

---

#### Готовая структура (для PowerShell) *(на всякий случай, если пригодится)*

Полный список папок и файлов:

```
mkdir src\app
mkdir src\dun
mkdir src\voxel
mkdir src\render
mkdir src\physics
mkdir src\tools
mkdir src\mvp0

ni src\app\mod.rs
ni src\app\setup.rs
ni src\app\plugins.rs

ni src\dun\mod.rs
ni src\dun\dun.rs
ni src\dun\kind.rs
ni src\dun\transform.rs
ni src\dun\route_lite.rs
ni src\dun\spawn.rs

ni src\voxel\mod.rs
ni src\voxel\grid.rs
ni src\voxel\mesher.rs

ni src\render\mod.rs
ni src\render\mesh_builder.rs

ni src\physics\mod.rs
ni src\physics\collider_builder.rs


ni src\tools\mod.rs
ni src\tools\debug_grid.rs
ni src\tools\egui_panel.rs

ni src\mvp0\mod.rs
ni src\mvp0\scene.rs
ni src\mvp0\stress_test.rs
```

---
