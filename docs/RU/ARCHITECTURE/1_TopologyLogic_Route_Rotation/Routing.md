**Document Status:** 🔬 Review  
**Version:** 0.2.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2026-04-16  
**Purpose:** Active MVP routing canon for Arden

---

[📚 Вернуться назад](./README.md)

[🧱 Архитектурный портал проекта](../../ARCHITECTURE/readme.md)

# **1.0. Обзор / mini-README раздела Routing**

Раздел Routing формирует **активную MVP-модель адресации Arden**.

Routing не является математикой пространства сам по себе.
Он живёт **поверх Topology** и использует уже зафиксированные:

* контейнеры,
* размеры,
* страйды,
* координатную модель,
* формулы преобразований.

В active MVP Routing больше **не трактуется** как обязательный постоянный runtime-скелет мира.

Его роль теперь такая:

> **Routing = адресный протокол по запросу**

То есть Routing отвечает не за “как объект живёт каждую миллисекунду в рантайме”,
а за то, **как системы получают и передают дискретный адрес нужной глубины**.

---

## **1.0.1. Что задаёт Routing**

Раздел 1.x фиксирует:

* различие между:
  * **RuntimePosition**
  * **DensityKey**
  * **SimSectorKey**
  * **FullRoute**

* правила, по которым:
  * runtime-позиция превращается в адрес,
  * адрес превращается в локальные/мировые координаты,
  * одна система передаёт адрес другой системе без потери смысла;

* канонические формы записи:
  * структурные,
  * строковые,
  * контекстные сокращённые;

* базовые операции над адресами:
  * сравнение,
  * принадлежность,
  * соседство,
  * подъём/опускание уровня,
  * нормализация,
  * преобразования между формами адресации.

Routing не вводит новую топологию,
а использует уже зафиксированный active MVP-канон:

