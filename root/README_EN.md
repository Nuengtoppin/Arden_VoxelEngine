# **Extended README**

🌐 **Documentation and Roadmaps:**  
[🇬🇧 English Roadmap (current version)](../docs/EN/roadmap.md)  
[🇷🇺 Russian Roadmap](../docs/RU/roadmap.md)

---

````md
# Arden Engine — Extended Description

## What it is

**Arden Engine** is an open R&D project built with **Rust + Bevy** for exploring the architecture of a hybrid voxel / mesh engine.

The project is not a finished game engine yet. It is an experimental codebase where I gradually build the core, test ideas in lab scenes, and document technical decisions along the way.

---

Status: **early experimental development**.

The current code can be launched and tested, but it should not be treated as a stable API, editor, or production-ready engine.  
Some decisions are temporary, some modules will be rewritten, and some old MVP traces are kept as historical context.

---

## Why this project exists

Arden is my practical lab for studying engine architecture.  
Not just reading how things work, but building live code, running into structural mistakes, seeing temporary workarounds, and gradually extracting a working system from that process.

The main interest of the project is the idea of a hybrid world: voxel data, mesh representation, spatial truth, debug tools, and future runtime layers should not exist as isolated parts, but as pieces of one understandable architecture.

Documentation is part of the development process here. It is not just a showcase; it keeps track of what already works, what is still raw, why a decision appeared, and what is expected to change later.

---

## Short philosophy

Arden should not be a “black box”.

Core principles:

- **observability** — internal states should be visible;
- **modularity** — each layer should have a clear responsibility;
- **verifiability** — architecture should be confirmed in a live scene;
- **honest state tracking** — temporary solutions, raw code, and future rewrites are documented instead of hidden.

---

> 📘 Terms and definitions are available in the [**Glossary**](../docs/EN/TERMS/Glossary.md).  
> It is useful to read it before going deeper into the project structure.

> 📖 For the broader project concept, see [**Concept**](../docs/EN/CONCEPT/readme.md).  
> It contains the general ideas behind the engine without going too deep into implementation details.

---

## Quick Start

From the repository root:

```bash
cargo run
````

## What you can try now

The current lab scene allows you to test:

* fly camera;
* debug HUD;
* machine / human notation;
* spatial probe;
* pinned target;
* layered gizmos;
* finite voxel sandbox;
* Paint / Erase;
* SelectBox;
* Fill / Delete volume;
* Clipboard copy/paste;
* save/load lab snapshot.

## Controls

### Camera

| Input                 | Action                         |
| --------------------- | ------------------------------ |
| `W / A / S / D`       | move camera                    |
| `Space`               | move up                        |
| `Left Ctrl`           | move down                      |
| `Shift`               | faster movement                |
| `Right Mouse`         | rotate camera                  |
| `Shift + Mouse Wheel` | move camera forward / backward |

### Debug / HUD

| Input               | Action                          |
| ------------------- | ------------------------------- |
| `F1`                | toggle HUD                      |
| `F2`                | toggle gizmos                   |
| `PageUp / PageDown` | switch debug lens               |
| `F3`                | switch machine / human notation |
| `F4`                | switch compact / detailed view  |
| `Q`                 | pin current inspect target      |
| `E`                 | clear pinned target             |

### Lab Tools

| Input        | Action                 |
| ------------ | ---------------------- |
| `1`          | Inspect                |
| `2`          | SelectBox              |
| `3`          | Paint                  |
| `4`          | Erase                  |
| `Left Mouse` | apply current tool     |
| `F`          | fill selected volume   |
| `Delete`     | delete selected volume |
| `C`          | copy selected volume   |
| `V`          | paste clipboard volume |
| `F5`         | save lab snapshot      |
| `F9`         | load lab snapshot      |

> Editing tools work in **Edit mode**.
> The mode can be switched through the HUD.

---

## 🤝 **Contribution Guide**

Thank you for your interest in **Arden Engine**!  
Your participation matters — you can contribute through code, documentation, ideas, or research.

---

### 🧭 **Principles of Contribution**

- Maintain a respectful tone (see [Code of Conduct](./CODE_OF_CONDUCT.md)).  
- Discuss major ideas in **Issues** or via email before implementation:  
  📧 **arden.engine@proton.me**
- All changes are introduced via **Pull Requests**, with a short explanation of purpose and content.

---

### 🧩 **Ways to Contribute**

You can help the project by:
- fixing bugs or improving the code (`/src`);  
- updating or expanding documentation (`/docs`);  
- refining conceptual drafts;  
- proposing architectural ideas;  
- testing performance or reporting issues;  
- creating visual assets, diagrams, or design documentation.

Every contribution — technical, artistic, or conceptual —  
will be acknowledged in the repository.

To add yourself to the contributors list:

 1. **Fork** the repository on GitHub.  
 2. Add your name and role to the table in the main README.  
 3. Open a **Pull Request** — I will review and merge it into the main branch.

---

## 📮 **Contact**

If you would like to discuss collaboration, research, feedback, or R&D ideas —  
you can contact the author here:

📧 **arden.engine@proton.me**

Feel free to reach out about:
- architecture and internal systems;  
- research proposals and experiments;  
- documentation or design contributions;  
- module development, testing, or optimization;  
- discussions and community initiatives.

**Arden Engine** is an open research project —  
thoughtful and constructive communication is always welcome.

---

## 🙏 **Acknowledgments and Wishes**

Thank you to everyone showing interest in **Arden Engine** —  
reading the docs, testing ideas, sharing thoughts, and providing feedback.  
Your input — whether it’s a line of code, a documentation fix, or an observation —  
helps keep the project alive and evolving.

This engine is built not around a single author, but around an idea:  
that open architecture can unite people, disciplines, and schools of thought.  
Anyone who helps clarify, refine, or even ask the right question  
becomes part of **Arden**.

> May the project grow openly, with patience and without losing its purpose.  
> May it inspire the creation of not just tools,  
> but systems where ideas and engineering remain in balance.

---

## 🜂 **Licenses**

The project is distributed under a combination of open licenses:

- **MIT License** — allows free use, copying, and modification of code with author attribution.  
- **Apache License 2.0** — provides additional legal protection for copyrights and patents.  
- **CC-BY 4.0** — applies to documentation, media, and written materials;  
  allows adaptation with proper credit to the author.

**Arden Engine** remains an open R&D project:  
its code, documentation, and assets may be freely used  
for research, education, and development — with proper attribution to the original source.
