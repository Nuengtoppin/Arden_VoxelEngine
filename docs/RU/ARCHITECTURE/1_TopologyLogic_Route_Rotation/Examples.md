**Document Status:** 🔬 Review  
**Version:** 0.2.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2026-04-16  
**Purpose:** Practical examples for active MVP Topology / Routing / Rotation

---

[📚 Вернуться назад](./README.md)

[🧱 Архитектурный портал проекта](../../ARCHITECTURE/readme.md)

---

# Examples — как пользоваться этим файлом

Этот документ собран поверх трёх базовых файлов:

- **Topology.md** — про структуру пространства, размеры, индексы и mapping,
- **Routing.md** — про active address forms и переадресацию,
- **Rotation.md** — про повороты, orientation-state и переориентацию содержимого.

Здесь мы не вводим новых правил,
а показываем на примерах,
как эти три слоя работают вместе в active MVP.

Если базовые документы — это сухой контракт,
то здесь — практический слой:

- как читать эти структуры глазами разработчика,
- как мыслить ими в runtime и tools,
- как переводить machine truth в compact/human form,
- как использовать это в генерации, copy/paste, movement и debug.

Этот файл нужен не для того,
чтобы ещё раз повторить канон,
а для того,
чтобы сделать его прикладным и живым.


## 1. Working profile

Чтобы примеры не были абстрактными,
мы дальше используем один рабочий профиль для проекта:

```text
CHUNK_SIZE              = 64
OCTO_SIZE               = 32
REGION_CHUNKS_PER_AXIS  = 16
REGION_SIZE             = 1024
REGION_SECTOR_SPLIT     = 2
REGION_SECTOR_SIZE      = 512
```

Он нужен только для того,
чтобы все дальнейшие примеры читались как одна и та же конкретная сцена,
а не распыляться на разные конфигурации.
Теория и опыт схожих проектов подсказывает,
что такая соразмерность выглядит в среднем рабочей и удобной для объяснения.

Мы получаем:

* Region как куб `1024 × 1024 × 1024`
* Chunk как основной плотный контейнер `64³`
* Octochunk как внутренний подслой `32³`
* RegionSector как coarse sim-секции `512³`

Такой профиль удобен тем,
что он остаётся достаточно крупным для практических примеров,
но при этом не раздувается до слишком тяжёлых чисел.

Важно:
этот профиль не является жёсткой догмой самого движка.
При необходимости можно менять плотность Chunk,
например использовать профили:

```text
16³ / 32³ / 64³ / 128³
```

При этом базовая логика не ломается:

* `Chunk` остаётся основным density-контейнером;
* `Octochunk` остаётся его внутренним подслоем;
* `RegionSector` остаётся coarse sim-разбиением `2 × 2 × 2`;
* active hierarchy не меняется.

Меняется только плотность и масштабы внутри профиля,
а не сама структура пространства.

То есть Examples использует один рабочий профиль для наглядности,
но сама система не привязана только к одному числу `64`.

---

## 2. One point across all layers

Одна и та же точка мира может быть представлена сразу на нескольких слоях.

Для live-runtime мы обычно думаем так:

```text
RuntimePosition = Region + LocalFloat
```

Это удобная форма для движения, камеры, навигации, лучей и других систем,
которым важна непрерывная позиция, а не полный дискретный адрес.

Если системе нужен основной плотный контейнер,
эта же позиция может быть сведена к:

```text
DensityKey = Region + Chunk
```

Если системе нужен coarse sim-уровень,
она же может быть сведена к:

```text
SimSectorKey = Region + SectorCoord
```

А если требуется максимально точное дискретное положение,
позиция может быть разложена глубже:

```text
FullRoute = Region + Chunk + Octochunk + Voxel
```

То есть active MVP опирается не на один перегруженный адрес,
а на несколько представлений одной и той же точки под разные задачи:

