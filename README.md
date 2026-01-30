<!-- LOGO -->
<p align="center">
  <img width="200" alt="membrane" src="https://github.com/user-attachments/assets/d9b8e51e-1cbb-4e89-8d91-975c64fa5aea" />
</p>

<h1 align="center">Membrane</h1>

<p align="center">
  <a href="https://crates.io/crates/membrane-cli">
    <img src="https://img.shields.io/crates/v/membrane-cli.svg" />
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
  Keys can be added, renamed, deleted, and inspected across projects.

- **Metadata is explicit**  
  Reserved keys (prefixed with `_`) track creation and update times
  without polluting user-defined structure.

---

## Installation

```bash
cargo install membrane-cli
```

This installs the `me` command.

---

## Getting Started

### Initialize a workspace

```bash
me init
```

Creates a `.membrane/` directory in the current folder.

---

### Create a project

```bash
me add my-project
```

Creates a new project file with basic metadata.

---

### List projects

```bash
me show
```

Displays all projects in the current workspace.

---

### Show a project

```bash
me show my-project
```

Displays all keys and values for the project.
Metadata keys (prefixed with `_`) are visually dimmed.

---

### Sort projects by a key

```bash
me show --sort status
me show --sort _updated --desc
```

When sorting is enabled, the selected key and its value
are shown inline for each project (when present).

---

### Set a key

```bash
me set my-project description "Initial prototype"
```

Values are parsed as YAML scalars when possible.

---

### Set a multi-line value (interactive)

```bash
me set my-project notes
```

You’ll be prompted to paste or type content directly into the terminal.
Finish with `Ctrl+D` (Linux/macOS) or `Ctrl+Z` + Enter (Windows).

---

### Push a project from YAML

```bash
me push roadmap.yaml
```

Or interactively:

```bash
me push
```

Paste YAML content directly into the terminal to create a project.

---

### Inspect keys across projects

```bash
me keys
```

Lists all keys and how frequently they appear.

```bash
me keys --similar
```

Highlights near-duplicate keys (e.g. `created_at` vs `created-at`).

---

### Rename a key

```bash
me keys rename old_key new_key
me keys rename status phase --project my-project
```

---

### Delete a project (safe)

```bash
me rm my-project
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

* **0.1.x** establishes the conceptual and architectural core.
* **0.2.x** adds inspection, mutation, and sorting capabilities.
* Future versions will extend views and filters
  without breaking existing project files.

---

## License

MIT