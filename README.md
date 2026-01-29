<!-- LOGO -->
<p align="center">
  <img width="200" alt="membrane" src="https://github.com/user-attachments/assets/d9b8e51e-1cbb-4e89-8d91-975c64fa5aea" />
</p>

<h1 align="center">Membrane</h1>

<p align="center">
  <a href="https://crates.io/crates/membrane-cli">
    <img src="https://img.shields.io/crates/v/mem.svg" />
  </a>
</p>

---

**Membrane** is a schema-optional, file-based project memory system.

It lets you name projects, attach arbitrary keys, and observe how language
emerges across your work — without enforcing structure, schemas, or workflows.

Membrane is not a task manager, calendar, or ticketing system.
It sits *above* those tools, acting as a lightweight semantic layer
over your projects.

---

## Why Membrane?

Most project tools fail in one of two ways:

- They are **too rigid** (fixed fields, schemas, workflows)
- Or **too vague** (notes with no structure at all)

Membrane takes a different approach:

- Projects are files
- Keys are language artifacts
- Structure is allowed to emerge organically
- Metadata exists, but stays out of the way

You decide what matters. Membrane just remembers it.

---

## Core Concepts

- **Workspace-based**  
  A Membrane workspace lives in a `.membrane/` directory.

- **Projects are YAML files**  
  Each project is a single, human-readable YAML file.

- **Schema-less by design**  
  Projects can contain any keys. No required fields.

- **Keys are first-class**  
  Membrane can analyze keys across projects to reveal patterns,
  inconsistencies, or duplication.

- **Metadata is explicit**  
  Reserved keys (prefixed with `_`) track creation and update times
  without polluting user-defined structure.

---

## Installation

```bash
cargo install mem
```

Or clone and build locally:

```bash
git clone https://github.com/yourname/membrane
cd membrane
cargo build --release
```

---

## Getting Started

### Initialize a workspace

```bash
mem init
```

Creates a `.membrane/` directory in the current folder.

---

### Create a project

```bash
mem add my-project
```

Creates a new project file with basic metadata.

---

### List projects

```bash
mem show
```

Displays all projects in the current workspace.

---

### Show a project

```bash
mem show my-project
```

Displays all keys and values for the project.
Metadata keys (prefixed with `_`) are visually dimmed.

---

### Set a key

```bash
mem set my-project description "Initial prototype"
```

Values are parsed as YAML scalars when possible.

---

### Set a multi-line value (interactive)

```bash
mem set my-project notes
```

You’ll be prompted to paste or type content directly into the terminal.
Finish with `Ctrl+D` (Linux/macOS) or `Ctrl+Z` + Enter (Windows).

---

### Inspect keys across projects

```bash
mem keys
```

Lists all keys and how frequently they appear.

```bash
mem keys --similar
```

Highlights near-duplicate keys (e.g. `created_at` vs `created-at`).

---

### Delete a project (safe)

```bash
mem rm my-project
```

You’ll be asked to confirm by typing the project name.

---

## Philosophy

Membrane is intentionally minimal.

It does **not**:

* enforce schemas
* impose workflows
* sync to the cloud
* track tasks or deadlines
* automate decisions

Instead, it provides a durable, inspectable memory layer
that you can reason about over time.

---

## Versioning

Membrane follows semantic versioning.

* **0.1.0** establishes the conceptual and architectural core.
* Future versions will add capabilities incrementally
  without breaking existing project files.

---

## Roadmap (non-binding)

Possible future additions:

* Key renaming / edit-sweep
* Key deletion
* Archiving projects
* Filters and views
* Editor integration
* JSON output modes

None of these are required to use Membrane effectively.

---

## License

MIT