* runtime,
* density,
* sim,
* deep debug/edit.

---

### Region и LocalFloat: откуда начинается локальная координата

Когда мы говорим `Region + LocalFloat`,
важно понимать:
локальная координата внутри Region начинается не от центра,
а от origin-угла этого Region.

То есть у каждого Region есть свой локальный диапазон:

```text
LocalFloat ∈ [0 .. REGION_SIZE)
```

А сама локаль получается как:

```text
LocalFloat = WorldXYZ - RegionOriginWorld
```

Это означает:

* у каждого Region своя собственная локальная система;
* при переходе через границу Region меняется `RegionCoord`;
* локаль при этом не “ломается”,
  а просто пересобирается в новый диапазон того Region,
  в который попала world-позиция.

Именно поэтому один и тот же объект может жить в runtime как
`Region + LocalFloat`,
а уже потом по запросу раскладываться
в Chunk / Sector / Octo / Voxel.

---

### Что это даёт на практике

Из одной и той же runtime-позиции
система может по запросу получить:

* в каком Region находится объект;
* в каком Chunk он находится;
* в какой RegionSector он попадает;
* в какой Octochunk или Voxel нужно опуститься,
  если нужна более глубокая детализация.

Иными словами:

```text
одна world/runtime-позиция
-> несколько address forms
-> каждая под свою задачу
```

Это и есть одна из самых полезных идей active MVP:
не таскать всегда один “супер-адрес”,
а извлекать нужную глубину тогда, когда она действительно нужна.

---

## 3. Machine / compact / human representations

Одна и та же адресная информация может читаться на нескольких слоях.

Каноническая machine-form остаётся координатной:

```text
R(rx|ry|rz) / S(sx|sy|sz) / C(cx|cy|cz) / O(ox|oy|oz) / v(vx|vy|vz)
```

Это форма,
на которую опираются Topology, Routing и внутренние преобразования.

Но для логов, HUD, tools и ручной работы
часто удобнее использовать более короткие представления.

---

Compact helper layer может выглядеть так:

```text
R(...) / S#n / C#n / O#n / v(...)
```

Здесь:

* `S#n` — компактный id сектора;
* `C#n` — компактный id Chunk внутри Region;
* `O#n` — компактный id Octochunk внутри Chunk.

Такая форма не заменяет machine truth,
а просто делает адреса короче и удобнее для чтения.

---

Для debug-слоя адрес я предлагаю сделать таким образом 
Для overlay-слоя:

```text 
R(...)
Sector D
Chunk #17
Octo #3
Voxel (12|4|29)
```

мне это кажется удобным для:

* логов;
* HUD;
* инспектора;
* tool-команд;
* copy/paste и selection-сценариев.

а не базовая machine-form.

---

## 4. Compact 3-bit forms

В некоторых местах маленькие координатные домены удобно упаковывать в compact id.

Простейший случай:

```text
(0/1, 0/1, 0/1) <-> packed id 0..7
```

Для active MVP это особенно естественно там,
где пространство уже само по себе разбито как `2 × 2 × 2`.

Это относится прежде всего к:

* `RegionSector`
* `Octochunk` внутри Chunk

То есть:

* `SectorCoord = (sx, sy, sz)` можно упаковать в `S#n`
* `OctochunkCoord = (ox, oy, oz)` можно упаковать в `O#n`

Compact 3-bit form удобна для:

* helper ids в логах и HUD;
* lookup-таблиц;
* bitmask / occupancy;
* child-slot indexing;
* небольших fixed-domain состояний внутри `2 × 2 × 2`.

---

Важно не путать две разные вещи:

1. **неотрицательные локальные координаты**
2. **3-bit packing**

Неотрицательность появляется не из-за упаковки,
а из-за самой container-local topology:

* локаль внутри Region идёт в `[0 .. REGION_SIZE)`
* локальные индексы контейнеров тоже неотрицательны

