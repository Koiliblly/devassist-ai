import { defineStore } from "pinia";
import type { AiSettings } from "@/types";
import { api } from "@/api/tauri";

/** 左侧导航的页面标识 */
export type ViewKey = "codegen" | "templates" | "toolbox";

/** 全局导航与界面状态 */
export const useAppStore = defineStore("app", {
  state: () => ({
    currentView: "codegen" as ViewKey,
    settingsModalOpen: false,
  }),
  actions: {
    setView(view: ViewKey) {
      this.currentView = view;
    },
    toggleSettingsModal(open: boolean) {
      this.settingsModalOpen = open;
    },
  },
});

const DEFAULT_SETTINGS: AiSettings = {
  apiBase: "https://api.openai.com/v1",
  apiKey: "",
  model: "gpt-4o-mini",
  temperature: 0.7,
};

/** AI 设置全局状态：启动时从 Rust 后端加载，保存时写回本地配置目录 */
export const useSettingsStore = defineStore("settings", {
  state: () => ({
    settings: { ...DEFAULT_SETTINGS } as AiSettings,
    loaded: false,
  }),
  actions: {
    async load() {
      try {
        this.settings = await api.getSettings();
      } catch (e) {
        console.error("加载模型设置失败，使用默认值", e);
        this.settings = { ...DEFAULT_SETTINGS };
      }
      this.loaded = true;
    },
    async save() {
      await api.saveSettings(this.settings);
    },
    reset() {
      this.settings = { ...DEFAULT_SETTINGS };
    },
  },
});
