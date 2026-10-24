<script setup lang="ts">
import { ref } from "vue";
import CodeInput from "@/components/CodeInput.vue";
import CodeDisplay from "@/components/CodeDisplay.vue";
import Icon from "@/components/Icon.vue";
import { api } from "@/api/tauri";
import type { DirEntry, ToolKey } from "@/types";

const tool = ref<ToolKey>("file");

const TOOL_TABS: { key: ToolKey; label: string }[] = [
  { key: "file", label: "本地文件" },
  { key: "json", label: "JSON 格式化" },
  { key: "base64", label: "Base64" },
];

// ===== 本地文件 =====
const path = ref("");
const entries = ref<DirEntry[]>([]);
const currentFile = ref("");
const fileContent = ref("");
const fileMsg = ref("");
const fileErr = ref("");

async function listDir() {
  fileErr.value = "";
  fileMsg.value = "";
  if (!path.value.trim()) {
    fileErr.value = "请输入目录路径";
    return;
  }
  try {
    entries.value = await api.listDir(path.value);
    currentFile.value = "";
    fileContent.value = "";
  } catch (e) {
    fileErr.value = String(e);
  }
}

async function openEntry(entry: DirEntry) {
  if (entry.isDir) {
    path.value = entry.path;
    await listDir();
    return;
  }
  await readFile(entry.path);
}

async function readFile(filePath: string) {
  fileErr.value = "";
  try {
    fileContent.value = await api.readLocalFile(filePath);
    currentFile.value = filePath;
  } catch (e) {
    fileErr.value = String(e);
  }
}

async function saveFile() {
  fileErr.value = "";
  if (!currentFile.value) return;
  try {
    await api.saveLocalFile(currentFile.value, fileContent.value);
    fileMsg.value = "已保存";
    setTimeout(() => (fileMsg.value = ""), 2000);
  } catch (e) {
    fileErr.value = String(e);
  }
}

