**Document Status:** 🧩 Draft  
**Version:** 0.3.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2026-05-07  

---

# 🧩 Arden Engine — MVP Structure

## 🎯 Назначение

Этот документ описывает общую структуру MVP-направления проекта **Arden Engine**.

MVP в контексте Arden Engine — это не финальный релиз и не строгая календарная сборка.  
Это технический слой проверки архитектуры, кода и исследовательских гипотез.

Документ нужен, чтобы зафиксировать:

- общий порядок MVP-слоёв;
- связь между Roadmap, кодовой базой и документацией;
- статус текущего rebuild;
- исторические MVP-прототипы;
- будущие направления развития.

Подробное описание текущего состояния `src` вынесено в:

[`MVP_Current_Rebuild.md`](./MVP_Current_Rebuild.md)

---

## 🧭 Текущая логика MVP

После пересмотра порядка разработки активный путь MVP больше не строится от DUN-first прототипа.

Новый порядок:

```text
Spatial Truth -> Pre-DUN Lab Sandbox -> DUN Stage -> EQ/HAOS
````

Это значит:

* сначала фиксируется пространственная истина;
* затем собирается управляемая pre-DUN voxel sandbox;
* затем вводится DUN как следующий runtime/container-слой;
* после этого можно переходить к EQ / HAOS / runtime orchestration.

---

## 🕯 MVP 0 — Historical DUN Prototype

**Status:** historical reference / to be archived later
**Document:** [`MVP_0.1.md`](./MVP_0.1.md)

### Purpose

Ранний DUN-first прототип для проверки идеи:

```text
VoxelGrid -> Surface Mesh -> Collider / Physics check
```

### Included

* Single DUN Prototype;
* Dynamic DUN stress test;
* surface mesh collider;
* шар / physics check;
* первичная проверка связи voxel grid → mesh → collider.

### Comment

Этот MVP сохраняется как исторический слой проекта.

Он больше не является активной основой разработки, потому что текущая архитектура требует сначала закрепить:

* spatial truth;
* address / mapping layer;
* finite lab sandbox;
* volume operation backend.

DUN возвращается позже как отдельный stage поверх этой основы.

---

## 🧩 MVP Current Rebuild

**Status:** 🧩 Draft / Active  
**Document:** [`MVP_Current_Rebuild.md`](./MVP_Current_Rebuild.md)

### Purpose

Зафиксировать текущий общий rebuild кодовой базы и документации после перехода от DUN-first MVP0 к новой последовательности:

```text
Spatial Truth -> Pre-DUN Lab Sandbox -> Pre-DUN UnitNode Bridge -> DUN Stage -> EQ/HAOS
```

### Includes

Текущий rebuild описывает первые три рабочих слоя:

```text
Step 1 — Spatial Truth Foundation
Step 2 — Pre-DUN Finite Lab Sandbox
Step 3 — Pre-DUN UnitNode Bridge
```

А также оставляет место для следующего полноценного DUN-прохода:

```text
Step 4 — DUN Stage
```

### Comment

`MVP_Current_Rebuild.md` не является финальной спецификацией.
Это техническая карта текущего состояния `src`, где фиксируется:

* что уже собрано;
* зачем это было добавлено;
* какие части являются опорными;
* какие части временные или сырые;
* что будет дорабатываться позже.

---

## 🧩 MVP 1 — Spatial Truth Foundation

**Status:** Draft / active base  
**Detailed document:** later, if needed

### Purpose

Закрепить пространственную истину движка.

### Includes

* topology layer;
* `Region`;
* `Chunk`;
* `Octochunk`;
* `Voxel`;
* `RuntimePosition`;
* `DensityKey`;
* `SimSectorKey`;
* `FullRoute`;
* world/runtime/address mapping;
* debug/probe/HUD/gizmos visibility.

### Result

Движок получает проверяемый spatial truth слой, который можно читать через runtime tools, HUD и gizmos.

---

## 🧩 MVP 2 — Pre-DUN Finite Lab Sandbox

**Status:** Draft / active  
**Detailed document:** later, if needed

### Purpose

Создать конечную voxel-песочницу до перехода к DUN.

### Includes

* finite lab world profile;
* Edit / Runtime mode;
* chunk-backed voxel storage;
* Paint / Erase;
* SelectBox;
* Fill / Delete volume;
* Clipboard copy/paste;
* lab save/load snapshot;
* volume operation backend;
* temporary lab render path.

### Result

Проект получает управляемую sandbox-среду для проверки voxel editing, selection workflows, payload operations и будущей подготовки к DUN.

---

## 🧩 MVP 3 — Pre-DUN UnitNode / DUN Bridge

**Status:** Draft / active local checkpoint  
**Detailed document:** [`MVP_Current_Rebuild.md`](./MVP_Current_Rebuild.md)

### Purpose

Проверить переход от мировой voxel-сетки к отдельной object/unit-node массе до полноценного DUN.

Этот слой не является финальным DUN.
Он нужен как подготовительный мост:

```text
World voxels -> Local payload object -> Unit movement -> C4 orientation -> Bake back
```

### Includes

* `LabVoxelObject`;
* local `VoxelPayload`;
* `LabObjectRegistry`;
* Extract Copy / Cut;
* object render preview;
* object bounds / pivot gizmo;
* selected object switching;
* selected object delete;
* object movement as one unit;
* object save/load inside lab snapshot;
* C4 orientation state;
* C4 yaw preview;
* rotated bake back to world.

### Result

Проект получает первый рабочий pre-DUN workflow:

```text
World -> Object -> Move / Rotate -> World
```

Это подготавливает будущий DUN Stage, но не заменяет его.

---

## 🧩 MVP 4 — DUN Stage

**Status:** planned / next architecture stage  
**Detailed document:** later

### Purpose

Ввести DUN как runtime/container-слой поверх уже работающей spatial truth, lab sandbox и pre-DUN UnitNode bridge.

### Planned

* DUN lens contract;
* Static DUN;
* Dynamic DUN;
* route anchor;
* runtime transform bridge;
* generated mesh/collider pipeline;
* DUN documentation refresh;
* eventual quaternion transform for Dynamic DUN.

### Result

DUN становится не первым фундаментом MVP, а следующим runtime/container-слоем поверх проверенной spatial architecture и object workflow.

---

## 🧩 MVP 5 — EQ / HAOS / Runtime Orchestration

**Status:** planned

### Purpose

Подготовить runtime-оркестрацию, оптимизацию и симуляционные правила.

### Planned

* EQ-Core / EQ-Sim boundary;
* DTO activity/sleep logic;
* HAOS optimization layer;
* rebuild scheduling;
* collider/mesh rebuild policy;
* event/intent pipeline;
* future streaming and LOD hooks.

---

## 🧩 MVP 6 — Stable Research Build

**Status:** planned

### Purpose

Подготовить Arden Engine к открытым R&D-экспериментам, демонстрации и дальнейшему open-collab развитию.

### Planned

* стабилизация core/lab/runtime layers;
* публично читаемая документация;
* RU/EN sync;
* cleaned examples;
* demo scene;
* contribution/license readiness.

---

## 🧰 Current TODO

* Дочистить `MVP_Current_Rebuild.md` по текущему состоянию `src`.
* Step 3 / Pre-DUN UnitNode Bridge описан в текущем rebuild.
* Позже оформить полноценный DUN Stage отдельным проходом.
* Позже пометить `MVP_0.1.md` как historical / archived.
* Синхронизировать `meta/log_2026.md`.
* Позже обновить `meta/RU/project_status.md`.
* Позже обновить `meta/status_index.md`.
* Провести portal/link cleanup отдельным проходом.
* Позже обновить Concept и EN-документы.

---

## 📘 Связанные материалы

[📄 MVP Current Rebuild](./MVP_Current_Rebuild.md)

[📚 Вернуться к дорожной карте](../roadmap.md)

[🧾 Панель управления проектом / Meta](../../../meta/RU/README.md)

[⚙ Глоссарий терминов](../TERMS/Glossary.md)

---

📚 [Вернуться к основному README →](../../../root/README_RU.md)

---