3-bit form — это уже не основа topology,
а только компактная helper-форма для маленького домена.

---

Для маленьких fixed-domain уровней `2 × 2 × 2`
packed id выглядит естественно.

Но для более крупных координат, вроде:

* `ChunkCoord`
* `VoxelCoord`
* полного world/local mapping

каноническая форма всё равно удобнее как обычные `x / y / z`.

### Practical summary

Полезно мыслить так:

```text 
machine truth = coords
compact 3-bit = helper form for tiny fixed domains
```

Это позволяет:

* не ломать базовый канон координат;
* при этом иметь короткие `S#` и `O#`
  там, где они действительно упрощают чтение и tooling.

```md 
## 5. Registration and where data goes

В active MVP один объект не обязан жить как один перегруженный address object.

Практически полезнее мыслить так:
одна и та же сущность может одновременно раскладываться
по нескольким слоям,
и каждый слой хранит только то,
что действительно нужно ему по роли.

---

### Runtime layer

Для live-runtime базовая форма выглядит так:

```text
RuntimePosition = Region + LocalFloat
````

Это нужно там,
где важна непрерывная позиция:

* движение;
* камера;
* навигация;
* лучи и хит-тесты;
* общий runtime-state объекта.

Именно эта форма обычно является “живым” якорем объекта,
а не глубокий дискретный адрес.

---

### Density layer

Если системе нужен основной плотный контейнер,
она работает уже на уровне:

```text id="8ek877"
DensityKey = Region + Chunk
```

Это удобно для:

* chunk storage;
* voxel payload;
* mesh build;
* local generation;
* compression / decompression;
* chunk-based processing.

То есть density-слой не обязан знать live-position “как у камеры”,
ему нужен свой ключ контейнера.

---

### Sim layer

Если системе нужен coarse sim bucket,
она использует:

```text id="7a245d"
SimSectorKey = Region + SectorCoord
```

Это подходит для:

* coarse activity buckets;
* sleep / awake logic;
* sim orchestration;
* broad-phase grouping.

Sim layer не обязан хранить deep density-address.
Ему достаточно coarse address form,
которая отвечает на вопрос:
в каком крупном секторе Region сейчас живёт объект или область.

---

### Deep debug / edit / serialization

Когда нужна более глубокая дискретная точность,
система может опускаться до:

```text
FullRoute = Region + Chunk + Octochunk + Voxel
```

Это полезно для:

* deep debug;
* voxel edit;
* точечных команд;
* копирования и вставки;
* сериализации и проверок.

Но это не значит,
что весь движок обязан жить на `FullRoute` постоянно.

---

## 5. Orientation хранится отдельно

Если у сущности есть ориентация,
её полезно мыслить отдельно от address form.

Практическая модель выглядит так:

```text
AnchorAddress + Orientation + LocalContent
```

То есть:

* address отвечает на вопрос “где”;
* orientation отвечает на вопрос “в какой yaw-ориентации”;
* local content отвечает на вопрос “что именно хранится внутри”.

Это особенно важно для copy/paste, preview, blueprint-like данных и rotation-aware workflows.

---

### Куда “уходят данные”

Если говорить грубо и без привязки к конкретному backend,
удобно мыслить так:

* **runtime registry**
  хранит `RuntimePosition` и live-state

* **density storage**
  хранит `DensityKey` и chunk-local content

* **sim buckets**
  группируют объекты по `SimSectorKey`

* **deep tool/debug layer**
  при необходимости использует `FullRoute`

* **orientation / content layer**
  хранит ориентацию и локальное содержимое отдельно от address key

Это не storage spec,
а conceptual map:
она показывает,
почему один и тот же объект не обязан везде выглядеть одинаково.

---

### Что это даёт на практике

Такое разделение даёт несколько плюсов сразу:

