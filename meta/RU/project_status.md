# 📊 Arden Engine — Project Status

---
**Document Status:** 🔬 Review  
**Version:** 0.2.1  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2026-05-07

---

## 🎯 Назначение

Документ отражает текущее состояние разработки проекта **Arden Engine**, включая прогресс по ключевым подсистемам, этапам MVP и общую динамику развития.

Используется для ручного контроля прогресса и синхронизации с дорожной картой, MVP-документацией и Meta-системой.

---

## 🔄 Current Focus Override — 2026-05-07

Текущий активный фокус проекта смещён от старого DUN-first MVP0 к новой последовательности разработки:

```text
Spatial Truth -> Pre-DUN Lab Sandbox -> DUN Stage -> EQ/HAOS
```

Текущий rebuild фиксирует первые три рабочих слоя:

```text
Step 1 — Spatial Truth Foundation
Step 2 — Pre-DUN Finite Lab Sandbox
Step 3 / DUN Stage
```

### Current Rebuild Documents

* `docs/RU/ROADMAP/roadmap_2025.md`
* `docs/RU/MVP/MVP_Structure.md`
* `docs/RU/MVP/MVP_Current_Rebuild.md`
* `meta/log_2026.md`

### Note

Таблицы ниже временно считаются **historical / pending refresh**.

Они отражают старую декабрьскую модель проекта и будут обновлены отдельным проходом после стабилизации MVP-документации, DUN Stage и Meta/status-системы.

---

## 🔄 Current Snapshot — май 2026

| Подсистема / слой | Состояние | Прогресс | Комментарий |
| ----------------- | --------- | -------- | ----------- |
| **Spatial Truth Foundation** | 🧪 Stable-Test | 85% | Работают `Topology / Routing / Mapping`, `RuntimePosition`, `DensityKey`, `SimSectorKey`, `FullRoute`, probe/HUD/gizmos. Нужны дальнейшие тесты и doc/code sync. |
| **Pre-DUN Finite Lab Sandbox** | 🧪 Stable-Test | 80% | Работают finite lab world, Edit/Runtime mode, Paint/Erase, SelectBox, Fill/Delete, chunk-aware volume backend, Copy/Paste, Save/Load snapshot. |
| **Pre-DUN UnitNode Bridge** | 🧪 Stable-Test | 60% | Работает цикл `World -> Object -> Move / Rotate -> Bake back`: `LabVoxelObject`, object registry, object render preview, C4 orientation, rotated bake, save/load objects. |
| **DUN Stage** | 🧩 Draft | 25% | Документально существует DUN-канон, но актуальный runtime-код ещё не оформлен как полноценный DUN layer. Следующий крупный архитектурный этап. |
| **Render Layer** | 🧩 Draft | 35% | Есть временный lab render path для world chunks и object preview. Render не является source of truth и позже требует rebuild policy. |
| **Physics / Collider Layer** | 🧩 Draft | 10% | Rapier подключён, но DUN/object collider pipeline ещё не интегрирован в текущий workflow. |
| **EQ-Core / HAOS / DTO** | 🧩 Draft | 10% | Пока концептуальный следующий слой после DUN Stage. Runtime orchestration, sleep/awake, rebuild scheduling и optimization policy ещё впереди. |
| **Docs RU** | 🔬 Review | 65% | RU-документы перестраиваются под новый порядок MVP. Step 1/2/3 описаны, DUN Stage и Concept refresh ещё впереди. |
| **Docs EN** | 🧩 Draft | 25% | EN-синхронизация отложена до стабилизации RU-канона и текущего MVP rebuild. |
| **Meta System** | 🧱 Stable | 100% | Система статусов и meta-структура остаются нормативной основой проекта. |

---

## 🧩 Current MVP Plan — май 2026

| Этап | Цель | Состояние | Комментарий |
| ---- | ---- | --------- | ----------- |
| **MVP 1** | Spatial Truth Foundation | 🧪 Stable-Test | Пространственная истина, address/mapping layer, probe, HUD, layered gizmos. |
| **MVP 2** | Pre-DUN Finite Lab Sandbox | 🧪 Stable-Test | Управляемая voxel sandbox: edit tools, selection, volume backend, clipboard, save/load. |
| **MVP 3** | Pre-DUN UnitNode Bridge | 🧪 Stable-Test | Extract Copy/Cut, object registry, object move, C4 rotation, save/load objects, rotated bake back. |
| **MVP 4** | DUN Stage | 🧩 Draft | Полноценный DUN contract в runtime: Static/Dynamic DUN, anchor, transform bridge, mesh/collider pipeline. |
| **MVP 5** | EQ / HAOS / Runtime Orchestration | 🧩 Draft | EQ-Core/EQ-Sim boundary, DTO, HAOS, rebuild queues, runtime scheduling. |
| **MVP 6** | Stable Research Build | 🧩 Draft | Cleanup, demo scene, RU/EN docs sync, public-facing R&D build, contribution readiness. |

---

## 🕯 Historical Note — декабрь 2025

Декабрьская модель MVP0 сохраняется как historical reference.

Старый порядок был ближе к:

```text
DUN prototype -> physics check -> stress test
```

Текущий активный порядок заменён на:

```text
Spatial Truth
-> Pre-DUN Lab Sandbox
-> Pre-DUN UnitNode Bridge
-> DUN Stage
-> EQ/HAOS
```

Старые таблицы December 2025 больше не используются как активный план разработки.

---

## 📚 Связанные документы

| Файл                                                                                   | Назначение                           |
| -------------------------------------------------------------------------------------- | ------------------------------------ |
| [`focus_2025.md`](focus_2025.md)                                                       | Приоритеты и календарный фокус       |
| [`../log_2025.md`](../log_2025.md)                                                     | Хронология событий и обновлений 2025 |
| [`../log_2026.md`](../log_2026.md)                                                     | Хронология событий и обновлений 2026 |
| [`../status_index.md`](../status_index.md)                                             | Реестр статусов всех файлов          |
| [`status_system.md`](status_system.md)                                                 | Справочник уровней зрелости          |
| [`../../docs/RU/roadmap.md`](../../docs/RU/roadmap.md)                                 | Внешний портал и Roadmap             |
| [`../../docs/RU/MVP/MVP_Current_Rebuild.md`](../../docs/RU/MVP/MVP_Current_Rebuild.md) | Текущий технический rebuild MVP      |

---

## 🧭 Примечание

* Файл обновляется вручную раз в 1–2 недели или после крупных архитектурных сдвигов.
* Текущий `Current Focus Override` добавлен как временный актуализирующий слой.
* Старые таблицы сохранены как historical snapshot до полного refresh.
* Полный rebuild этого файла будет сделан после описания Step 3 / DUN Stage и стабилизации MVP-документации.
* Все статусы позже должны быть сверены с [`../status_index.md`](../status_index.md).

---
