import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import "./style.css";
import { useSettingsStore } from "./stores";

const app = createApp(App);
app.use(createPinia());
app.mount("#app");

// 启动时从 Rust 后端加载 AI 配置
void useSettingsStore().load();