* runtime не перегружается deep address-деталями;
* density-системы получают свой естественный ключ;
* sim-системы не вынуждены работать на уровне chunk или voxel;
* tool/debug слой может опускаться глубже только тогда,
  когда это действительно нужно.

Итоговая мысль здесь простая:

```text
один объект
-> несколько согласованных слоёв представления
-> каждый слой хранит только свою роль
```

Именно это делает регистрацию чище
и не заставляет весь движок жить в одном перегруженном формате.

---

## 6. Readdressing during movement

Когда объект движется в мире,
его address forms не обязаны пересобираться одинаково и постоянно.

В active MVP полезно мыслить так:

* runtime живёт на `RuntimePosition`
* density по запросу получает `DensityKey`
* sim по запросу получает `SimSectorKey`
* `FullRoute` нужен только там,
  где действительно требуется глубокая дискретная точность

То есть движение — это не “тащить весь deep-address каждый тик”,
а в первую очередь обновлять live-position,
а уже потом извлекать из неё нужную глубину адресации.

---

###  Базовая цепочка

Практически переадресация выглядит так:

```
WorldXYZ
-> RuntimePosition
-> DensityKey
-> SimSectorKey
-> FullRoute (on demand)
```

Это значит:

* мир сначала даёт непрерывную позицию;
* затем она привязывается к конкретному Region;
* дальше из неё по запросу получаются более дискретные формы.

---

### Движение внутри того же Chunk

Если объект остаётся внутри того же Chunk,
то обычно меняется только `LocalFloat`.

При этом:

* `RuntimePosition` обновляется постоянно;
* `DensityKey` может остаться тем же;
* `SimSectorKey` тоже может остаться тем же;
* `FullRoute` может вообще не вычисляться,
  если глубокой дискретной работы сейчас не требуется.

Это самый дешёвый сценарий:
объект движется,
но его coarse address forms остаются прежними.

---

### Переход в соседний Chunk

Если объект пересекает границу Chunk,
ситуация уже меняется.

Мы всё ещё начинаем с `RuntimePosition`,
но теперь из него получается новый `ChunkCoord`,
а значит обновляется и:

```text
DensityKey = Region + Chunk
```

При этом:

* `RegionCoord` может оставаться тем же;
* `SimSectorKey` может измениться или не измениться —
  зависит от того,
  пересёк ли объект и границу coarse sector.

То есть не каждое изменение Chunk
автоматически означает смену sim-sector.

---

### Переход в соседний Region

Когда объект пересекает границу Region,
обновляется уже не только локальная позиция,
но и сам `RegionCoord`.

Это означает:

* объект получает новый `Region`;
* `LocalFloat` пересобирается в диапазон нового Region;
* `DensityKey`, `SimSectorKey` и любые более глубокие формы
  уже вычисляются относительно нового Region.

Именно здесь хорошо видно,
почему machine-local внутри Region начинается от origin-угла региона:
при переходе через границу
локаль не “ломается”,
а просто заново считается в контейнере нового Region.

---

### Что переадресуется постоянно, а что по запросу

Практически удобно держать такое правило:

* **всегда живой слой** —
  `RuntimePosition`

* **часто обновляемый coarse layer** —
  `DensityKey`, `SimSectorKey`

* **глубокий слой по запросу** —
  `FullRoute`

Это особенно полезно,
чтобы не перегружать runtime ненужной deep-address логикой там,
где системе на самом деле нужна только coarse container info.

---

#### Такой подход даёт сразу несколько плюсов:

* runtime не тащит на себе лишнюю дискретную детализацию;
* density получает свой естественный ключ только тогда,
  когда объект реально пересекает границы density-контейнеров;
* sim получает свой coarse bucket независимо от deep density-address;
* deep tools и debug могут опускаться глубже
  только в тех местах,
  где это реально полезно.

Итоговая мысль:

```text
movement first updates live-position,
and only then derives the address depth that is needed
```

То есть движение не начинается с deep route,
а заканчивается им только там,
где это действительно нужно.

