//! DevAssist AI —— Tauri 2 桌面应用入口与命令注册
//!
//! 职责：
//! 1. 初始化全局状态（AI 客户端、提示词模板库）
//! 2. 注册全部 Tauri 命令，供前端通过 `invoke` 调用
//! 3. 模型设置与自定义模板的本地持久化

mod ai_client;
mod file_handler;
mod template;
mod tools;

use ai_client::{AiClient, AiSettings};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::Manager;
use template::{PromptTemplateStore, TemplateItem};

/// 全局应用状态：AI 客户端 + 提示词模板库
pub struct AppState {
    pub ai: Mutex<AiClient>,
    pub templates: Mutex<PromptTemplateStore>,
}

// ==================== AI 能力命令 ====================

/// 代码生成：需求 + 目标语言 → 代码
#[tauri::command]
async fn generate_code(
    state: tauri::State<'_, AppState>,
    requirement: String,
    language: String,
) -> Result<String, String> {
    let template = template::CodeGenTemplate { language };
    let ai = state.ai.lock().map_err(|e| e.to_string())?.clone();
    ai.complete(&template, &requirement).await
}

/// 代码重构：粘贴代码 → 优化后代码 + 改动说明
#[tauri::command]
async fn refactor_code(state: tauri::State<'_, AppState>, code: String) -> Result<String, String> {
    let template = template::RefactorTemplate;
    let ai = state.ai.lock().map_err(|e| e.to_string())?.clone();
    ai.complete(&template, &code).await
}

/// Bug 调试：报错信息 → 排查思路与修复方案
#[tauri::command]
async fn debug_error(state: tauri::State<'_, AppState>, error: String) -> Result<String, String> {
    let template = template::DebugTemplate;
    let ai = state.ai.lock().map_err(|e| e.to_string())?.clone();
    ai.complete(&template, &error).await
}

/// 代码评审：粘贴代码 → 多维评审意见
#[tauri::command]
async fn analyze_code(state: tauri::State<'_, AppState>, code: String) -> Result<String, String> {
    let template = template::CodeReviewTemplate;
    let ai = state.ai.lock().map_err(|e| e.to_string())?.clone();
    ai.complete(&template, &code).await
}

/// 使用指定模板生成：模板 id + 输入内容 → 模型输出
#[tauri::command]
async fn generate_from_template(
    state: tauri::State<'_, AppState>,
    template_id: String,
    input: String,
) -> Result<String, String> {
    // 先在局部作用域内查找模板并释放模板库锁，避免跨 await 持有锁
    let tpl = {
        state
            .templates
            .lock()
            .map_err(|e| e.to_string())?
            .find(&template_id)
            .ok_or_else(|| format!("模板不存在: {template_id}"))?
    };
    let ai = state.ai.lock().map_err(|e| e.to_string())?.clone();
    ai.complete(tpl.as_ref(), &input).await
}

// ==================== 模板管理命令 ====================

/// 列出全部模板（内置 + 自定义）
#[tauri::command]
fn list_templates(state: tauri::State<'_, AppState>) -> Vec<TemplateItem> {
    state
        .templates
        .lock()
        .map(|store| store.list())
        .unwrap_or_default()
}

/// 新增 / 覆盖自定义模板
#[tauri::command]
fn save_custom_template(
    state: tauri::State<'_, AppState>,
    mut item: TemplateItem,
) -> Result<(), String> {
    item.is_builtin = false;
    if item.id.trim().is_empty() {
        item.id = format!(
            "custom_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or(0)
        );
    }
    state
        .templates
        .lock()
        .map_err(|e| e.to_string())?
        .save_custom(item)
}

/// 删除自定义模板
#[tauri::command]
fn delete_custom_template(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    state
        .templates
        .lock()
        .map_err(|e| e.to_string())?
        .delete_custom(&id)
}

// ==================== 本地文件命令 ====================

#[tauri::command]
fn read_local_file(path: String) -> Result<String, String> {
    file_handler::read_local_file(path)
}

#[tauri::command]
fn save_local_file(path: String, content: String) -> Result<(), String> {
    file_handler::save_local_file(path, content)
}

#[tauri::command]
fn list_dir(path: String) -> Result<Vec<file_handler::DirEntry>, String> {
    file_handler::list_dir(path)
}

// ==================== 工具箱命令 ====================

#[tauri::command]
fn json_format(text: String) -> Result<String, String> {
    tools::json_format(text)
}

#[tauri::command]
fn base64_encode(text: String) -> String {
    tools::base64_encode(text)
}

#[tauri::command]
fn base64_decode(text: String) -> Result<String, String> {
    tools::base64_decode(text)
}

// ==================== 模型设置命令 ====================

#[tauri::command]
fn get_settings(state: tauri::State<'_, AppState>) -> AiSettings {
    state
        .ai
        .lock()
        .map(|ai| ai.settings().clone())
        .unwrap_or_default()
}

#[tauri::command]
fn save_settings(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    settings: AiSettings,
) -> Result<(), String> {
    state
        .ai
        .lock()
        .map_err(|e| e.to_string())?
        .update(settings.clone());
    persist_settings(&app, &settings)
}

// ==================== 设置持久化 ====================

fn settings_path(config_dir: &Path) -> PathBuf {
    config_dir.join("settings.json")
}

fn load_settings(config_dir: &Path) -> AiSettings {
    fs::read_to_string(settings_path(config_dir))
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

fn persist_settings(app: &tauri::AppHandle, settings: &AiSettings) -> Result<(), String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("获取配置目录失败: {e}"))?;
    fs::create_dir_all(&dir).map_err(|e| format!("创建配置目录失败: {e}"))?;
    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("设置序列化失败: {e}"))?;
    fs::write(settings_path(&dir), json).map_err(|e| format!("设置写入失败: {e}"))
}

// ==================== 应用入口 ====================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let config_dir = app
                .path()
                .app_config_dir()
                .map_err(|e| -> Box<dyn std::error::Error> { e.to_string().into() })?;
            fs::create_dir_all(&config_dir).ok();

            let settings = load_settings(&config_dir);
            let ai = AiClient::new(settings)
                .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
            let templates = PromptTemplateStore::new(config_dir.join("templates.json"));

            app.manage(AppState {
                ai: Mutex::new(ai),
                templates: Mutex::new(templates),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_code,
            refactor_code,
            debug_error,
            analyze_code,
            generate_from_template,
            list_templates,
            save_custom_template,
            delete_custom_template,
            read_local_file,
            save_local_file,
            list_dir,
            json_format,
            base64_encode,
            base64_decode,
            get_settings,
            save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
