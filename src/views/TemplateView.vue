<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import CodeInput from "@/components/CodeInput.vue";
import CodeDisplay from "@/components/CodeDisplay.vue";
import Icon from "@/components/Icon.vue";
import { api } from "@/api/tauri";
import { useSettingsStore } from "@/stores";
import type { TemplateCategory, TemplateItem } from "@/types";

const settingsStore = useSettingsStore();

const templates = ref<TemplateItem[]>([]);
const selectedId = ref("");
const input = ref("");
const output = ref("");
const loading = ref(false);
const err = ref("");

/** 编辑器状态 */
const editorOpen = ref(false);
const editDraft = ref<TemplateItem | null>(null);

const CATEGORY_LABELS: Record<TemplateCategory, string> = {
  codegen: "代码生成",
  refactor: "代码重构",
  debug: "Bug 调试",
  review: "代码评审",
  design: "接口设计",
  test: "单元测试",
  plan: "需求拆解",
  custom: "自定义",
};

const builtins = computed(() => templates.value.filter((t) => t.isBuiltin));
const customs = computed(() => templates.value.filter((t) => !t.isBuiltin));
const selected = computed(() => templates.value.find((t) => t.id === selectedId.value));

const canRun = computed(() => !!selected.value && input.value.trim().length > 0);

async function load() {
  templates.value = await api.listTemplates();
  if (!selectedId.value && templates.value.length > 0) {
    selectedId.value = templates.value[0].id;
  }
}

async function runTemplate() {
  if (!selected.value) return;
  err.value = "";
  output.value = "";
  if (!settingsStore.loaded) await settingsStore.load();
  if (!settingsStore.settings.apiKey.trim()) {
    err.value = "请先点击左下角「模型设置」，填写 API Key 与模型地址";
    return;
  }
  loading.value = true;
  try {
    output.value = await api.generateFromTemplate(selected.value.id, input.value);
  } catch (e) {
    err.value = String(e);
  } finally {
    loading.value = false;
  }
}

function openNew() {
  editDraft.value = {
    id: "",
    name: "",
    category: "custom",
    description: "",
    systemPrompt: "你是一名资深软件工程师，请严格按照要求输出高质量结果。",
    userPrompt: "{input}",
    isBuiltin: false,
  };
  editorOpen.value = true;
}

function openEdit(item: TemplateItem) {
  editDraft.value = { ...item };
  editorOpen.value = true;
}

async function saveItem() {
  if (!editDraft.value) return;
  if (!editDraft.value.name.trim()) return;
  err.value = "";
  try {
    await api.saveCustomTemplate(editDraft.value);
    editorOpen.value = false;
    await load();
  } catch (e) {
    err.value = String(e);
  }
}