## 7. Copy / Paste and rotation

Copy / paste полезно понимать не как одну магическую операцию,
а как небольшой pipeline,
где address, content и orientation работают вместе,
но не смешиваются в одну сущность.

Практически это выглядит так:

1. выбрать source anchor;
2. определить local content bounds;
3. считать содержимое в локальной системе;
4. при необходимости применить rotation;
5. выбрать target anchor;
6. нормализовать target address;
7. записать содержимое в новую область.

---

### Copy / Paste без rotation

Самый простой случай:

```text
copy source:  R(...) / C#17
paste target: R(...) / C#4
orientation:  R0
```

Что здесь происходит:

* source anchor задаёт,
  из какого контейнера мы читаем локальное содержимое;
* target anchor задаёт,
  куда мы это содержимое пишем;
* адрес источника и адрес цели независимы друг от друга;
* ориентация содержимого остаётся той же,
  то есть rotation не применяется.

Такой режим полезен для:

* простого копирования фрагментов;
* повторного использования модулей;
* черновой сборки структур без изменения ориентации.

---

### Copy / Paste с rotation

В более интересном случае мы хотим вставить тот же локальный content,
но уже в другой ориентации.

Например:

```text
copy source:  R(...) / C#17
paste target: R(...) / C#4
orientation:  R90
mode:         content rotation
```

Здесь меняется не сам source address,
а локальная форма содержимого.

Pipeline становится таким:

* взять local content из source;
* повернуть его в local space;
* сохранить target anchor отдельным;
* записать rotated content в target container.

То есть мы крутим **содержимое**,
а не обязаны крутить сам address источника как якорь мира.

---

### Anchor rotation и content rotation — не одно и то же

Это один из главных practical-пунктов active MVP.

Нужно различать:

#### **Anchor rotation**

когда вращается сам anchor / позиция в мире.

Тогда:

* может измениться `RuntimePosition`;
* может измениться `DensityKey`;
* может измениться `SimSectorKey`.

#### **Content rotation**

когда anchor остаётся тем же,
но локальное содержимое внутри контейнера переориентируется.

Тогда:

* address якоря может остаться прежним;
* меняется только local content;
* orientation-state обновляется отдельно.

Это очень важно для:

* blueprint placement;
* copy/paste;
* preview tools;
* повторного использования локальных структур.

---

### Address и orientation не надо смешивать

При copy/paste особенно хорошо видно,
почему active MVP держит `Orientation` отдельно от address form.

Практически это означает:

```text id="kw2wck"
address != orientation
```

Один и тот же `DensityKey` может использоваться:

* с `R0`,
* с `R90`,
* с `R180`,
* с `R270`

если меняется только ориентация содержимого,
а не сам anchor.

Именно поэтому удобнее держать модель вида:

```text
AnchorAddress + Orientation + LocalContent
```

а не пытаться зашить всё в один перегруженный route-like object.

---

### Tool-команда и machine pipeline

Человеку и tool layer удобно работать более мягким языком.

Например:

```text "
copy C#17
paste to R(1|0|0) / C#4 with R90
```

Но под капотом это всё равно должно проходить через machine pipeline:

```text
human/debug input
-> parser / translator
-> canonical machine form
-> validate / normalize
-> execute
```

Это позволяет:

* держать человекочитаемый tool-layer;
* при этом не ломать machine truth;
* одинаково использовать copy/paste в editor, debug и генерации.

---

### Что это даёт на практике

Copy / paste в таком виде становится не “особой магией редактора”,
а естественным следствием уже существующей базы:

* `Routing` даёт address forms;
* `Rotation` даёт orientation и local transform rules;
* `Topology` даёт container-local math,
  по которой содержимое читается и раскладывается обратно.

Итоговая практическая формула здесь такая:

```text
copy/paste = address selection
           + local content extraction
           + optional rotation
           + normalized writeback
```

