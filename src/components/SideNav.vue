<script setup lang="ts">
import { useAppStore, type ViewKey } from "@/stores";
import Icon from "@/components/Icon.vue";

const appStore = useAppStore();

const navItems: { key: ViewKey; label: string; icon: string }[] = [
  { key: "codegen", label: "代码生成", icon: "bolt" },
  { key: "templates", label: "提示词模板", icon: "list" },
  { key: "toolbox", label: "本地工具箱", icon: "wrench" },
];
</script>

<template>
  <aside class="sidenav">
    <div class="sidenav-brand">
      <div class="brand-mark">DA</div>
      <div class="brand-text">
        <strong>DevAssist AI</strong>
        <span>AI 辅助开发</span>
      </div>
    </div>

    <nav class="sidenav-nav">
      <button
        v-for="item in navItems"
        :key="item.key"
        class="nav-item"
        :class="{ active: appStore.currentView === item.key }"
        @click="appStore.setView(item.key)"
      >
        <Icon :name="item.icon" :size="16" />
        <span>{{ item.label }}</span>
      </button>
    </nav>

    <div class="sidenav-footer">
      <button class="nav-item" @click="appStore.toggleSettingsModal(true)">
        <Icon name="gear" :size="16" />
        <span>模型设置</span>
      </button>
      <div class="version">v0.1.0 · Rust + Tauri</div>
    </div>
  </aside>
</template>

<style scoped>
.sidenav {
  width: 208px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  background: var(--bg-panel);
  border-right: 1px solid var(--border);
}
.sidenav-brand {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 16px 14px;
  border-bottom: 1px solid var(--border);
}
.brand-mark {
  width: 34px;
  height: 34px;
  border-radius: 9px;
  background: linear-gradient(135deg, #4f8cff, #7a5cff);
  color: #fff;
  font-weight: 800;
  font-size: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  letter-spacing: 0.5px;
}
.brand-text {
  display: flex;
  flex-direction: column;
  line-height: 1.25;
}
.brand-text strong {
  font-size: 13.5px;
}
.brand-text span {
  font-size: 11px;
  color: var(--text-faint);
}
.sidenav-nav {
  flex: 1;
  padding: 12px 10px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.nav-item {
  display: flex;
  align-items: center;
  gap: 9px;
  padding: 9px 12px;
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-dim);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  text-align: left;
  transition: background 0.15s, color 0.15s;
}
.nav-item:hover {
  background: var(--bg-hover);
  color: var(--text);
}
.nav-item.active {
  background: var(--accent-soft);
  color: var(--accent);
  font-weight: 600;
}
.sidenav-footer {
  padding: 12px 10px;
  border-top: 1px solid var(--border);
}
.version {
  margin-top: 10px;
  padding: 0 12px;
  font-size: 10.5px;
  color: var(--text-faint);
}
</style>
