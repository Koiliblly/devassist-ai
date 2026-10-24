# DevAssist AI — AI-Assisted Development Desktop Tool Built with Rust + Tauri

> A lightweight local desktop AI coding assistant, benchmarking the core capabilities of GitHub Copilot / Cursor: code generation, code refactoring, bug triage, and prompt template management — helping you build a reusable AI-assisted development workflow.
>
> Tech stack: **Rust + Tauri 2 + Vue 3 + TypeScript + Vite**, compatible with the OpenAI API spec, connectable to any free LLM endpoint.

---

## Features

| Module | Capability | What it maps to |
| ---- | ---- | ---- |
| AI Code Interaction | Code generation: requirement + target language → runnable code | AI tooling, prompt engineering |
| AI Code Interaction | Code refactoring: one-click readability / performance / security improvements, comments & formatting | Code standards, engineering discipline |
| AI Code Interaction | Bug debugging: paste an error → root-cause analysis and a fix plan | Problem diagnosis |
| Prompt Templates | 7 built-in dev-scenario templates: code review, API design, unit test generation, requirement breakdown, etc. | Prompt engineering |
| Prompt Templates | Custom templates: create / edit / save locally, `{input}` placeholder reuse | Workflow standardization |
| Local Toolbox | Local code file read & preview, JSON formatting, Base64 encode/decode | Engineering fundamentals |
| IPC Layer | Tauri IPC between frontend and Rust backend; AI calls & file ops wrapped uniformly in Rust | Full-stack Rust / Tauri |

## Tech Stack

| Layer | Choice | Notes |
| ---- | ---- | ---- |
| Desktop framework | Tauri 2.x | High performance, lightweight (far smaller than Electron bundles) |
| Backend | Rust (Cargo) | Structs, traits, error handling, async commands, modular design |
| Frontend | Vue 3 + TypeScript + Vite | Composition API, Pinia state, syntax highlighting |
| AI capability | OpenAI-compatible API | OpenAI / DeepSeek / SiliconFlow / local Ollama, etc. |
| Engineering | Git + modular layout | Conventional commits, module-based structure |

## Quick Start

### Prerequisites

- Node.js ≥ 18
- Rust (stable toolchain)
- Windows: Visual Studio Build Tools (Desktop development with C++) and WebView2 runtime

### Commands

```bash
npm install        # install frontend dependencies
npm run tauri dev  # launch the desktop app (dev server fixed at port 65532)
```

Build installers:

```bash
npm run tauri build
```

### First Run

1. Open **Model Settings** (bottom-left), fill in `API Base`, `API Key`, and `Model name`.
2. No paid OpenAI key needed: point `API Base` to any OpenAI-compatible service — DeepSeek, SiliconFlow, Qwen, or a local Ollama (`http://localhost:11434/v1`).
3. Go to **Code Generation**, type a requirement and pick a language, then hit **Run**.

## Project Structure

```
devassist-ai/
├── src/                     # Frontend (Vue)
│   ├── components/          # Code editor, code display, navigation, settings modal
│   ├── views/               # CodeGen / Templates / Toolbox views
│   ├── api/                 # Tauri command wrappers (IPC)
│   ├── stores/              # Pinia global state (navigation, AI settings)
│   ├── types/               # TypeScript type definitions
│   └── App.vue / main.ts
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── main.rs          # Entry point
│   │   ├── lib.rs           # App state init + Tauri command registration
│   │   ├── ai_client.rs     # AI client struct + OpenAI-compatible API wrapper
│   │   ├── template.rs      # PromptTemplate trait + built-in / custom templates
│   │   ├── file_handler.rs  # Local file read/write & directory browsing
│   │   └── tools.rs         # Local tools (JSON format, Base64)
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── capabilities/        # Permission config
├── README.md                # Chinese docs
├── README.en.md             # English docs (this file)
├── package.json
└── .gitignore
```

## Highlights

- **High performance & lightweight**: built on Rust + Tauri 2 — much lower memory footprint and bundle size than Electron.
- **Prompt engineering as code**: templates abstracted behind a Rust trait, 7 built-in dev templates, custom templates persisted locally — standardizing a reusable AI-assisted dev workflow.
- **Modular & extensible**: backend split into `ai_client / template / file_handler / tools`; frontend talks to Rust through a unified IPC wrapper, ready to extend toward industrial software / IDE tooling.
- **Model-agnostic**: OpenAI-compatible, configurable base URL and model — free LLM endpoints work out of the box.

## Roadmap

- [ ] Multi-turn conversation context (session memory)
- [ ] Codebase indexing & semantic retrieval (repo-level understanding, Copilot-like)
- [ ] One-click insertion of generated code into local files
- [ ] Custom shortcuts & command palette
- [ ] Template marketplace / import-export

## License

MIT
