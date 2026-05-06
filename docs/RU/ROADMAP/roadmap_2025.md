**Document Status:** 🔬 Review
**Version:** 0.4.0
**Maintainer:** Nuengtoppin
**Last update:** 2026-05-07

---

# 🧭 **Arden Engine — Roadmap (2025–2026)**

## 🎯 Назначение

Данный документ описывает стратегические этапы развития **Arden Engine** —
от исследовательской базы (R&D foundation) до первых стабильных сборок.
Цель — зафиксировать *путь архитектуры*, а не расписание релизов.

Каждый этап строится поверх предыдущего и связан с системой MVP и `meta/project_status.md`.

---

## 🧩 Phase 0 — Research Foundation

**Цель:**  
проверить базовые идеи топологии, координат, маршрутизации и voxel/DUN-модели до активной реализации.

**Содержание:**

- первые схемы Region / Chunk / Voxel;
- эксперименты с координатами и вложенностью;
- первичная формулировка DUN, EQ, HAOS и Route;
- проверка идеи hybrid voxel + mesh approach.

**Результат:**  
проект получил первичный архитектурный язык и набор рабочих гипотез.

---

## 🧩 Phase 1 — Documentation and Repository Foundation

**Цель:**  
собрать базовую структуру репозитория и зафиксировать основные архитектурные документы.

**Содержание:**

- RU/EN documentation portal;
- Concept / Architecture / Terms;
- Topology / Routing / Rotation triad;
- DUN documentation draft;
- Meta/status system;
- licenses, README and repository structure.

**Результат:**  
проект получил документационный фундамент и основу для дальнейшего R&D-развития.

---

## 🧩 Phase 2 — Spatial Truth + Pre-DUN Lab Sandbox

**Цель:**  
собрать стабильную пространственную основу движка и конечную voxel-песочницу до перехода к DUN.

**Содержание:**

- активный spatial core:
  - `Region`;
  - `Chunk`;
  - `Octochunk`;
  - `Voxel`;
- runtime bridge:
  - `RuntimePosition = Region + LocalFloat`;
- адресные формы:
  - `DensityKey`;
  - `SimSectorKey`;
  - `FullRoute`;
- mapping pipeline:
  - world position → runtime position;
  - runtime position → density key;
  - runtime position → full route;
- debug/probe слой:
  - camera probe;
  - inspect under crosshair;
  - pinned target;
  - machine/human notation;
- layered gizmos:
  - Region;
  - Sector;
  - Chunk;
  - Octochunk;
  - Voxel;
  - Selection;
- finite lab sandbox:
  - `LabWorldProfile`;
  - Edit / Runtime mode;
  - `LabVoxelWorld`;
  - Paint / Erase;
  - SelectBox;
  - Fill / Delete volume;
  - Clipboard copy/paste;
  - save/load snapshot;
- volume operation backend:
  - operation intent;
  - chunk-aware planning;
  - octochunk refine;
  - voxel frontier;
  - dirty/rebuild queue.

**Результат:**  
проект получает управляемую pre-DUN песочницу, где можно проверять spatial truth, voxel editing, volume operations, payload workflows и будущую подготовку к DUN.

## 🧩 Phase 3 — DUN Stage

**Цель:**  
ввести DUN как слой поверх уже работающей spatial truth и lab sandbox.

**Содержание:**

- обновление DUN-документации под актуальный MVP-канон;
- DUN lens contract;
- Static DUN preparation;
- route anchor + runtime transform bridge;
- generated mesh/collider pipeline;
- подготовка перехода к Dynamic DUN.

**Результат:**  
DUN становится не первым фундаментом проекта, а следующим уровнем поверх проверенной topology/mapping/lab-основы.

## 🧩 Phase 4 — EQ / HAOS / Runtime Orchestration

**Цель:**  
перейти от lab sandbox к более явной системе runtime-оркестрации.

**Содержание:**

- EQ-Core / EQ-Sim boundary;
- DTO activity/sleep policy;
- HAOS optimization layer;
- dirty/rebuild queues;
- runtime events and system coordination;
- preparation for streaming, LOD and simulation scheduling.

**Результат:**  
движок получает основу для управляемой симуляции, оптимизации и будущей runtime-архитектуры.

---

## 🧩 Phase 5 — Stable Research Build

**Цель:**  
подготовить Arden Engine к открытым R&D-экспериментам и публикации.

**Содержание:**

- стабилизация core/lab/runtime layers;
- обновление RU/EN documentation;
- cleanup examples and public-facing docs;
- preparation of demo scenes;
- license and contribution flow validation.

**Результат:**  
Arden Engine готов к открытым R&D-экспериментам, демонстрации и дальнейшему open-collab развитию.

---

## 🔗 **Связанные документы**

| Раздел | Файл |
| ------ | ---- |
| 🧱 Статус проекта | [`meta/RU/project_status.md`](../../../meta/RU/project_status.md) |
| 🧩 Структура MVP | [`docs/RU/MVP/MVP_Structure.md`](../MVP/MVP_Structure.md) |
| 📚 Архитектура | [`docs/RU/ARCHITECTURE/readme.md`](../ARCHITECTURE/readme.md) |
| 📘 Концепт | [`docs/RU/CONCEPT/Concept_Overview.md`](../CONCEPT/Concept_Overview.md) |
| ⚙ Meta-панель | [`meta/RU/README.md`](../../../meta/RU/README.md) |
| 🧾 Status Index | [`meta/status_index.md`](../../../meta/status_index.md) |
---

[📚 Вернуться к дорожной карте](../roadmap.md)  

---

📚 [Вернуться к основному README →](../../../root/README_RU.md)

---
