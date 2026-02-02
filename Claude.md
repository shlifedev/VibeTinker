# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Tauri 2.0 desktop application with SvelteKit frontend and Rust backend. A collection of developer tools with a sidebar navigation UI.

## Principles

1. **Lint before commit**: Always run `cargo fmt && cargo clippy` before committing
2. **Type-safe errors**: Use `thiserror` for custom error types in Rust
3. **Keep CLAUDE.md updated**: Update this file when adding new features or patterns
4. **KISS**: Keep code simple, avoid unnecessary abstraction

## Build Commands

```bash
# Development
bun run tauri dev          # Run app in dev mode

# Frontend only
bun run dev                # Vite dev server
bun run check              # TypeScript type check

# Backend only
cd src-tauri
cargo check                # Check compilation
cargo fmt                  # Format code
cargo clippy               # Lint code

# Production
bun run tauri build        # Build release
```

## Project Structure

```
/src                          # Frontend (SvelteKit)
├── routes/
│   ├── tools/
│   │   ├── encoding/
│   │   │   └── base64/              # Base64 encoder/decoder
│   │   ├── llm-toolset/
│   │   │   └── motion-descriptor/   # Mouse motion to LLM prompt
│   │   └── gamedev/
│   │       ├── sprite-sheet-describer/ # Sprite sheet grid annotator
│   │       └── tween-visualizer/       # Easing function visualizer
│   ├── +page.svelte         # Pages (+ prefix = SvelteKit special file)
│   └── +layout.svelte       # Layouts
├── lib/
│   ├── bindings.ts          # Auto-generated (DO NOT EDIT)
│   ├── components/
│   │   └── Sidebar.svelte   # Navigation sidebar
│   └── config/
│       └── tools.ts         # Tool categories and metadata

/src-tauri                    # Backend (Rust)
├── src/
│   ├── lib.rs               # App initialization, state management
│   ├── main.rs              # Entry point
│   ├── command.rs           # Tauri commands registry
│   ├── modules/
│   │   ├── types.rs         # Shared types and events
│   │   └── logger.rs        # Logging utility
│   └── tools/               # Tool implementations
│       ├── encoding/        # Base64 commands
│       └── llm_toolset/     # Motion descriptor commands
└── Cargo.toml
```

## Tool Categories

Tools are organized into categories defined in `src/lib/config/tools.ts`:

| Category | ID | Tools |
|----------|----|----- |
| 인코딩 | `encoding` | Base64 |
| LLM 도구 | `llm-toolset` | Motion Descriptor |
| 게임 개발 | `gamedev` | Sprite Sheet Describer, Tween Visualizer |

Adding a new tool category:
1. Add to `toolCategories` array in `tools.ts`
2. Create route directory: `src/routes/tools/{category}/{tool-id}/+page.svelte`
3. Implement backend commands in `src-tauri/src/tools/{category}/`

## TypeScript/Svelte Guidelines

### Imports
```typescript
import { invoke } from "@tauri-apps/api/core"
import { onMount } from "svelte"
import { commands } from "$lib/bindings"
```

### Svelte 5 Runes
```typescript
let count = $state(0)
let doubled = $derived(count * 2)
```

### Formatting
- No semicolons
- Double quotes for strings
- 2 spaces indentation

## Rust Guidelines

### Tauri Commands
```rust
#[tauri::command]
#[specta::specta]
pub fn my_command(state: State<'_, Mutex<AppState>>) -> Result<String, AppError> {
    // implementation
}
```

After creating a command:
1. Add to `collect_commands![]` in `lib.rs`
2. Run app to regenerate TypeScript bindings

### Error Handling
Use thiserror for typed errors:
```rust
#[derive(Debug, thiserror::Error, specta::Type, serde::Serialize)]
pub enum AppError {
    #[error("File error: {0}")]
    FileError(String),
    #[error("{0}")]
    Custom(String),
}
```

### Types for Frontend
```rust
#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct MyData {
    pub id: u32,
    #[serde(rename = "userName")]
    pub user_name: String,
}
```

### Tauri Events
```rust
#[derive(Clone, specta::Type, tauri_specta::Event, Serialize)]
pub struct MyEvent {
    pub message: String,
}
```
Register in `lib.rs`: `collect_events![MyEvent]`