Именно в этом виде copy/paste уже хорошо ложится
и на editor-thinking,
и на generation-thinking,
и на future blueprint workflows.

---

## 8. Octochunk as first coarse filter for SVO

В active MVP `Octochunk` полезно понимать не только как внутренний подслой `Chunk`,
но и как первый практический шаг
перед более глубокой иерархической обработкой.

Если `Chunk = 64³`,
то он естественно делится на:

```text
8 Octochunk = 2 × 2 × 2
```

где каждый Octochunk имеет размер:

```text"
32³
```

Это даёт удобную coarse-ступень
между целым Chunk и более глубокой детализацией.

---

### Зачем это вообще нужно

Если смотреть на весь `Chunk 64³` сразу,
то алгоритм вынужден обрабатывать сразу весь объём,
даже если значимая структура есть только в его небольшой части.

Octochunk позволяет сначала сделать очень грубую классификацию,
а уже потом решать,
нужно ли углубляться дальше.

То есть вместо подхода:

```text
analyse whole Chunk as one heavy unit
```

мы получаем:

```text
Chunk
-> split into 8 Octochunk
-> classify each Octochunk
-> refine only where needed
```

---

### Базовая coarse-классификация

Для каждого Octochunk можно быстро получить summary-состояние.
Например:

* **empty** — пусто, можно не углубляться;
* **solid / uniform** — всё однородно, можно не углубляться;
* **mixed** — есть граница, поверхность, полость или неоднородность,
  значит нужно смотреть глубже.

Пример:

```text
Octo 0 = empty
Octo 1 = empty
Octo 2 = solid
Octo 3 = mixed
Octo 4 = solid
Octo 5 = mixed
Octo 6 = empty
Octo 7 = solid
```

После этого более тяжёлая обработка запускается
только для `mixed`-областей.

---

### Что это даёт на практике

Такой coarse-step полезен сразу в нескольких задачах.

#### **Для SVO / refinement**

Не нужно одинаково углубляться во все 64³.
Сначала Octochunk выступает как первый фильтр,
и только затем запускается более глубокий refinement.

#### **Для meshing**

Пустые и однородные Octochunk можно быстро отбрасывать
или обрабатывать упрощённо,
а `mixed` считать кандидатами на более детальную surface-работу.

#### **Для summary / masks**

На уровне Octochunk удобно держать:

* occupancy,
* rough density summary,
* uniform / mixed flag,
* признаки поверхности,
* coarse material hints.

#### **Для future SVO-style processing**

Octochunk может быть первым и очень дешёвым “воротами”,
через которые Chunk проходит
перед более глубокой иерархической обработкой.

---

### Почему это не равно самому SVO

Важно:
Octochunk summary — это ещё не сам SVO.

Это скорее первый coarse buffer / refinement gate,
который помогает решить:

* надо ли вообще идти глубже;
* куда именно углубляться;
* какие части Chunk можно пропустить сразу.

То есть:

```text
dense chunk data
-> octochunk summary
-> deeper refinement only where needed
```

Именно поэтому Octochunk здесь полезен не как абстрактная “красивая прослойка”,
а как реально операционный уровень.

---

### Практическая роль Octochunk

Если собрать это в одну короткую мысль,
то для Examples удобно зафиксировать так:

```text id="99n80w"
Octochunk = first coarse filter
Octochunk = first refinement gate
Octochunk = first summary domain inside Chunk
```

Это одна из самых сильных practical-ролей Octochunk в active MVP:
не просто структурный подслой,
а первый рабочий слой перед более глубокой SVO-style логикой.

## 9. Mapping, flat index, Morton

Когда мы говорим об address forms и локальных координатах,
полезно помнить,
что одна и та же 3D-структура может быть сведена
к разным индексным представлениям
в зависимости от задачи.

В Examples нам важны три связанные идеи:

