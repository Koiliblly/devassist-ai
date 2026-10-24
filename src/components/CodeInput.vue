<script setup lang="ts">
/** 代码输入组件：多行文本输入，v-model 双向绑定 */
defineProps<{ modelValue: string; placeholder?: string; rows?: number }>();
defineEmits<{ (e: "update:modelValue", value: string): void }>();
</script>

<template>
  <textarea
    class="code-input"
    :value="modelValue"
    :placeholder="placeholder"
    :rows="rows ?? 8"
    spellcheck="false"
    @input="
      $emit('update:modelValue', ($event.target as HTMLTextAreaElement).value)
    "
  ></textarea>
</template>