```text
Region → Chunk → Octochunk → Voxel
````

и отдельный sim-overlay:

```text
RegionSector = 2 × 2 × 2
```

---

## **1.0.2. Структура раздела 1.x**

**1.1. Понятие Routing и область применения**
Определяет Routing как слой адресного протокола,
а не как постоянное состояние рантайма.

**1.2. Канонические формы адреса**
Фиксирует четыре главные формы:

* RuntimePosition
* DensityKey
* SimSectorKey
* FullRoute

**1.3. Инварианты и валидность**
Определяет, что считается корректным адресом в active MVP.

**1.4. Связь Routing с Topology и координатами**
Показывает переходы:

* WorldXYZ ↔ RuntimePosition
* RuntimePosition ↔ DensityKey
* RuntimePosition ↔ SimSectorKey
* RuntimePosition ↔ FullRoute

**1.5. Операции над адресами**
Сравнение, вложенность, соседство, offset, Up/Down, normalize.

**1.6. Строковое представление**
Формат для логов, инспекторов, devtools и отладки.

**1.7. Mapping / Address API**
Минимальный набор операций сопоставления и конвертации.

**1.8. Использование Routing в системах движка**
Как active MVP-формы адреса используются в runtime, density, sim, tooling и ECS.

---

## **1.0.3. Как читать этот раздел**

* Если нужно понять, **что Routing теперь означает в active MVP** — читай `1.1–1.2`.
* Если важна **корректность и инварианты** — `1.3`.
* Если работаешь с преобразованиями координат и адресов — `1.4`.
* Если проектируешь операции над адресами — `1.5`.
* Если нужен лог/инспектор/CLI — `1.6`.
* Если нужен рабочий интерфейс конвертаций — `1.7`.
* Если нужна связь с подсистемами движка — `1.8`.

---

# **1.1. Понятие Routing и область применения**

Routing в active MVP — это **не один перегруженный объект**, а слой правил,
по которым система получает **адрес нужной глубины под задачу**.

Главная мысль:

> Runtime живёт на позиции.
> Routing живёт на адресе.

То есть:

* камера,
* игрок,
* физика,
* навигация,
* лучи,
* инструменты движения

не обязаны постоянно носить на себе полный маршрут через все уровни.

Их базовое живое состояние:

```text
Region + LocalFloat
```

А вот когда системе нужен дискретный адрес,
Routing строит одну из канонических форм адресации.

---

## **1.1.1. Routing не равен Runtime**

В active MVP нужно жёстко различать:

### **RuntimePosition**

Живое положение объекта:

```text
RegionCoord + LocalFloat
```

Это не Route.

---

### **Route / Address**

Дискретная адресная форма,
которая строится из runtime-положения или другого контекста,
когда системе нужен:

* ключ контейнера,
* адрес вокселя,
* sim-sector,
* debug-путь,
* сериализуемая ссылка.

---

## **1.1.2. Почему Routing больше не один “супер-объект”**

Старый подход перегружал один Route сразу несколькими ролями:

* runtime-state,
* адрес контейнера,
* глубокий voxel-route,
* симуляционный ключ,
* debug-string.

В active MVP эти роли разведены:

* `RuntimePosition` — для живой позиции,
* `DensityKey` — для density-уровня,
* `SimSectorKey` — для coarse sim,
* `FullRoute` — для точного дискретного адреса.

Это делает модель проще,
чище для подсистем
и дешевле для поддержки.

---

## **1.1.3. Где используется Routing**

Routing нужен там, где системам недостаточно просто знать float-позицию.

Основные сценарии:

* **Density / streaming**

  * какой Chunk нужен,
  * какой Chunk обновлять,
  * какой Chunk хранит плотность / меш / компрессию.

* **Sim / orchestration**

  * в какой RegionSector попал объект,
  * какой coarse sim bucket должен проснуться или уснуть.

* **Tools / debug**

  * инспектор,
  * лог адреса,
  * точечная команда ядру,
  * deep debug.

* **Serialization**

  * точный или сокращённый дискретный адрес,
  * воспроизводимая ссылка на область.

* **ECS / resource indexing**

  * стабильные ключи для entity/meta/storage.

---

## **1.1.4. Routing как общий язык между подсистемами**

Разные подсистемы могут иметь разную внутреннюю реализацию,
но адресный контракт между ними должен оставаться единым.

Например:

* runtime говорит:
  “объект находится в `Region + LocalFloat`”

* density-система хочет:
  `DensityKey = Region + Chunk`

* sim-система хочет:
  `SimSectorKey = Region + SectorCoord`

* debug/tooling хочет:
  `FullRoute` или его строковое представление.

Routing как раз и задаёт,
как эти формы выводятся друг из друга
и как они не теряют согласованность.

---

# **1.2. Канонические формы адреса**

В active MVP Routing использует **не одну**, а несколько строгих форм адресации.

---

## **1.2.1. RuntimePosition**

Базовая runtime-форма:

```text
RuntimePosition {
    region = RegionCoord
    local  = LocalFloat
}
```

Где:

* `RegionCoord = (rx, ry, rz)`
* `LocalFloat = (fx, fy, fz)`, `0 ≤ f* < REGION_SIZE`

RuntimePosition:

* не является Route;
* не хранит дискретный deep-address;
* является базовым живым состоянием для движения и физики.

---

## **1.2.2. DensityKey**

Ключ density-уровня:

```text
DensityKey {
    region = RegionCoord
    chunk  = ChunkCoord
}
```

Где:

* `ChunkCoord = (cx, cy, cz)`

Это главная активная адресная форма для:

* chunk streaming,
* density payload,
* mesh build,
* compression / decompression,
* chunk-based generation.

---

## **1.2.3. SimSectorKey**

Ключ coarse sim-overlay:

```text
SimSectorKey {
    region = RegionCoord
    sector = SectorCoord
}
```

Где:

* `SectorCoord = (sx, sy, sz)`, `sx, sy, sz ∈ {0,1}`

`SectorId ∈ [0..7]` допускается как packed/helper representation,
но не является первичной канонической формой `SimSectorKey`.

Эта форма нужна для:

* coarse sim streaming,
* activity buckets,
* sleep / awake / degradation,
* broad-phase orchestration.

---

## **1.2.4. FullRoute**

Точный дискретный адрес активной density-цепочки:

```text
FullRoute {
    region = RegionCoord
    chunk  = ChunkCoord
    octo   = OctochunkCoord
    voxel  = VoxelCoord
}
```

Где:

* `OctochunkCoord = (ox, oy, oz)`
* `VoxelCoord = (vx, vy, vz)`

FullRoute нужен для:

* deep debug,
* точечного редактирования,
* точных дискретных команд,
* сериализации,
* devtools,
* редких межсистемных ссылок.

---

## **1.2.5. Contextual Local Route**

Допускаются и контекстные локальные формы,
если внешний контейнер уже известен системе.

Например:

```text
Chunk + Voxel
Octochunk + Voxel
Chunk + Octochunk
```

Но важно:

* это не глобальный address form;
* такие формы валидны **только в явно заданном контексте**;
* они не заменяют `DensityKey`, `SimSectorKey` или `FullRoute`.

---

## **1.2.6. Что убрано из active MVP Routing**

В active MVP:

* **Block** не является обязательной частью address model;
* **Octant** не является частью machine-truth route;
* буквы `A/B/C/D/E/F/G/I` не являются обязательной частью дискретного адреса.

Они могут жить:

* в архивных/legacy-материалах,
* в human/debug overlay,
* в визуальных обозначениях сектора,

но не задают active machine-addressing canon.

---

## **1.2.7. Route как umbrella-term**

Для удобства речи слово **Route** может использоваться как общий термин
для дискретных адресных форм.

Но в active MVP это не должно размывать роли.

Рекомендуемое правило:

* если речь о live-position — говорить `RuntimePosition`;
* если речь о chunk key — говорить `DensityKey`;
* если речь о sim overlay — говорить `SimSectorKey`;
* если речь о deep discrete address — говорить `FullRoute`.

---

# **1.3. Инварианты и валидность**

Active MVP Routing требует,
чтобы любая адресная форма была согласована с Topology
и не подменяла одну роль другой.

---

## **1.3.1. Global address всегда содержит Region**

Любая глобальная address form должна содержать:

```text
RegionCoord = (rx, ry, rz)
```

Без Region адрес не считается глобально интерпретируемым.

Следовательно:

* `DensityKey` без Region — невалиден как глобальный ключ;
* `SimSectorKey` без Region — невалиден как глобальный ключ;
* `FullRoute` без Region — невалиден.

---

## **1.3.2. Порядок active density-уровней фиксирован**

Порядок уровней не меняется:

```text
Region → Chunk → Octochunk → Voxel
```

Инварианты:

* `Chunk` не может появляться после `Octochunk`;
* `Voxel` не может существовать как часть FullRoute без `Chunk` и `Octochunk`;
* contextual local forms допустимы,
  но только если явно задан внешний контейнер.

---

## **1.3.3. Локальные координаты всегда неотрицательны**

Любые локальные индексы address forms:

```text
cx, cy, cz ≥ 0
ox, oy, oz ≥ 0
vx, vy, vz ≥ 0
sx, sy, sz ≥ 0
```

И должны лежать в диапазонах active topology.

---

## **1.3.4. Sim truth числовая, не буквенная**

Для `RegionSector`:

* machine-truth = `SectorCoord` или `SectorId`;
* буквенная форма (`A/B/C/D/E/F/G/I`) — только human/debug overlay.

Следовательно:

* буквы не участвуют в машинной валидации;
* буквы не являются обязательным полем адреса;
* буквенные подписи не должны подменять `SectorCoord` / `SectorId`.

---

## **1.3.5. FullRoute должен быть внутренне согласован**

Для полного адреса обязательно:

* `voxel` принадлежит `octo`,
* `octo` принадлежит `chunk`,
* `chunk` принадлежит `region`.

Это означает:

* индексы должны быть в диапазонах;
* итоговая локальная точка должна лежать внутри Region;
* никакой уровень не может ссылаться “за пределы” контейнера выше.

---

## **1.3.6. Partial / task-specific addresses тоже должны быть однозначны**

`DensityKey` считается валидным, если:

* есть Region,
* есть Chunk,
* координаты Chunk допустимы.

`SimSectorKey` считается валидным, если:

* есть Region,
* sector корректен как `(sx,sy,sz)` или `sector_id`,
* сектор принадлежит диапазону active Region overlay.

Контекстные локальные формы валидны только если:

* внешний контейнер явно известен,
* интерпретация адреса однозначна.

---

## **1.3.7. Обратимость с Topology**

Адресная форма считается корректной,
если она согласована с active topology
и может быть корректно преобразована:

* в локальную геометрию Region,
* в `RuntimePosition`-якорь,
* в `WorldXYZ`,
* или в другую допустимую address form.

---

# **1.4. Связь Routing с Topology и координатами**

Routing опирается на Topology,
но не подменяет её.

Topology даёт:

* размеры,
* страйды,
* диапазоны,
* формулы `World ↔ Region ↔ local indices`.

Routing даёт:

* адресные формы,
* правила переходов между ними,
* смысловую роль каждой формы.

---

## **1.4.1. WorldXYZ ↔ RuntimePosition**

Базовый runtime-мост:

```text
WorldXYZ ↔ RegionCoord + LocalFloat
```

Где:

* `RegionCoord` определяет куб Region,
* `LocalFloat` — непрерывную позицию внутри него.

Это базовое runtime-state active MVP.

---

## **1.4.2. RuntimePosition → DensityKey**

Из `RuntimePosition` можно в любой момент вычислить:

```text
ChunkCoord = floor(LocalFloat / CHUNK_SIZE)
```

Тем самым:

```text
RuntimePosition → DensityKey
```

Это и есть основной практический переход
из live-position к density-address.

---

## **1.4.3. RuntimePosition → SimSectorKey**

Из той же `RuntimePosition`
можно независимо вычислить sim-sector:

```text
SectorCoord = floor(LocalFloat / REGION_SECTOR_SIZE)
```

или упакованный:

```text
SectorId = pack(SectorCoord)
```

То есть:

```text
RuntimePosition → SimSectorKey
```

без участия chunk-address.

---

## **1.4.4. RuntimePosition → FullRoute**

Если нужна точная deep-addressing форма,
из `RuntimePosition` строится:

1. `ChunkCoord`
2. остаток внутри Chunk
3. `OctochunkCoord`
4. остаток внутри Octochunk
5. `VoxelCoord`

И получается:

```text
RuntimePosition → FullRoute
```

Это on-demand операция,
а не обязательное постоянное состояние runtime.

---

## **1.4.5. FullRoute → RuntimePosition**

Из `FullRoute` можно восстановить опорную runtime-позицию внутри Region.

Типовые anchor modes:

* `corner`
* `center_of_voxel`
* `center_of_octo`
* `center_of_chunk`

Это важно для:

* debug,
* tool selection,
* якорных точек,
* перехода от дискретного адреса к визуальному/физическому положению.

---

## **1.4.6. DensityKey ↔ FullRoute**

`DensityKey` можно рассматривать как:

* обрезанный адрес chunk-уровня;
* контейнер,
  внутри которого могут существовать все возможные `FullRoute`,
  начинающиеся с того же `Region + Chunk`.

И наоборот:

* любой `FullRoute` однозначно сворачивается в `DensityKey`
  простым отбрасыванием нижних уровней.

---

## **1.4.7. SimSectorKey ↔ геометрия сектора**

`SimSectorKey` задаёт не точку,
а coarse spatial bucket.

Из него можно восстановить:

* базовый угол сектора внутри Region,
* bounding volume сектора,
* его local AABB внутри Region.

---

## **1.4.8. Contextual local addressing**

Если система уже работает внутри известного Chunk,
допускаются локальные формы вроде:

```text
C(...) / v(...)
```

Но важно:

* это не глобальный Route;
* это не универсальная форма межсистемного обмена;
* при выходе за пределы контекста такой адрес должен быть расширен
  до `DensityKey` или `FullRoute`.

---

# **1.5. Операции над адресами**

Routing задаёт минимальный рабочий набор операций.

---

## **1.5.1. Equality**

Адреса равны,
если совпадают все их обязательные и явно присутствующие поля.

Примеры:

* два `DensityKey` равны, если совпадают `region + chunk`;
* два `SimSectorKey` равны, если совпадают `region + sector`;
* два `FullRoute` равны, если совпадают `region + chunk + octo + voxel`.

---

## **1.5.2. Ordering**

Для deterministic sorting рекомендуется лексикографический порядок по уровням.

Примеры:

* `DensityKey`: `region → chunk`
* `SimSectorKey`: `region → sector`
* `FullRoute`: `region → chunk → octo → voxel`

---

## **1.5.3. Contains / Within**

Допустимые отношения вложенности:

* `Region` содержит любой `DensityKey`, `SimSectorKey`, `FullRoute` с тем же Region;
* `DensityKey` содержит любой `FullRoute` с тем же `region + chunk`;
* `SimSectorKey` содержит любую локальную точку / runtime-position,
  попадающую в объём сектора.

Важно:

* `SimSectorKey` не является родителем `Chunk` в active topology;
* `SimSectorKey` и `DensityKey` — разные адресные срезы одного Region.

---

## **1.5.4. Up / Down**

Для density-addressing:

* `FullRoute → DensityKey` = поднятие к chunk-уровню;
* `DensityKey → FullRoute range` = опускание до множества возможных deep-addresses внутри chunk.

Для sim-overlay:

* `Region → SimSectorKey children`
* `SimSectorKey → Region parent`

`Block` в active MVP в этих операциях не участвует.

---

## **1.5.5. Offset**

Offset имеет смысл только для тех форм,
которые допускают геометрически однозначное смещение.

### Для `RuntimePosition`

смещение естественно:

```text
local/world vector addition
```

### Для `DensityKey`

обычно используется как:

* соседний chunk,
* offset по chunk-grid.

### Для `FullRoute`

может использоваться как:

* смещение по voxel-grid,
* последующая нормализация в новый `FullRoute`.

### Для `SimSectorKey`

обычно применяется как:

* переход к соседнему сектору внутри Region
  или через Region boundary при явной политике переноса.

---

## **1.5.6. Neighbor predicates**

Базовые полезные предикаты:

* `same_region(a, b)`
* `same_chunk(a, b)`
* `same_sector(a, b)`
* `adjacent_chunks(a, b)`
* `adjacent_sectors(a, b)`

Они используются в:

* streaming,
* sim orchestration,
* mesh updates,
* tooling,
* debug traversal.

---

## **1.5.7. Normalize**

Нормализация проверяет,
что address form:

* лежит в допустимых диапазонах;
* не нарушает структуру уровней;
* согласована с active topology;
* может быть интерпретирована однозначно.

Нормализация рекомендуется после:

* ручной сборки адреса,
* десериализации,
* offset,
* переходов между формами.

---

## **1.5.8. Address → Range**

Некоторые формы адресуют не точку, а объём:

* `DensityKey` → chunk volume
* `SimSectorKey` → sector volume

Следовательно любая такая форма должна уметь разворачиваться в:

* local bounds,
* voxel range,
* world/local AABB

в зависимости от задачи.

---

# **1.6. Строковое представление**

Строковый формат нужен для:

* логов,
* devtools,
* inspector/debug,
* CLI,
* временных R&D-инструментов.

Канонический формат должен быть:

* обратимым,
* однозначным,
* компактным,
* согласованным с active MVP-формами адреса.

---

## **1.6.1. Общая структура**

Сегменты разделяются `/`.

Канонические префиксы:

* `R(...)` — Region
* `S(...)` — SectorCoord
* `C(...)` — ChunkCoord
* `O(...)` — OctochunkCoord
* `v(...)` — VoxelCoord

Порядок осей внутри сегментов:

```text
x | y | z
```

То есть:

* `R(rx|ry|rz)`
* `S(sx|sy|sz)`
* `C(cx|cy|cz)`
* `O(ox|oy|oz)`
* `v(vx|vy|vz)`

---

## **1.6.2. Канонические строки active MVP**

### RuntimePosition

Обычно не сериализуется как чистый `RouteString`,
потому что это live-position,
но при необходимости может иметь debug-форму:

```text
R(0|0|0) / p(123.5|42.0|87.25)
```

Это debug/helper формат,
не основная дискретная address form.
Дополнительный префикс `p(...)` может использоваться
только в debug/helper-представлении `RuntimePosition`.

---

### DensityKey

```text
R(0|0|0) / C(7|2|3)
```

---

### SimSectorKey

```text
R(0|0|0) / S(1|0|1)
```

или, если проекту удобнее packed-вид:

```text
R(0|0|0) / S#5
```

Но один из вариантов должен быть выбран как канонический.
Для active MVP безопаснее базовый вид:

```text
S(sx|sy|sz)
```

---

### FullRoute

```text
R(0|0|0) / C(7|2|3) / O(1|0|1) / v(12|4|29)
```

---

## **1.6.3. Contextual local strings**

Допускаются локальные строки вроде:

```text
C(7|2|3) / v(12|4|29)
O(1|0|1) / v(12|4|29)
```

Но они валидны только внутри явно известного внешнего контекста.

Для межсистемного обмена рекомендуется использовать только глобальные формы,
включающие `R(...)`.

---

## **1.6.4. Обязательные и опциональные сегменты**

Для глобальных address forms обязательно:

* `R(...)`

Остальные сегменты зависят от формы:

* `DensityKey` → `R / C`
* `SimSectorKey` → `R / S`
* `FullRoute` → `R / C / O / v`

В active MVP:

* `Octant` не входит в каноническую машинную строку;
* `Block` не входит в active route string.

---

## **1.6.5. Human/debug labels сектора**

Буквы `A/B/C/D/E/F/G/I` могут использоваться в UI, логах и overlay,
но не должны быть обязательным machine-segment.

Допустимая debug-надстройка:

```text
R(0|0|0) / S(1|0|1) @label(D)
```

или:

```text
R(0|0|0) / S(1|0|1) #D
```

Но основная truth-часть строки остаётся числовой.

---

## **1.6.6. Parse / format invariant**

Инвариант строкового формата:

```text
parse(format(Address)) == Address
```

Требования к парсеру:

* фиксированный порядок сегментов;
* корректные диапазоны индексов;
* отсутствие неявного домысливания обязательных уровней;
* optional metadata должна игнорироваться,
  если она не нужна основной логике.

---

## **1.6.7. Примеры**

Корректно:

```text
R(0|0|0) / C(7|2|3)
R(0|0|0) / S(1|0|1)
R(-1|0|2) / C(4|3|1) / O(0|1|0) / v(7|12|5)
C(7|2|3) / v(12|4|29)     // только локальный контекст
```

Некорректно:

```text
C(7|2|3)                  // как глобальный адрес без Region
R(0|0|0) / v(1|2|3)      // как FullRoute без Chunk/Octo
R(0|0|0) / O(1|0|1)      // пропущен Chunk
R(0|0|0) / S(2|0|0)      // сектор вне диапазона
```

---

# **1.7. Mapping / Address API**

Этот раздел задаёт минимальный интерфейс конвертаций и извлечения адресных данных.

Он не повторяет Topology-математику,
а оформляет её в usable address-contract.

---

## **1.7.1. Базовые извлечения**

```text
region_of(runtime|density|sector|full) -> RegionCoord
chunk_of(density|full)                 -> ChunkCoord
sector_of(sim)                         -> SectorCoord 
octo_of(full)                          -> OctochunkCoord
voxel_of(full)                         -> VoxelCoord
```

---

## **1.7.2. Основные конвертации**

```text
world_to_runtime(WorldXYZ)                  -> RuntimePosition
runtime_to_world(RuntimePosition)           -> WorldXYZ