* обычный mapping через container-local coordinates;
* flat index для линейной памяти;
* Morton для spatial-locality.

---

### Mapping: от world к локальной структуре

Базовая логика active MVP выглядит так:

```text
WorldXYZ
-> RegionCoord + LocalFloat
-> ChunkCoord
-> OctochunkCoord
-> VoxelCoord
```

Это и есть основной machine mapping.

Он отвечает на вопрос:
как непрерывная world-позиция
раскладывается в дискретные контейнеры и локальные индексы.

Практически это значит,
что одна и та же позиция может быть прочитана на разной глубине:

* только как `RuntimePosition`,
* как `DensityKey`,
* как `SimSectorKey`,
* как `FullRoute`.

---

### Flat index: когда нужен линейный порядок

Если данные уровня нужно уложить в обычный линейный массив,
используется flat index.

Для canonical XYZ-порядка:

```text
flat_index =
    x +
    size_x * (
        y +
        size_y * z
    )
```

Эта формула полезна там,
где данные живут в обычной плотной памяти.

Например:

* `ChunkCoord -> C#n`
* `OctochunkCoord -> O#n`
* `SectorCoord -> S#n`

То есть compact ids удобно строить именно на этой индексной логике,
если нужно короткое и стабильное helper-представление.

---

### Что flat index даёт practically

Flat index нужен,
когда системе важен не spatial-tree,
а обычный линейный layout:

* плотный буфер;
* массив чанков/окто/секторов;
* helper ids в логах и HUD;
* compact indexing для tools.

Это не “адрес мира” сам по себе,
а просто способ уложить локальную 3D-структуру в 1D-порядок.

---

### Morton: когда важна spatial locality

Morton нужен в другой ситуации:
когда важно,
чтобы близкие в пространстве элементы
оставались относительно близкими и в индексном порядке.

Идея грубо выглядит так:

```text
(x, y, z) -> interleave_bits(x, y, z)
```

В отличие от flat index,
Morton не столько про простой линейный массив,
сколько про spatial-locality,
иерархические структуры
и tree-like обход.

---

### Flat и Morton не конкурируют

Очень полезно не путать эти две модели.

```text
flat   = linear memory order
Morton = spatial-local order
```

То есть:

* flat удобен для плотного storage и простых helper ids;
* Morton удобен для spatial trees,
  иерархических структур
  и сценариев,
  где важна локальность в пространстве.

Они не заменяют друг друга,
а отвечают на разные вопросы.

---

### Где это видно в Examples

Для читателя Examples это полезно так:

* `mapping` объясняет,
  как позиция вообще попадает в нужный контейнер;

* `flat index` объясняет,
  откуда берутся удобные `S# / C# / O#`;

* `Morton` объясняет,
  почему более spatial-aware структуры
  не обязаны использовать тот же порядок,
  что и обычный плотный буфер.

---

### Practical summary

Если собрать всё в короткую практическую формулу:

```text
mapping -> находит нужный контейнер и локальные координаты
flat    -> даёт линейный helper/index order
Morton  -> даёт spatial-local order
```

Так становится проще понимать,
почему одна и та же 3D-структура
может иметь сразу несколько представлений,
и почему это не конфликт,
а нормальная часть active MVP.

---

## 10. Conceptual buffers

В Examples буфер лучше понимать не как жёсткий backend layout,
а как удобную ментальную карту:
какой слой какие данные в целом держит
и зачем они ему нужны.

Это важно,
чтобы не смешивать live-position,
density-data,
sim buckets,
tool staging
и helper indexing
в один перегруженный “супер-буфер”.

---

### Runtime buffer / registry

Runtime-слой в первую очередь хранит живое состояние объекта.

Здесь естественно живут:

* `RuntimePosition = Region + LocalFloat`
* текущий live-state
* orientation-state
* временные runtime-поля
  вроде velocity, flags, timers и другой оперативной логики

