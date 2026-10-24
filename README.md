# DevAssist AI — 基于 Rust + Tauri 的 AI 辅助开发桌面工具

> 轻量级本地桌面端 AI 编程助手，对标 GitHub Copilot / Cursor 核心能力：代码生成、代码重构、Bug 排查、提示词模板管理，帮你沉淀一套可复用的 AI 辅助开发流程。
>
> 技术栈：**Rust + Tauri 2 + Vue 3 + TypeScript + Vite**，兼容 OpenAI API 规范，可接入任意免费大模型。

---

## 功能特性

| 模块 | 能力 | 对应岗位要求 |
| ---- | ---- | ---- |
| AI 代码交互 | 代码生成：输入需求 + 目标语言，输出可直接运行的代码 | AI 工具使用、提示词工程 |
| AI 代码交互 | 代码重构：一键优化可读性 / 性能 / 安全性，补注释、规范化格式 | 代码规范、工程化 |
| AI 代码交互 | Bug 调试：粘贴报错信息，输出排查思路与修复方案 | 问题诊断能力 |
| 提示词模板管理 | 内置 7 类开发场景模板：代码评审、接口设计、单元测试生成、需求拆解等 | 提示词工程 |
| 提示词模板管理 | 自定义模板新增 / 编辑 / 本地保存，`{input}` 占位符复用 | 流程沉淀 |
| 本地开发工具箱 | 本地代码文件读取与预览、JSON 格式化、Base64 编解码 | 工程化基础能力 |
| 跨端通信层 | Tauri IPC 前后端通信，Rust 统一封装 AI 请求与本地文件操作 | Rust / Tauri 全链路 |

## 技术栈

| 层级 | 技术选型 | 说明 |
| ---- | ---- | ---- |
| 桌面框架 | Tauri 2.x | 高性能、轻量（安装包远小于 Electron） |
| 后端逻辑 | Rust（Cargo 管理） | 结构体、Trait、错误处理、异步命令、模块化设计 |
| 前端界面 | Vue 3 + TypeScript + Vite | 组合式 API、Pinia 状态管理、代码高亮 |
| AI 能力 | OpenAI API 兼容接口 | 可接入 OpenAI / DeepSeek / 硅基流动 / Ollama 本地等 |
| 工程化 | Git + 模块化目录 | 规范提交记录、按模块拆分 |

## 快速开始

### 环境要求

- Node.js ≥ 18
- Rust（stable 工具链）
- Windows 需安装 VS Build Tools（C++ 桌面开发工作负载）与 WebView2 运行时

### 启动命令

```bash
npm install        # 安装前端依赖
npm run tauri dev  # 启动桌面应用（开发端口固定 65532）
```

打包安装包：

```bash
npm run tauri build
```

### 首次使用

1. 点击左下角「模型设置」，填写 `API Base`、`API Key`、`模型名称`。
2. 无需 OpenAI 付费 Key：可将 `API Base` 指向 DeepSeek、硅基流动、通义千问等 OpenAI 兼容免费额度服务，或本地 Ollama（`http://localhost:11434/v1`）。
3. 进入「代码生成」页，输入需求与目标语言，点击「开始执行」。

## 项目结构

```
devassist-ai/
├── src/                     # 前端 Vue 源码
│   ├── components/          # 通用组件（代码编辑器、代码展示、导航、设置弹窗）
│   ├── views/               # 页面视图（代码生成、模板管理、工具箱）
│   ├── api/                 # Tauri 命令封装（IPC 调用）
│   ├── stores/              # Pinia 全局状态（导航、AI 设置）
│   ├── types/               # TypeScript 类型定义
│   └── App.vue / main.ts
├── src-tauri/               # Rust 后端源码
│   ├── src/
│   │   ├── main.rs          # 程序入口
│   │   ├── lib.rs           # 全局状态初始化 + Tauri 命令注册
│   │   ├── ai_client.rs     # AI 客户端结构体 + OpenAI 兼容接口封装
│   │   ├── template.rs      # PromptTemplate Trait + 内置/自定义模板
│   │   ├── file_handler.rs  # 本地文件读写与目录浏览
│   │   └── tools.rs         # 本地小工具（JSON 格式化、Base64 编解码）
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── capabilities/        # 权限配置
├── README.md                # 中文文档
├── README.en.md             # English docs
├── package.json
└── .gitignore
```

## 设计亮点

- **高性能轻量**：Rust + Tauri 2 实现，内存占用与安装包体积远小于 Electron 方案。
- **提示词工程沉淀**：以 Trait 抽象提示词模板，内置 7 类开发场景模板，支持自定义模板本地持久化，把 AI 辅助开发流程标准化、可复用。
- **模块化可扩展**：后端按 `ai_client / template / file_handler / tools` 拆分为独立模块，可快速扩展工业软件、IDE 工具等能力；前端通过统一的 IPC 封装层调用。
- **模型无关**：兼容 OpenAI API 规范，服务地址与模型名可配置，零成本接入免费大模型。

## Roadmap

- [ ] 多轮对话上下文（会话记忆）
- [ ] 代码库索引与语义检索（对标 Copilot 仓库级理解）
- [ ] 一键插入代码到本地文件
- [ ] 自定义快捷键与命令面板
- [ ] 模板市场 / 模板导入导出

## License

MIT
