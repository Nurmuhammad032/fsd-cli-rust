# fsd-cli

A command-line tool for scaffolding [Feature-Sliced Design (FSD)](https://feature-sliced.design/) folder structures.

## Installation

```bash
cargo install --path .
```

Or run directly with cargo:

```bash
cargo run -- <command>
```

## Commands

### `fsd init`

Initialize a complete FSD folder structure in your project.

**Usage:**

```bash
fsd init [OPTIONS]
```

**Options:**

- `--root <PATH>` - Project root directory (defaults to current directory)
- `--base <PATH>` - Base directory where FSD lives (defaults to `src`)
- `--style <STYLE>` - Template style: `minimal` or `full` (default: `minimal`)
  - `minimal`: Creates only the layer folders (app, pages, widgets, features, entities, shared)
  - `full`: Creates layer folders plus common structure in shared and app layers
- `--processes` - Include the optional "processes" layer
- `--dry-run` - Preview what would be created without actually creating anything
- `--force` - Continue even if directories already exist

**Examples:**

```bash
# Initialize with minimal structure in src/
fsd init

# Initialize with full structure including processes layer
fsd init --style full --processes

# Preview what would be created
fsd init --dry-run

# Initialize in a different base directory
fsd init --base frontend/src
```

**Created structure (minimal):**

```
src/
├── app/
├── pages/
├── widgets/
├── features/
├── entities/
└── shared/
```

**Created structure (full):**

```
src/
├── app/
│   ├── providers/
│   ├── router/
│   └── styles/
├── pages/
├── widgets/
├── features/
├── entities/
└── shared/
    ├── ui/
    ├── lib/
    ├── api/
    ├── config/
    └── assets/
```

---

### `fsd add`

Add one or more slices to a specific layer with an `index.ts` file for public exports.

**Usage:**

```bash
fsd add [OPTIONS] <LAYER> <SLICE>...
```

**Arguments:**

- `<LAYER>` - FSD layer: `app` (or `a`), `processes` (or `pr`), `pages` (or `p`), `widgets` (or `w`), `features` (or `f`), `entities` (or `e`), or `shared` (or `s`)
- `<SLICE>...` - One or more slice names (e.g., `auth`, `user`, `profile`) - supports multiple slices

**Options:**

- `--root <PATH>` - Project root directory (defaults to current directory)
- `--base <PATH>` - Base directory where FSD lives (defaults to `src`)
- `-s, --segments <SEGMENTS>` - Create segments inside the slice (comma-separated)
  - By default, no segments are created (just the slice folder + `index.ts`)
  - Use `-s default` or `--segments default` for standard segments: `api,model,ui,lib,config`
  - Use `-s <list>` for custom segments (e.g., `-s ui,model,api`)
  - Use `-s none` to explicitly create no segments
  - Available segments: `ui`, `model`, `api`, `lib`, `config`, `assets`
- `--dry-run` - Preview what would be created without actually creating anything
- `--force` - Continue even if directories already exist

**Examples:**

```bash
# Add a single slice (creates pages/ui/index.ts)
fsd add pages ui
# OR use short alias:
fsd add p ui

# Add multiple slices at once
fsd add p ui model
# Creates:
#   - pages/ui/index.ts
#   - pages/model/index.ts

fsd add f auth theme-switcher user-profile
# Creates:
#   - features/auth/index.ts
#   - features/theme-switcher/index.ts
#   - features/user-profile/index.ts

# Add a page with ui segment (auto-creates index.ts + page.tsx in ui/)
fsd add p home -s ui
# Result:
# src/pages/home/
#   ├── index.ts
#   └── ui/
#       ├── index.ts
#       └── page.tsx

# Add a feature with all standard segments
fsd add f auth -s default
# Result:
# src/features/auth/
#   ├── index.ts
#   ├── api/
#   ├── model/
#   ├── ui/
#   │   ├── index.ts
#   │   └── page.tsx
#   ├── lib/
#   └── config/

# Add multiple entities with ui and model segments (using short aliases!)
fsd add e user product order -s ui,model

# Preview what would be created
fsd add w header footer --dry-run

# Add to a different base directory
fsd add f theme --base frontend/src
```

**Default behavior:**

- Creates the slice folder
- Adds an `index.ts` file with `export {};` for public API
- No segments by default (use `-s` or `--segments` to add them)
- When creating a `ui` segment, automatically adds both `index.ts` and `page.tsx` files

**Layer Shortcuts:**

Save time by using short aliases for layers:

- `a` → `app`
- `pr` → `processes`
- `p` → `pages`
- `w` → `widgets`
- `f` → `features`
- `e` → `entities`
- `s` → `shared`

---

## FSD Layer Hierarchy

From top to bottom:

1. **app** - Application-wide settings, styles, providers, and router
2. **processes** (optional) - Complex inter-page scenarios
3. **pages** - Full pages of your application
4. **widgets** - Large compositional UI blocks
5. **features** - User interactions and business features
6. **entities** - Business entities (user, product, order, etc.)
7. **shared** - Reusable infrastructure code (UI kit, libs, API, configs)

## Common Workflows

### Starting a new project

```bash
# Initialize FSD structure
fsd init --style full

# Add your first pages (multiple at once using short alias!)
fsd add p home about contact -s ui

# Add some features with ui
fsd add f auth theme-switcher user-settings -s ui,model,api

# Add entities with segments
fsd add e user product order -s ui,model,api
```

### Adding a new feature

```bash
# Simple feature (just the folder + index.ts)
fsd add f search

# Feature with ui segment (gets index.ts + page.tsx automatically)
fsd add f shopping-cart -s ui

# Full-featured slice with all segments
fsd add f checkout -s default
```

### Working with shared layer

```bash
# Add shared utilities (using short alias 's')
fsd add s utils
fsd add s hooks
fsd add s types

# Add shared ui components with ui segment
fsd add s button input card -s ui
```

## Tips

- Use `--dry-run` to preview changes before creating them
- Slice names must not contain path separators (`/`, `\`)
- The tool creates only known segments to prevent accidental filesystem writes
- Use `--force` if you're adding to an existing structure

## Performance Benchmark

`rfsd` is built with Rust for maximum performance. Here's how it compares to the official Node.js-based FSD CLI:

| Test                  | Official fsd | rfsd (Rust) | **Speedup**        |
| --------------------- | ------------ | ----------- | ------------------ |
| Simple page creation  | 211.1 ms     | 2.6 ms      | **🚀 82x faster**  |
| Page with segments    | 280.8 ms     | 2.6 ms      | **🚀 107x faster** |
| Multiple pages        | 208.9 ms     | 2.3 ms      | **🚀 90x faster**  |
| Feature with segments | 283.6 ms     | 3.4 ms      | **🚀 85x faster**  |
| Startup time (--help) | 49.8 ms      | 1.8 ms      | **🚀 28x faster**  |

**Why so fast?**

- ⚡ Compiled to native code (no JavaScript runtime overhead)
- 🚀 Single binary (no module loading)
- 💨 Direct system calls via Rust's `std::fs`
- 🎯 Zero dependencies at runtime

Run the benchmark yourself:

```bash
./benchmark.sh
```

_Benchmarks performed using [hyperfine](https://github.com/sharkdp/hyperfine) on macOS._

## License

MIT