function formatSize(size: number): string {
  if (size < 1024) return `${size} B`;
  if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`;
  return `${(size / 1024 / 1024).toFixed(1)} MB`;
}

// ===== JSON =====
const jsonInput = ref("");
const jsonOutput = ref("");
const jsonErr = ref("");
const jsonLoading = ref(false);

async function formatJson() {
  jsonErr.value = "";
  jsonLoading.value = true;
  try {
    jsonOutput.value = await api.jsonFormat(jsonInput.value);
  } catch (e) {
    jsonErr.value = String(e);
  } finally {
    jsonLoading.value = false;
  }
}

// ===== Base64 =====
const b64Mode = ref<"encode" | "decode">("encode");
const b64Input = ref("");
const b64Output = ref("");
const b64Err = ref("");
const b64Loading = ref(false);

async function convertB64() {
  b64Err.value = "";
  b64Loading.value = true;
  try {
    b64Output.value =
      b64Mode.value === "encode"
        ? await api.base64Encode(b64Input.value)
        : await api.base64Decode(b64Input.value);
  } catch (e) {
    b64Err.value = String(e);
  } finally {
    b64Loading.value = false;
  }
}

function copyText(text: string) {
  void navigator.clipboard.writeText(text);
}
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h2>本地开发工具箱</h2>
        <p class="page-sub">本地代码文件读取与预览、JSON 格式化、Base64 编解码</p>
      </div>
    </div>

    <div class="tabs">
      <button
        v-for="t in TOOL_TABS"
        :key="t.key"
        class="tab"
        :class="{ active: tool === t.key }"
        @click="tool = t.key"
      >
        {{ t.label }}
      </button>
    </div>

    <!-- 本地文件 -->
    <div v-if="tool === 'file'" class="card">
      <label class="field-label">目录 / 文件路径</label>
      <div style="display: flex; gap: 10px">
        <input
          v-model="path"
          class="input mono"
          type="text"
          placeholder="例如 C:\Users\Lenovo\source\my-project"
          @keydown.enter="listDir"
        />
        <button class="btn btn-ghost" @click="listDir">
          <Icon name="folder" :size="13" /> 列出目录
        </button>
      </div>
      <p v-if="fileErr" class="status-bar status-error" style="user-select: text">{{ fileErr }}</p>

      <div v-if="entries.length" class="file-list" style="margin-top: 12px">
        <div
          v-for="e in entries"
          :key="e.path"
          class="file-row"
          :style="{ color: e.isDir ? 'var(--accent)' : undefined }"
          @click="openEntry(e)"
        >
          <Icon :name="e.isDir ? 'folder' : 'file'" :size="14" />
          <span>{{ e.name }}</span>
          <span v-if="!e.isDir" class="size">{{ formatSize(e.size) }}</span>
        </div>
      </div>

      <template v-if="currentFile">
        <label class="field-label">当前文件：{{ currentFile }}</label>
        <CodeInput v-model="fileContent" :rows="12" />
        <div style="display: flex; gap: 10px; margin-top: 12px; align-items: center">
          <button class="btn" @click="saveFile">
            <Icon name="save" :size="13" /> 保存修改
          </button>
          <span v-if="fileMsg" class="status-bar status-ok">{{ fileMsg }}</span>
        </div>
      </template>

      <div v-if="!entries.length && !currentFile" class="empty-tip">
        输入目录路径后点击「列出目录」，或直接输入文件完整路径回车读取
      </div>
    </div>

    <!-- JSON -->
    <div v-else-if="tool === 'json'" class="card">
      <label class="field-label">待格式化 JSON</label>
      <CodeInput v-model="jsonInput" :rows="9" placeholder='{"name": "DevAssist", "version": "0.1.0"}' />
      <div style="display: flex; gap: 10px; margin-top: 12px; align-items: center">
        <button class="btn" :disabled="jsonLoading" @click="formatJson">
          {{ jsonLoading ? "格式化中…" : "格式化" }}
        </button>
      </div>
      <p v-if="jsonErr" class="status-bar status-error" style="user-select: text">{{ jsonErr }}</p>
      <div v-if="jsonOutput" style="margin-top: 14px">
        <div class="page-sub" style="margin-bottom: 8px">
          格式化结果
          <button class="btn btn-ghost btn-sm" style="margin-left: 8px" @click="copyText(jsonOutput)">
            <Icon name="copy" :size="12" /> 复制
          </button>
        </div>
        <CodeDisplay :code="jsonOutput" language="json" />
      </div>
    </div>

    <!-- Base64 -->
    <div v-else class="card">
      <label class="field-label">转换方向</label>
      <div class="tabs" style="margin-bottom: 12px">
        <button
          class="tab"
          :class="{ active: b64Mode === 'encode' }"
          @click="b64Mode = 'encode'"
        >
          文本 → Base64
        </button>
        <button
          class="tab"
          :class="{ active: b64Mode === 'decode' }"
          @click="b64Mode = 'decode'"
        >
          Base64 → 文本
        </button>
      </div>

      <label class="field-label">{{ b64Mode === "encode" ? "原始文本" : "Base64 字符串" }}</label>
      <CodeInput v-model="b64Input" :rows="6" />
      <div style="display: flex; gap: 10px; margin-top: 12px">
        <button class="btn" :disabled="b64Loading" @click="convertB64">
          {{ b64Loading ? "转换中…" : "转换" }}
        </button>
      </div>
      <p v-if="b64Err" class="status-bar status-error" style="user-select: text">{{ b64Err }}</p>
      <div v-if="b64Output" style="margin-top: 14px">
        <div class="page-sub" style="margin-bottom: 8px">
          转换结果
          <button class="btn btn-ghost btn-sm" style="margin-left: 8px" @click="copyText(b64Output)">
            <Icon name="copy" :size="12" /> 复制
          </button>
        </div>
        <CodeDisplay :code="b64Output" />
      </div>
    </div>
  </div>
</template>
