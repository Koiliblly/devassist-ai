<script setup lang="ts">
/**
 * 轻量内联 SVG 图标组件：以字符串方式内联，避免引入图标库依赖。
 */
defineProps<{ name: string; size?: number }>();

const ICONS: Record<string, string> = {
  bolt: '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M8.5 1.5 3 9h4l-.5 5.5L12 7H8l.5-5.5Z"/></svg>',
  list: '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"><path d="M2.5 4h11M2.5 8h11M2.5 12h7"/></svg>',
  wrench: '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"><path d="M13.5 6.5a4 4 0 0 1-5.3 4.4L4.2 14.9a1.5 1.5 0 0 1-2.1-2.1l4-4A4 4 0 0 1 10.6 3.2l-1.9 1.9 2.2 2.2 1.9-1.9c.5.8.7 1.7.7 2.6Z"/></svg>',
  gear: '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"><circle cx="8" cy="8" r="2.2"/><path d="M8 1.8v2M8 12.2v2M1.8 8h2M12.2 8h2M3.6 3.6l1.4 1.4M11 11l1.4 1.4M12.4 3.6 11 5M5 11l-1.4 1.4"/></svg>',
  copy: '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"><rect x="5.5" y="5.5" width="8" height="8" rx="1.5"/><path d="M10.5 3.5h-5a2 2 0 0 0-2 2v5"/></svg>',
  play: '<svg viewBox="0 0 16 16" fill="currentColor"><path d="M4.5 2.8v10.4a.6.6 0 0 0 .92.5l8.4-5.2a.6.6 0 0 0 0-1l-8.4-5.2a.6.6 0 0 0-.92.5Z"/></svg>',
  folder: '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"><path d="M1.8 3.5h4l1.5 2h6.9v7a1.2 1.2 0 0 1-1.2 1.2H3a1.2 1.2 0 0 1-1.2-1.2v-9Z"/></svg>',
  file: '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"><path d="M4 1.5h5l3 3v10H4z"/><path d="M9 1.5v3h3"/></svg>',
  plus: '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"><path d="M8 3v10M3 8h10"/></svg>',
  trash: '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"><path d="M2.5 4h11M6.5 4V2.5h3V4M4 4l.6 9.3a1 1 0 0 0 1 .7h4.8a1 1 0 0 0 1-.7L12 4M6.5 7v4.5M9.5 7v4.5"/></svg>',
  save: '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"><path d="M2.5 2.5h9l2 2v9h-11z"/><path d="M5 2.5V6h5V2.5M5 13.5v-4h6v4"/></svg>',
  search: '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"><circle cx="7" cy="7" r="4.5"/><path d="m10.5 10.5 3 3"/></svg>',
};
</script>

<template>
  <span
    class="icon"
    :style="{ width: (size ?? 16) + 'px', height: (size ?? 16) + 'px' }"
    v-html="ICONS[name] || ''"
  ></span>
</template>

<style scoped>
.icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}
.icon :deep(svg) {
  width: 100%;
  height: 100%;
}
</style>
