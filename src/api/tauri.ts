import { invoke } from "@tauri-apps/api/core";
import type { AiSettings, DirEntry, TemplateItem } from "@/types";

/**
 * Tauri IPC 统一封装：前端通过 invoke 调用 Rust 后端的命令。
 * 参数名采用 camelCase，由 Tauri 自动映射为 Rust 侧的 snake_case。
 */
export const api = {
  // ===== AI 能力 =====
  generateCode: (requirement: string, language: string) =>
    invoke<string>("generate_code", { requirement, language }),
  refactorCode: (code: string) => invoke<string>("refactor_code", { code }),
  debugError: (error: string) => invoke<string>("debug_error", { error }),
  analyzeCode: (code: string) => invoke<string>("analyze_code", { code }),
  generateFromTemplate: (templateId: string, input: string) =>
    invoke<string>("generate_from_template", { templateId, input }),

  // ===== 提示词模板管理 =====
  listTemplates: () => invoke<TemplateItem[]>("list_templates"),
  saveCustomTemplate: (item: TemplateItem) =>
    invoke<void>("save_custom_template", { item }),
  deleteCustomTemplate: (id: string) =>
    invoke<void>("delete_custom_template", { id }),

  // ===== 本地文件 =====
  readLocalFile: (path: string) => invoke<string>("read_local_file", { path }),
  saveLocalFile: (path: string, content: string) =>
    invoke<void>("save_local_file", { path, content }),
  listDir: (path: string) => invoke<DirEntry[]>("list_dir", { path }),

  // ===== 本地工具箱 =====
  jsonFormat: (text: string) => invoke<string>("json_format", { text }),
  base64Encode: (text: string) => invoke<string>("base64_encode", { text }),
  base64Decode: (text: string) => invoke<string>("base64_decode", { text }),

  // ===== 模型设置 =====
  getSettings: () => invoke<AiSettings>("get_settings"),
  saveSettings: (settings: AiSettings) =>
    invoke<void>("save_settings", { settings }),
};
