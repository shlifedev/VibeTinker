# Agent Guidelines for New App

This document provides coding guidelines and conventions for AI agents working on this Tauri project.

## Project Overview

A Tauri-based desktop application with:
- **Frontend**: SvelteKit 2.x with TypeScript and Svelte 5
- **Backend**: Rust with Tauri 2.0, using tauri-specta for type-safe bindings
- **Architecture**: SPA mode with file-based routing

## Build, Lint, and Test Commands

### Frontend (SvelteKit)
```bash
bun run dev
bun run build
bun run preview
bun run check
bun run check:watch
```

### Backend (Rust/Tauri)
```bash
bun run tauri dev
bun run tauri build

cd src-tauri && cargo test
cd src-tauri && cargo test test_name
cd src-tauri && cargo check
cd src-tauri && cargo fmt
cd src-tauri && cargo clippy
```

### Generate TypeScript Bindings
TypeScript bindings are automatically generated when running the Tauri app. The bindings are exported to `src/lib/bindings.ts` via tauri-specta.

## Project Structure

```
/src                          # Frontend (SvelteKit)
├── routes/                   # File-based routing
│   ├── +page.svelte         # Route pages (+ prefix = special SvelteKit file)
│   └── +layout.svelte       # Layout components
├── lib/
│   └── bindings.ts          # Auto-generated TypeScript bindings from Rust

/src-tauri                    # Backend (Rust)
├── src/
│   ├── lib.rs               # Library entry point, app initialization
│   ├── main.rs              # Binary entry point
│   ├── command.rs           # Tauri commands
│   └── modules/
│       ├── types.rs         # Shared types and events
│       └── logger.rs        # Logging utility
└── Cargo.toml               # Rust dependencies
```

## Code Style Guidelines

### TypeScript/Svelte

#### Imports
- Import Tauri APIs from `@tauri-apps/api/*`
- Import auto-generated bindings from `$lib/bindings`
- Use SvelteKit path aliases: `$lib` for `/src/lib`
- Group imports: external packages -> Svelte/Tauri -> local imports
```typescript
import { invoke } from "@tauri-apps/api/core"
import { onMount } from "svelte"
import { events, commands } from "$lib/bindings"
```

#### Naming Conventions
- **Files**: 
  - SvelteKit special files: `+page.svelte`, `+layout.svelte`, `+page.ts`
  - Regular files: `page.css`, `utils.ts`, `Button.svelte` (no `+` prefix)
- **Variables**: camelCase
- **Types**: PascalCase
- **Constants**: UPPER_SNAKE_CASE for true constants

#### TypeScript
- Use strict mode (enabled in `tsconfig.json`)
- Prefer `type` over `interface` for simple object types
- Use Svelte 5 runes: `$state`, `$derived`, `$effect` instead of old reactive syntax
```typescript
let count = $state(0)
let doubled = $derived(count * 2)
```

#### Formatting
- No semicolons (following project convention)
- Use double quotes for strings
- 2 spaces for indentation

### Rust

#### Tauri Commands
All commands must have both annotations:
```rust
#[tauri::command]
#[specta::specta]
fn command_name(state: State<'_, Mutex<AppState>>) -> ReturnType {
    // implementation
}
```

After creating a command:
1. Add it to `lib.rs` in the `collect_commands![]` macro
2. Run the app to regenerate TypeScript bindings

#### Tauri Events
Define events in `modules/types.rs`:
```rust
#[derive(Clone, Type, Event)]
pub struct EventName {
    pub field: String,
}
```

Register in `lib.rs` using `collect_events![]` macro.

#### Types
- Use `serde` attributes for JSON serialization
- Use `#[serde(rename = "camelCase")]` for camelCase in TypeScript
- All shared types must derive: `Serialize, Deserialize, Type, Clone`

#### State Management
- App state is managed through `AppState` struct in `lib.rs`
- State is wrapped in `Mutex` for thread safety
- Access state via `State<'_, Mutex<AppState>>` in commands

#### Naming Conventions
- **Files**: snake_case (`command.rs`, `types.rs`)
- **Functions**: snake_case (`greet`, `add_item`)
- **Types**: PascalCase (`AppState`)
- **Module names**: snake_case (`modules::types`)

#### Error Handling
- Use `Result<T, E>` for operations that can fail
- Prefer `.unwrap()` only for state locks or in development
- Use proper error types for production code

#### Formatting
- Run `cargo fmt` before committing
- Follow Rust standard style guide
- Use 4 spaces for indentation

## Important Notes

1. **TypeScript bindings** (`src/lib/bindings.ts`) are auto-generated - never edit manually
2. **File naming**: Only SvelteKit routing files use `+` prefix
3. **State management**: Single `AppState` for simplicity
4. **Svelte 5 syntax**: Use runes (`$state`, `$derived`) instead of `let` + reactive statements
5. **Always run type checking** before committing: `bun run check`

## Common Tasks

### Adding a New Tauri Command
1. Define function in `src-tauri/src/command.rs` with annotations
2. Add to `collect_commands![]` in `lib.rs`
3. Run app to generate TypeScript bindings
4. Use from frontend via `commands.yourCommand()`

### Adding a New Route
1. Create `src/routes/your-route/+page.svelte`
2. Route will be available at `/your-route`

### Working with State
1. Add fields to `AppState` struct in `lib.rs`
2. Update state initialization in `.manage()` call
3. Access in commands via `State<'_, Mutex<AppState>>`
