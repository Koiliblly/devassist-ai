<script setup lang="ts">
import { ref } from "vue";
import { useAppStore, useSettingsStore } from "@/stores";

const appStore = useAppStore();
const settingsStore = useSettingsStore();

const form = ref({ ...settingsStore.settings });
const saving = ref(false);
const saved = ref(false);
const error = ref("");

async function save() {
  saving.value = true;
  error.value = "";
  saved.value = false;
  try {
    settingsStore.settings = { ...form.value };
    await settingsStore.save();
    saved.value = true;
  } catch (e) {
    error.value = String(e);
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <div class="modal-mask" @click.self="appStore.toggleSettingsModal(false)">
    <div class="modal">
      <div class="modal-head">
        <h3>模型设置</h3>
        <button class="btn btn-ghost btn-sm" @click="appStore.toggleSettingsModal(false)">
          关闭
        </button>
      </div>

      <div class="modal-body">
        <label class="field-label">API Base（兼容 OpenAI API 规范）</label>
        <input
          v-model="form.apiBase"
          class="input"
          type="text"
          placeholder="https://api.openai.com/v1"
        />
        <p class="muted" style="margin: 6px 0 0; font-size: 11.5px">
          可填任意 OpenAI 兼容接口地址（如 DeepSeek、硅基流动、Ollama 本地服务等）
        </p>

        <label class="field-label">API Key</label>
        <input
          v-model="form.apiKey"
          class="input"
          type="password"
          placeholder="sk-..."
          autocomplete="off"
        />

        <label class="field-label">模型名称</label>
        <input v-model="form.model" class="input" type="text" placeholder="gpt-4o-mini" />

        <label class="field-label">温度 Temperature（0 ~ 2）</label>
        <input
          v-model.number="form.temperature"
          class="input"
          type="number"
          step="0.1"
          min="0"
          max="2"
        />

        <p v-if="error" class="status-bar status-error">{{ error }}</p>
        <p v-if="saved" class="status-bar status-ok">已保存，下次请求生效</p>
      </div>

      <div class="modal-foot">
        <button class="btn" :disabled="saving" @click="save">
          {{ saving ? "保存中…" : "保存" }}
        </button>
        <button class="btn btn-ghost" @click="appStore.toggleSettingsModal(false)">取消</button>
      </div>
    </div>
  </div>
</template>