### Naming Conventions
| Type | Convention | Example |
|------|------------|---------|
| Files | snake_case | `my_module.rs` |
| Functions | snake_case | `get_user()` |
| Types/Structs | PascalCase | `AppState` |
| Constants | UPPER_SNAKE | `MAX_SIZE` |

### State Management
```rust
pub struct AppState {
    pub count: u32,
}

// In commands:
fn my_cmd(state: State<'_, Mutex<AppState>>) -> Result<(), AppError> {
    let mut state = state.lock().unwrap();
    state.count += 1;
    Ok(())
}
```

## Before Committing

Husky pre-commit hook automatically runs:
- Git user.name/email verification (shlifedev/shlifedev@gmail.com)
- `cargo fmt --check` and `cargo clippy`
- `bun run check` for TypeScript validation

Manual checks (if needed):
```bash
cd src-tauri && cargo fmt && cargo clippy
bun run check
```

## Release Process

Releases are automated via GitHub Actions when you push a tag:

```bash
# Create and push a version tag
git tag v1.0.0
git push origin v1.0.0
```

This triggers `.github/workflows/release.yml` which:
1. Builds for Windows (exe), macOS (dmg/app - Universal binary), Linux (AppImage/deb)
2. Uses Rust cache for faster builds
3. Creates a draft release with all platform binaries
4. Review and publish the draft release on GitHub

**Note:** macOS builds are signed if `TAURI_SIGNING_PRIVATE_KEY` secret is configured.

## Common Tasks

### Adding a New Command
1. Define in `src-tauri/src/command.rs` with `#[tauri::command]` + `#[specta::specta]`
2. Add to `collect_commands![]` in `lib.rs`
3. Run `bun run tauri dev` to regenerate bindings
4. Use via `commands.myCommand()` in frontend

### Adding a New Tool
1. Add entry to `toolCategories` in `src/lib/config/tools.ts`
2. Create route: `src/routes/tools/{category}/{tool-id}/+page.svelte`
3. (Optional) Add backend commands in `src-tauri/src/tools/{category}/`
4. Register commands in `src-tauri/src/command.rs` and `lib.rs`

### Adding a New Route (Non-Tool)
1. Create `src/routes/my-route/+page.svelte`
2. Available at `/my-route`

### Adding State Fields
1. Add to `AppState` struct in `lib.rs`
2. Update initialization in `.manage()` call

## Tools Overview

### Base64 (인코딩)
**Path:** `/tools/encoding/base64`

텍스트 ↔ Base64 인코딩/디코딩 도구. 양방향 변환 지원.

**Backend:** `src-tauri/src/tools/encoding/base64.rs`
- `base64_encode(input: String) -> String`
- `base64_decode(input: String) -> Result<String, Base64Error>`

### Motion Descriptor (LLM 도구)
**Path:** `/tools/llm-toolset/motion-descriptor`

마우스 모션을 LLM 프롬프트로 변환. 캔버스에 그린 동작을 텍스트로 설명.

**Features:**
- 캔버스에 마우스로 모션 그리기
- 샘플링 빈도 조절 (5-150 샘플)
- 좌표 정규화 (0-1 범위)
- 프롬프트 자동 생성 및 복사

### Sprite Sheet Describer (게임 개발)
**Path:** `/tools/gamedev/sprite-sheet-describer`

스프라이트 시트를 n×n 그리드로 나누고 각 셀에 설명 추가.

**Features:**
- 이미지 파일 로드 (drag & drop or file picker)
- 그리드 크기: 1×1 ~ 50×50
- 단일/멀티 셀 선택 모드
- 프리셋 태그 + 커스텀 태그 (localStorage)
- 출력: JSON, 텍스트 리스트
- 인덱스 형식: Row/Col 또는 Index
- 프로젝트 저장/불러오기

**Selection Modes:**
- Single: Click cell → modal
- Multi: Click (single), Ctrl+click (toggle), Shift+click (rectangle)

### Tween Visualizer (게임 개발)
**Path:** `/tools/gamedev/tween-visualizer`

이징 함수(easing functions) 시각화 및 비교 도구.

**Features:**
- 30+ 이징 함수 지원 (Linear, Quad, Cubic, Elastic, Bounce 등)
- 최대 3개 함수 동시 비교
- 실시간 애니메이션 재생
- 그래프 호버 시 t/value 표시
- 컬러 코드로 구분

## Keeping This File Updated

When you add:
- New command → Document pattern if non-trivial
- New module → Add to project structure
- New dependency → Note if it changes patterns
- New convention → Add to guidelines
