**Document Status:** 🧩 Draft  
**Version:** 0.1.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2026-05-07  

---

# 🧩 Arden Engine — MVP Current Rebuild

## 🎯 Назначение

Этот документ фиксирует текущий технический rebuild MVP-направления **Arden Engine**.

Это не changelog, не финальная спецификация и не точное описание отдельных git-патчей.

Документ нужен, чтобы зафиксировать:

- что сейчас собрано в `src`;
- зачем эти части были добавлены;
- какие слои уже являются опорными;
- какие части пока сырые или временные;
- что будет дорабатываться позже;
- как текущая кодовая база связана с дальнейшими MVP-этапами.

Текущий rebuild рассматривается как общий implementation bundle, а не как набор идеально разделённых патчей.

---

## 🧭 Общая логика rebuild

Старый активный порядок был ближе к DUN-first MVP0:

```text
DUN prototype -> physics check -> stress test
````

Новый порядок разработки:

```text
Spatial Truth -> Pre-DUN Lab Sandbox -> DUN Stage -> EQ/HAOS
```

Это значит, что DUN больше не является первым фундаментом MVP.
Сначала фиксируется пространственная истина, затем управляемая lab sandbox, и только потом DUN как следующий runtime/container-слой.

---

# 1. Spatial Core

## Зачем добавлен

Spatial Core нужен как базовая пространственная истина движка.

Он отвечает за:

* размеры контейнеров;
* координатные структуры;
* адресные формы;
* переходы между world-space и дискретной topology;
* основу для debug/probe/HUD/tools.

Этот слой не является UI, редактором или DUN-системой.

---

## Что входит

* `Region`;
* `Chunk`;
* `Octochunk`;
* `Voxel`;
* `Sector`;
* `RuntimePosition`;
* `DensityKey`;
* `SimSectorKey`;
* `FullRoute`;
* validation helpers;
* mapping helpers.

---

## Source files

| File                     | Role                                                 |
| ------------------------ | ---------------------------------------------------- |
| `src/core/topology.rs`   | размеры, координаты, `RuntimePosition`, bounds check |
| `src/core/address.rs`    | address forms, validation, display helpers           |
| `src/core/mapping.rs`    | world/runtime/address conversion                     |
| `src/core/voxel/grid.rs` | dense voxel grid primitive                           |
| `src/core/mod.rs`        | public exports для active core API                   |

---

## Status

Опорный слой.

Требует дальнейшей синхронизации с архитектурной документацией и будущих тестов для mapping/address helpers.

---

# 2. Lab Sandbox

## Зачем добавлен

Lab Sandbox нужен как управляемая finite voxel-среда до перехода к DUN.

Это не полноценный editor UI.
Это техническая песочница для проверки:

* voxel storage;
* selection workflows;
* edit tools;
* volume operations;
* copy/paste payload;
* save/load snapshot;
* dirty/rebuild flow.

---

## Что входит

* finite world profile;
* Edit / Runtime mode;
* chunk-backed voxel world;
* Paint;
* Erase;
* SelectBox;
* Fill selection;
* Delete selection;
* Clipboard copy/paste;
* save/load snapshot;
* lab HUD summary.

---

## Source files

| File                   | Role                                     |
| ---------------------- | ---------------------------------------- |
| `src/lab/sandbox.rs`   | finite world profile, Edit/Runtime mode  |
| `src/lab/world.rs`     | storage truth: `DensityKey -> VoxelGrid` |
| `src/lab/selection.rs` | select-box state and bounds              |
| `src/lab/clipboard.rs` | temporary copy/paste volume payload      |
| `src/lab/save.rs`      | lab snapshot save/load                   |
| `src/lab/scene.rs`     | Bevy resource/system wiring              |
| `src/lab/mod.rs`       | lab module map and responsibility notes  |

---

## Status

Рабочий pre-DUN lab layer.

Часть поведения пока является lab-level и позже может быть вынесена в более чистый engine API.

---

# 3. Volume Operation Backend

## Зачем добавлен

Старый путь `voxel-by-voxel` плохо подходит для больших selection operations.

Volume backend нужен, чтобы операции выполнялись не как прямой обход каждого voxel через tool-layer, а через план:

```text
intent -> operation plan -> chunk pass -> octochunk refine -> voxel frontier -> dirty queue
```

---

## Что входит

* `VolumeOpKind`;
* `VolumeIntent`;
* `OperationPlan`;
* `ChunkPlan`;
* `OctoPlan`;
* full chunk path;
* partial chunk path;
* octochunk refine;
* voxel frontier;
* dirty queue.

---

## Source files

| File                   | Role                                   |
| ---------------------- | -------------------------------------- |
| `src/lab/volume.rs`    | planner/executor for volume operations |
| `src/lab/world.rs`     | storage helpers used by executor       |
| `src/lab/selection.rs` | source bounds for volume intent        |

---

## Status

Архитектурно правильное направление, но пока это MVP/lab backend.

Позже dirty/rebuild flow может быть пересобран под более явные render/collider/runtime queues.

---

# 4. Probe / HUD / Gizmos / Debug Tools

## Зачем добавлено

Этот слой нужен, чтобы spatial truth была видимой и проверяемой в сцене.

Без него topology/mapping слой существует только как код, но не как проверяемая runtime-картина.

---

## Что входит

* camera probe;
* inspect under crosshair;
* pinned target;
* machine/human notation;
* debug lenses;
* layered gizmos;
* current tool state;
* debug key bindings;
* fly camera controls.

---

## Source files

| File                             | Role                                                |
| -------------------------------- | --------------------------------------------------- |
| `src/lab/probe.rs`               | camera/inspect/pinned target resolving              |
| `src/lab/formatters.rs`          | machine/human formatting                            |
| `src/lab/hud.rs`                 | debug overlay                                       |
| `src/lab/gizmos.rs`              | Region/Sector/Chunk/Octo/Voxel/Selection wireframes |
| `src/tools/debug/config.rs`      | key bindings                                        |
| `src/tools/debug/input.rs`       | debug input handling                                |
| `src/tools/debug/state.rs`       | debug UI state                                      |
| `src/tools/camera_controller.rs` | fly camera                                          |

---

## Status

Рабочий debug/tools слой.

DUN lens пока является stub и будет подключаться позже.

---

# 5. Render Layer

## Зачем добавлен

Render layer нужен для отображения текущего `LabVoxelWorld`.

Render не является source of truth.
Истина хранится в lab/core слоях, а render только строит визуальное представление.

---

## Source files

| File                             | Role                                |
| -------------------------------- | ----------------------------------- |
| `src/render/mesh_builder.rs`     | `VoxelGrid -> Bevy Mesh`            |
| `src/render/lab_chunk_render.rs` | chunk render rebuild from lab world |

---

## Status

Временный lab render path.

Позже потребуется отдельная политика rebuild для:

* mesh;
* collider;
* runtime objects;
* DUN objects.

---

# 6. DUN Bridge / Future Prep

## Зачем оставлено

DUN пока не активный фундамент текущего MVP.

Он остаётся следующим stage после:

```text
Spatial Truth
-> Pre-DUN Lab Sandbox
```

---

## Сейчас

* DUN lens существует как будущая точка подключения.
* Старая DUN-first документация пока может оставаться как historical reference.
* Реальный DUN stage будет оформляться позже.

---

## Later

* DUN lens contract;
* Static DUN;
* Dynamic DUN;
* route anchor;
* runtime transform bridge;
* generated mesh/collider pipeline;
* DUN physics integration;
* DUN documentation refresh.

---

# 🧰 Technical Debt / Cleanup Notes

## Сделано временно или сыро

* DUN lens пока stub.
* Save/load — lab snapshot, не production save format.
* Dirty/rebuild queue — lab-level механизм.
* Render rebuild пока связан с lab world.
* Часть старой документации всё ещё описывает DUN-first порядок.
* Часть структуры `src` ещё может отражать старую MVP0-логику.
* MVP-документация ещё не полностью синхронизирована с текущим code state.

---

## Что поправить позже

* Обновить `MVP_Structure.md` как короткий индекс.
* Пометить `MVP_0.1.md` как historical / archived.
* Обновить `meta/RU/project_status.md`.
* Обновить `meta/status_index.md`.
* Провести portal/link cleanup.
* Позже решить, нужны ли отдельные документы для MVP 1 и MVP 2.
* Синхронизировать Concept после MVP-документов.
* Позже обновить EN-версии.

---

# 🚫 Not in Scope

В текущий rebuild не входит:

* полноценный editor UI;
* production save/load;
* collider rebuild pipeline;
* Dynamic DUN physics;
* HAOS scheduling;
* EQ runtime orchestration;
* streaming / LOD;
* финальная EN-синхронизация;
* публичная stable-сборка.

---

# ➡ Next Documentation Step

Следующий порядок:

1. Сократить `MVP_Structure.md` до роли индекса.
2. Добавить запись в `meta/log_2026.md`.
3. Позже обновить `meta/RU/project_status.md`.
4. Потом перейти к Concept refresh.
5. Потом отдельно сделать portal/link cleanup.

---

# 📘 Related

[← Back to MVP Structure](./MVP_Structure.md)

[← Back to Roadmap](../roadmap.md)

[← Back to Meta](../../../meta/RU/README.md)

---

