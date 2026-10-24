/** AI 服务配置（兼容 OpenAI API 规范） */
export interface AiSettings {
  apiBase: string;
  apiKey: string;
  model: string;
  temperature: number;
}

/** 提示词模板分类 */
export type TemplateCategory =
  | "codegen"
  | "refactor"
  | "debug"
  | "review"
  | "design"
  | "test"
  | "plan"
  | "custom";

/** 提示词模板项 */
export interface TemplateItem {
  id: string;
  name: string;
  category: TemplateCategory;
  description: string;
  systemPrompt: string;
  userPrompt: string;
  isBuiltin: boolean;
}

/** 目录条目 */
export interface DirEntry {
  name: string;
  path: string;
  isDir: boolean;
  size: number;
}

/** 代码生成页的三种工作模式 */
export type GenMode = "generate" | "refactor" | "debug";

/** 工具箱的三种工具 */
export type ToolKey = "file" | "json" | "base64";