async function removeItem(id: string) {
  err.value = "";
  try {
    await api.deleteCustomTemplate(id);
    if (selectedId.value === id) {
      selectedId.value = "";
      output.value = "";
    }
    await load();
  } catch (e) {
    err.value = String(e);
  }
}
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h2>提示词模板</h2>
        <p class="page-sub">内置开发场景模板 + 自定义模板，沉淀可复用的 AI 辅助开发流程</p>
      </div>
      <button class="btn" @click="openNew">
        <Icon name="plus" :size="13" /> 新建模板
      </button>
    </div>

    <p v-if="err" class="status-bar status-error" style="user-select: text">{{ err }}</p>

    <div class="split">
      <!-- 左侧模板列表 -->
      <div style="width: 320px; flex-shrink: 0">
        <div class="list-group-title">内置模板</div>
        <div class="tpl-list">
          <div
            v-for="t in builtins"
            :key="t.id"
            class="tpl-item"
            :class="{ active: selectedId === t.id }"
            @click="selectedId = t.id"
          >
            <div style="min-width: 0">
              <div class="tpl-item-name">{{ t.name }}</div>
              <div class="tpl-item-desc">{{ t.description }}</div>
            </div>
            <span class="tag tag-builtin">内置</span>
          </div>
        </div>

        <div v-if="customs.length" class="list-group-title">自定义模板</div>
        <div class="tpl-list">
          <div
            v-for="t in customs"
            :key="t.id"
            class="tpl-item"
            :class="{ active: selectedId === t.id }"
            @click="selectedId = t.id"
          >
            <div style="min-width: 0">
              <div class="tpl-item-name">{{ t.name }}</div>
              <div class="tpl-item-desc">{{ t.description }}</div>
            </div>
            <div class="tpl-item-actions">
              <button
                class="btn btn-ghost btn-sm"
                title="编辑"
                @click.stop="openEdit(t)"
              >
                <Icon name="gear" :size="12" />
              </button>
              <button
                class="btn btn-danger btn-sm"
                title="删除"
                @click.stop="removeItem(t.id)"
              >
                <Icon name="trash" :size="12" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧详情 -->
      <div class="grow">
        <div v-if="selected" class="card">
          <div style="display: flex; align-items: center; gap: 10px">
            <h3 style="margin: 0; font-size: 15px">{{ selected.name }}</h3>
            <span class="tag">{{ CATEGORY_LABELS[selected.category] }}</span>
          </div>
          <p class="muted" style="margin: 8px 0 0; font-size: 12.5px">{{ selected.description }}</p>

          <label class="field-label">系统提示词</label>
          <pre class="code-pre" style="max-height: 140px; border: 1px solid var(--border); border-radius: var(--radius-sm)">{{ selected.systemPrompt }}</pre>

          <label class="field-label">模板正文（{{ "{input}" }} 为用户输入占位符）</label>
          <pre v-if="selected.userPrompt" class="code-pre" style="max-height: 120px; border: 1px solid var(--border); border-radius: var(--radius-sm)">{{ selected.userPrompt }}</pre>

          <label class="field-label">输入内容</label>
          <CodeInput v-model="input" :rows="4" placeholder="填写本次要交给模板的内容…" />

          <div style="display: flex; gap: 10px; margin-top: 12px; align-items: center">
            <button class="btn" :disabled="!canRun" @click="runTemplate">
              <Icon name="play" :size="13" />
              {{ loading ? "生成中…" : "使用该模板生成" }}
            </button>
          </div>

          <div v-if="output" style="margin-top: 16px">
            <div class="page-sub" style="margin-bottom: 8px">生成结果</div>
            <CodeDisplay :code="output" />
          </div>
        </div>
        <div v-else class="empty-tip">从左侧选择一个模板开始</div>
      </div>
    </div>

    <!-- 新建 / 编辑弹窗 -->
    <div v-if="editorOpen && editDraft" class="modal-mask" @click.self="editorOpen = false">
      <div class="modal" style="width: 560px">
        <div class="modal-head">
          <h3>{{ editDraft.id ? "编辑模板" : "新建模板" }}</h3>
          <button class="btn btn-ghost btn-sm" @click="editorOpen = false">关闭</button>
        </div>
        <div class="modal-body">
          <label class="field-label">模板名称</label>
          <input v-model="editDraft.name" class="input" type="text" placeholder="如：API 接口文档生成" />

          <label class="field-label">分类</label>
          <select v-model="editDraft.category" class="input">
            <option
              v-for="(label, key) in CATEGORY_LABELS"
              :key="key"
              :value="key"
            >
              {{ label }}
            </option>
          </select>

          <label class="field-label">简介</label>
          <input v-model="editDraft.description" class="input" type="text" placeholder="一句话说明模板用途" />

          <label class="field-label">系统提示词</label>
          <textarea
            v-model="editDraft.systemPrompt"
            class="code-input"
            rows="4"
            placeholder="定义 AI 的角色与输出要求"
          ></textarea>

          <label class="field-label">模板正文（用 {{ "{input}" }} 表示用户输入位置）</label>
          <textarea
            v-model="editDraft.userPrompt"
            class="code-input"
            rows="4"
            placeholder="例如：请根据以下需求输出接口设计文档：\n{input}"
          ></textarea>
        </div>
        <div class="modal-foot">
          <button
            class="btn"
            :disabled="!editDraft.name.trim()"
            @click="saveItem"
          >
            <Icon name="save" :size="13" /> 保存
          </button>
          <button class="btn btn-ghost" @click="editorOpen = false">取消</button>
        </div>
      </div>
    </div>
  </div>
</template>
