<script setup lang="ts">
import { computed } from "vue";
import hljs from "highlight.js/lib/common";
import "highlight.js/styles/github-dark.css";
import Icon from "@/components/Icon.vue";

const props = defineProps<{ code: string; language?: string }>();
const emit = defineEmits<{ (e: "copy"): void }>();

/** 使用 highlight.js 渲染高亮 HTML（失败时回退为纯文本转义） */
const highlightedHtml = computed(() => {
  if (!props.code) return "";
  try {
    if (props.language && hljs.getLanguage(props.language)) {
      return hljs.highlight(props.code, { language: props.language }).value;
    }
    return hljs.highlightAuto(props.code).value;
  } catch {
    return escapeHtml(props.code);
  }
});

function escapeHtml(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

function copy() {
  void navigator.clipboard.writeText(props.code);
  emit("copy");
}
</script>

<template>
  <div class="code-display">
    <div class="code-display-head">
      <span class="code-lang">{{ language || "text" }}</span>
      <button class="btn btn-ghost btn-sm" title="复制代码" @click="copy">
        <Icon name="copy" :size="13" /> 复制
      </button>
    </div>
    <pre class="code-pre"><code v-html="highlightedHtml"></code></pre>
  </div>
</template>
