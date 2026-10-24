//! 本地文件处理模块
//!
//! 覆盖 Rust 核心考点：
//! - `Path` / `PathBuf` 路径处理
//! - 标准库 `fs` 文件读写、错误处理与优雅降级
//! - 序列化（`DirEntry` 输出给前端）

use serde::Serialize;
use std::fs;
use std::path::PathBuf;

/// 单文件读取大小上限（1MB），避免大文件拖慢界面
const MAX_FILE_SIZE: u64 = 1024 * 1024;

/// 目录条目信息
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
}

/// 读取本地文本文件内容
pub fn read_local_file(path: String) -> Result<String, String> {
    let p = PathBuf::from(&path);
    if !p.is_file() {
        return Err(format!("文件不存在或不是普通文件: {path}"));
    }
    let meta = fs::metadata(&p).map_err(|e| format!("读取文件元信息失败: {e}"))?;
    if meta.len() > MAX_FILE_SIZE {
        return Err("文件超过 1MB 读取上限，暂不支持".into());
    }
    fs::read_to_string(&p).map_err(|e| format!("读取文件失败: {e}"))
}

/// 将内容保存到本地文件（自动创建父目录）
pub fn save_local_file(path: String, content: String) -> Result<(), String> {
    let p = PathBuf::from(&path);
    if let Some(parent) = p.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {e}"))?;
        }
    }
    fs::write(&p, content).map_err(|e| format!("写入文件失败: {e}"))
}

/// 列出目录内容：目录优先、其余按名称排序
pub fn list_dir(path: String) -> Result<Vec<DirEntry>, String> {
    let p = PathBuf::from(&path);
    if !p.is_dir() {
        return Err(format!("路径不是目录: {path}"));
    }

    let mut entries: Vec<DirEntry> = Vec::new();
    let rd = fs::read_dir(&p).map_err(|e| format!("读取目录失败: {e}"))?;

    for entry in rd.flatten() {
        let ep = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = ep.is_dir();
        let size = if is_dir {
            0
        } else {
            ep.metadata().map(|m| m.len()).unwrap_or(0)
        };
        entries.push(DirEntry {
            name,
            path: ep.to_string_lossy().to_string(),
            is_dir,
            size,
        });
    }

    entries.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    Ok(entries)
}