runtime_to_density_key(RuntimePosition)     -> DensityKey
runtime_to_sim_sector(RuntimePosition)      -> SimSectorKey
runtime_to_full_route(RuntimePosition)      -> FullRoute

full_route_to_runtime(FullRoute, anchor)    -> RuntimePosition
density_key_to_chunk_bounds(DensityKey)     -> ChunkBounds
sim_sector_to_bounds(SimSectorKey)          -> SectorBounds
```

---

## **1.7.3. Идентичность и соседство**

```text
same_region(a, b)       -> bool
same_chunk(a, b)        -> bool
same_sector(a, b)       -> bool

adjacent_chunks(a, b)   -> bool
adjacent_sectors(a, b)  -> bool
```

---

## **1.7.4. Level traversal**

```text
full_to_density(full)               -> DensityKey
density_children_octo(density)      -> iterator<OctochunkCoord>
octo_children_voxel(full|octo_ctx)  -> iterator<VoxelCoord>

region_sector_children(region)      -> iterator<SimSectorKey>
```

---

## **1.7.5. Optional index helpers**

Если подсистеме нужно:

```text
chunk_flat_index(DensityKey)        -> integer
octo_flat_index(FullRoute)          -> integer
voxel_flat_index(FullRoute)         -> integer

octo_morton(FullRoute)              -> integer
voxel_morton(FullRoute)             -> integer
sector_packed_id(SimSectorKey)      -> integer
```

Но сами формулы остаются частью Topology.

---

## **1.7.6. Cache policy**

Допускается кэширование производных значений:

* world anchor,
* chunk bounds,
* sector bounds,
* flat index,
* morton index,
* packed sector id.

Но source of truth остаются:

* `RuntimePosition`
* `DensityKey`
* `SimSectorKey`
* `FullRoute`

---

# **1.8. Использование Routing в системах движка**

Routing в active MVP распределяется по ролям.

---

## **1.8.1. Runtime**

Runtime живёт на:

```text
RuntimePosition = Region + LocalFloat
```

Это базовое состояние:

* камеры,
* игрока,
* физики,
* навигации,
* лучей,
* инструментов.

---

## **1.8.2. Density**

Density-системы живут на:

```text
DensityKey = Region + Chunk
```

Это ключ для:

* voxel payload,
* генерации плотности,
* chunk storage,
* mesh build,
* compression / decompression,
* density streaming.

---

## **1.8.3. Sim**

Sim orchestration живёт на:

```text
SimSectorKey = Region + Sector
```

Это ключ для:

* coarse sim streaming,
* activity buckets,
* awake / sleep / degradation,
* broad-phase sim zoning.

---

## **1.8.4. Deep tools / edit / debug**

Глубокие точечные системы используют:

```text
FullRoute = Region + Chunk + Octochunk + Voxel
```

Это нужно для:

* deep debug,
* voxel editing,
* inspector jump,
* точечных команд ядру,
* точной сериализации.

---

## **1.8.5. Human/debug layer**

Human/debug overlay живёт поверх machine truth:

* буквенные сектора,
* компактные подписи,
* centered anchors,
* inspector labels.

Machine truth не должен зависеть от этих обозначений.

---

## **1.8.6. Итог**

Active MVP Routing фиксирует простую иерархию ролей:

```text
RuntimePosition = live position
DensityKey      = chunk address
SimSectorKey    = coarse sim address
FullRoute       = deep discrete address
```

Это и есть новый active routing canon Arden.

---

[📚 Вернуться назад](./README.md)

[🧱 Архитектурный портал проекта](../../ARCHITECTURE/readme.md)  
