<script setup lang="ts">
import { computed } from "vue";
import SideNav from "@/components/SideNav.vue";
import CodeGenView from "@/views/CodeGenView.vue";
import TemplateView from "@/views/TemplateView.vue";
import ToolboxView from "@/views/ToolboxView.vue";
import SettingsModal from "@/components/SettingsModal.vue";
import { useAppStore } from "@/stores";

const appStore = useAppStore();

const viewMap = {
  codegen: CodeGenView,
  templates: TemplateView,
  toolbox: ToolboxView,
} as const;

const CurrentView = computed(() => viewMap[appStore.currentView]);
</script>

<template>
  <div class="app-shell">
    <SideNav />
    <main class="main-area">
      <component :is="CurrentView" />
    </main>
    <SettingsModal v-if="appStore.settingsModalOpen" />
  </div>
</template>
