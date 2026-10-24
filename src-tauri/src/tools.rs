//! 本地开发小工具：JSON 格式化、Base64 编解码

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;

/// JSON 格式化：解析并美化缩进，保留原始键顺序
pub fn json_format(text: String) -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("JSON 解析失败: {e}"))?;
    serde_json::to_string_pretty(&value).map_err(|e| format!("JSON 序列化失败: {e}"))
}

/// Base64 编码（UTF-8 文本 → Base64）
pub fn base64_encode(text: String) -> String {
    BASE64.encode(text.as_bytes())
}

/// Base64 解码（Base64 → UTF-8 文本）
pub fn base64_decode(text: String) -> Result<String, String> {
    let bytes = BASE64
        .decode(text.trim())
        .map_err(|e| format!("Base64 解码失败: {e}"))?;
    String::from_utf8(bytes).map_err(|e| format!("解码结果不是合法 UTF-8 文本: {e}"))
}
