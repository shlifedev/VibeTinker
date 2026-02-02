# VibeTinker

> Your extensible workbench for developer tools, powered by LLM-assisted development

VibeTinker is a desktop application that lets you build your own collection of developer utilities with an intuitive sidebar interface. Add new tools on the fly using LLM assistance, making it your personalized productivity workbench.

## ✨ Features

- 🎨 **Sidebar Navigation** - Clean, organized tool access
- 🤖 **LLM-Powered Extension** - Build new tools with AI assistance
- ⚡ **Native Performance** - Tauri + Rust backend for speed
- 🎯 **Purpose-Built Tools** - Start with curated utilities, expand to your needs

## 🛠️ Built-in Tools

### 인코딩
- **Base64** - Encode/decode text ↔ Base64

### LLM 도구
- **Motion Descriptor** - Convert mouse gestures to natural language descriptions for LLM prompts

### 게임 개발
- **Sprite Sheet Describer** - Annotate sprite sheet grids with descriptions, export as JSON
- **Tween Visualizer** - Visualize and compare 30+ easing functions

## 🚀 Quick Start

### Prerequisites
- [Bun](https://bun.sh) (Package manager)
- [Rust](https://rustup.rs) (For Tauri backend)

### Development

```bash
# Install dependencies
bun install

# Run in development mode
bun run tauri dev
```

### Build

```bash
# Build production app
bun run tauri build
```

## 📦 Tech Stack

- **Frontend**: SvelteKit 2.x + Svelte 5 with Skeleton UI
- **Backend**: Rust + Tauri 2.0
- **Type Safety**: tauri-specta for automatic TypeScript bindings
- **Styling**: Tailwind CSS 4

## 🎯 Project Philosophy

VibeTinker is designed to grow with you. Rather than being a monolithic toolset, it's a **framework for building your own tools**. Use LLMs to quickly scaffold new utilities, customize existing ones, and create a workbench that matches your workflow.

## 🧩 Adding New Tools

Tools are organized in categories. To add a new tool:

1. Define it in `src/lib/config/tools.ts`
2. Create route: `src/routes/tools/{category}/{tool-id}/+page.svelte`
3. (Optional) Add backend commands in `src-tauri/src/tools/`

See [CLAUDE.md](./Claude.md) for detailed development guidelines.

## 🏗️ Architecture

```
VibeTinker/
├── src/                    # Frontend (SvelteKit)
│   ├── routes/tools/      # Tool implementations
│   ├── lib/components/    # Reusable UI components
│   └── lib/config/        # Tool metadata & routing
├── src-tauri/             # Backend (Rust)
│   ├── src/tools/         # Backend command implementations
│   └── src/modules/       # Shared utilities
└── .github/workflows/     # CI/CD for multi-platform builds
```

## 📝 Development Workflow

1. **Pre-commit Hooks** - Husky runs format/lint checks automatically
2. **Type Safety** - Automatic TypeScript bindings from Rust commands
3. **Hot Reload** - Instant feedback during development

## 🚢 Release Process

VibeTinker uses GitHub Actions for automated multi-platform builds:

```bash
# Create and push a version tag
git tag v1.0.0
git push origin v1.0.0
```

This triggers builds for:
- **Windows** - `.exe` installer
- **macOS** - Universal `.dmg` + `.app` (Intel + Apple Silicon)
- **Linux** - `.AppImage` + `.deb`

## 🤝 Contributing

Contributions are welcome! This project thrives on community-added tools and improvements.

1. Fork the repository
2. Create your feature branch
3. Add your tool following the project structure
4. Submit a pull request

## 📄 License

MIT

## 🙏 Acknowledgments

- Built with [Tauri](https://tauri.app)
- UI components from [Skeleton](https://skeleton.dev)
- Inspired by the desire to make developer tools more accessible and customizable