Это буфер не про глубокую дискретную адресацию,
а про то,
что сейчас реально живёт, двигается и обновляется в рантайме.

---

### Density buffer / chunk storage

Density-слой мыслится уже не через live-position,
а через контейнер плотности.

Здесь естественно живут:

* `DensityKey = Region + Chunk`
* chunk-local voxel payload
* material / density data
* локальные данные,
  нужные для generation, meshing, compression и похожих задач

Это основной буфер про сам Chunk как плотный контейнер,
а не про coarse sim или инструментальный debug.

---

### Octochunk summary buffer

Если смотреть на Chunk не как на одну тяжёлую глыбу,
а как на контейнер с первым coarse-step внутри,
то полезно отдельно мыслить summary-слой по Octochunk.

Для каждого из 8 Octochunk можно хранить очень грубую сводку, например:

* `empty`
* `solid / uniform`
* `mixed`

Или другие summary-признаки:

* occupancy
* rough density summary
* surface hint
* material hint
* local activity flag

Это не обязано быть самим SVO.
Такой буфер можно понимать как:

```text
dense chunk data
-> octochunk summary
-> deeper refinement only where needed
```

То есть Octochunk summary-buffer — это первый coarse filter
и первый refinement gate внутри Chunk.

---

### Sim buckets

Sim-слой удобнее мыслить отдельно от density-storage.

Здесь естественно живут:

* `SimSectorKey = Region + SectorCoord`
* coarse activity grouping
* awake / sleep flags
* orchestration queues
* broad-phase sim grouping

Этот буфер отвечает не на вопрос
“какой voxel здесь лежит”,
а на вопрос
“в каком coarse sim bucket сейчас находится объект или область”.

---

### Tool / staging buffer

Отдельно полезно мыслить временный слой для инструментов.

Здесь могут жить:

* selection bounds
* copy/paste payload
* rotated preview
* transformed local content
* import/export staging data
* временные tool-команды перед writeback

Это особенно удобно для editor-thinking:

```text id="4mxr2r"
source content
-> staging buffer
-> optional transform / rotation
-> target writeback
```

Так tool layer не обязан сразу писать всё напрямую в основной density-buffer,
а может сначала собрать, проверить и преобразовать данные в staging-слое.

---

### Helper / index cache

Некоторые формы данных полезно держать не как source of truth,
а как helper-кеш.

Сюда могут относиться:

* compact ids (`S#`, `C#`, `O#`)
* flat indices
* Morton helpers
* masks
* lookup tables
* derived preview/meta values

Это не новый канон адресации
и не замена machine-form,
а просто ускоряющий и упрощающий слой там,
где он реально нужен.

---

### Как всё это читать вместе

Если собрать это в одну практическую картину,
то получается такая ментальная схема:

* **runtime buffer**
  хранит live-position и текущее состояние

* **density buffer**
  хранит Chunk и его локальное содержимое

* **octochunk summary buffer**
  хранит coarse классификацию внутри Chunk

* **sim buckets**
  группируют объекты и области по coarse sector-слою

* **tool/staging buffer**
  держит временные данные для selection, copy/paste и transform

* **helper/index cache**
  держит производные compact/index forms

---

### Важная оговорка

Этот раздел не задаёт storage spec
и не фиксирует обязательный backend layout.

Это только conceptual view,
который помогает понимать:

* почему разные подсистемы не обязаны жить в одном формате;
* почему одни данные удобнее хранить как live-position,
  а другие как density/sim/address summary;
* как active MVP раскладывает данные по ролям,
  а не сваливает всё в одну структуру.

Итоговая мысль здесь простая:

```text 
different layers keep different kinds of truth
```

Именно это делает систему чище,
понятнее для tools
и удобнее для дальнейшего роста.

---

[📚 Вернуться назад](./README.md)

[🧱 Архитектурный портал проекта](../../ARCHITECTURE/readme.md)  

