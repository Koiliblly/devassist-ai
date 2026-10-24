<script setup lang="ts">
import { computed, ref } from "vue";
import CodeInput from "@/components/CodeInput.vue";
import CodeDisplay from "@/components/CodeDisplay.vue";
import Icon from "@/components/Icon.vue";
import { api } from "@/api/tauri";
import { useSettingsStore } from "@/stores";
import type { GenMode } from "@/types";

const settingsStore = useSettingsStore();

/** 当前工作模式 */
const mode = ref<GenMode>("generate");
const requirement = ref("");
const language = ref("python");
const codeInput = ref("");
const errorInput = ref("");
const output = ref("");
const outputLang = ref("");
const loading = ref(false);
const err = ref("");
const copied = ref(false);

const LANGUAGES = [
  "python",
  "javascript",
  "typescript",
  "rust",
  "go",
  "java",
  "c",
  "cpp",
  "csharp",
  "sql",
  "bash",
  "json",
  "html",
  "css",
] as const;

const MODES: { key: GenMode; label: string }[] = [
  { key: "generate", label: "代码生成" },
  { key: "refactor", label: "代码重构" },
  { key: "debug", label: "Bug 调试" },
];

const canRun = computed(() => {
  if (loading.value) return false;
  if (mode.value === "generate") return requirement.value.trim().length > 0;
  if (mode.value === "refactor") return codeInput.value.trim().length > 0;
  return errorInput.value.trim().length > 0;
});

/** 校验设置并执行 AI 调用 */
async function run() {
  err.value = "";
  copied.value = false;
  if (!settingsStore.loaded) await settingsStore.load();
  if (!settingsStore.settings.apiKey.trim()) {
    err.value = "请先点击左下角「模型设置」，填写 API Key 与模型地址";
    return;
  }
  loading.value = true;
  output.value = "";
  try {
    if (mode.value === "generate") {
      output.value = await api.generateCode(requirement.value, language.value);
      outputLang.value = language.value;
    } else if (mode.value === "refactor") {
      output.value = await api.refactorCode(codeInput.value);
      outputLang.value = "";
    } else {
      output.value = await api.debugError(errorInput.value);
      outputLang.value = "";
    }
  } catch (e) {
    err.value = String(e);
  } finally {
    loading.value = false;
  }
}

function copy() {
  copied.value = true;
  setTimeout(() => (copied.value = false), 1500);
}
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h2>AI 代码交互</h2>
        <p class="page-sub">代码生成 / 一键重构 / Bug 排查，全程调用大模型，输入即输出</p>
      </div>
    </div>

    <div class="tabs">
      <button
        v-for="m in MODES"
        :key="m.key"
        class="tab"
        :class="{ active: mode === m.key }"
        @click="mode = m.key"
      >
        {{ m.label }}
      </button>
    </div>

    <!-- 代码生成 -->
    <div v-if="mode === 'generate'" class="card">
      <label class="field-label">目标语言</label>
      <select v-model="language" class="input" style="max-width: 260px">
        <option v-for="lang in LANGUAGES" :key="lang" :value="lang">{{ lang }}</option>
      </select>

      <label class="field-label">开发需求描述</label>
      <CodeInput
        v-model="requirement"
        :rows="7"
        placeholder="例如：用 Python 写一个读取 CSV 并统计每列缺失率的函数，输出 Markdown 表格"
      />
    </div>

    <!-- 代码重构 -->
    <div v-else-if="mode === 'refactor'" class="card">
      <label class="field-label">粘贴需要优化的代码</label>
      <CodeInput
        v-model="codeInput"
        :rows="10"
        placeholder="粘贴你的代码，AI 将完成：可读性优化、补注释、格式规范化，并给出改动说明"
      />
    </div>

    <!-- Bug 调试 -->
    <div v-else class="card">
      <label class="field-label">粘贴报错信息 / 异常堆栈</label>
      <CodeInput
        v-model="errorInput"
        :rows="8"
        placeholder="例如：KeyError: 'price' at line 42 ...（附带相关代码片段效果更佳）"
      />
    </div>

    <div style="display: flex; gap: 10px; align-items: center">
      <button class="btn" :disabled="!canRun" @click="run">
        <Icon name="play" :size="13" />
        {{ loading ? "生成中…" : "开始执行" }}
      </button>
      <span v-if="!settingsStore.settings.apiKey" class="status-bar status-error">
        尚未配置模型，点击左下角「模型设置」
      </span>
    </div>

    <p v-if="err" class="status-bar status-error" style="user-select: text">{{ err }}</p>

    <div v-if="output" style="margin-top: 16px">
      <div class="page-sub" style="margin-bottom: 8px">
        执行结果<span v-if="copied" class="status-ok"> · 已复制</span>
      </div>
      <CodeDisplay :code="output" :language="outputLang" @copy="copy" />
    </div>
  </div>
</template>
