//! 提示词模板模块
//!
//! 覆盖 Rust 核心考点：
//! - Trait 定义与默认方法（`PromptTemplate`）
//! - Trait 对象 / 动态分发（`Box<dyn PromptTemplate>`）
//! - 结构体、枚举（`TemplateCategory`）、错误处理
//! - 本地 JSON 持久化（自定义模板）

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// 模板分类（序列化为小写字符串，与前端 TypeScript 类型一一对应）
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TemplateCategory {
    Codegen,
    Refactor,
    Debug,
    Review,
    Design,
    Test,
    Plan,
    Custom,
}

impl TemplateCategory {
    pub fn label(self) -> &'static str {
        match self {
            Self::Codegen => "代码生成",
            Self::Refactor => "代码重构",
            Self::Debug => "Bug 调试",
            Self::Review => "代码评审",
            Self::Design => "接口设计",
            Self::Test => "单元测试",
            Self::Plan => "需求拆解",
            Self::Custom => "自定义",
        }
    }
}

/// 提示词模板 Trait：所有内置模板与自定义模板的统一抽象。
///
/// 使用 Trait 对象（`&dyn PromptTemplate`）作为 AI 客户端的输入，
/// 让 AI 调用层与具体模板解耦，便于横向扩展新场景模板。
pub trait PromptTemplate: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn category(&self) -> TemplateCategory;
    fn description(&self) -> &str;
    fn system_prompt(&self) -> &str;

    /// 将用户输入渲染为最终发送给模型的内容。
    /// 默认实现为「系统提示词 + 用户输入」，具体模板可按需覆写。
    fn render(&self, input: &str) -> String {
        format!("{}\n\n{}", self.system_prompt(), input)
    }
}

/// 模板数据模型：内置与自定义统一表示，用于 JSON 序列化后传输给前端
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateItem {
    pub id: String,
    pub name: String,
    pub category: TemplateCategory,
    pub description: String,
    pub system_prompt: String,
    pub user_prompt: String,
    pub is_builtin: bool,
}

// ==================== 内置模板实现 ====================

/// 代码生成模板（语言由命令参数注入）
pub struct CodeGenTemplate {
    pub language: String,
}

impl PromptTemplate for CodeGenTemplate {
    fn id(&self) -> &str {
        "code_gen"
    }
    fn name(&self) -> &str {
        "代码生成"
    }
    fn category(&self) -> TemplateCategory {
        TemplateCategory::Codegen
    }
    fn description(&self) -> &str {
        "根据开发需求与目标语言生成可直接运行的代码"
    }
    fn system_prompt(&self) -> &str {
        "你是一名资深软件工程师。请严格根据用户需求输出可直接运行的代码，包含必要的注释，并在最后用简短列表说明关键实现要点。"
    }
    fn render(&self, input: &str) -> String {
        format!(
            "请使用 {lang} 语言实现以下需求：\n\n{input}",
            lang = self.language
        )
    }
}

/// 代码重构模板
pub struct RefactorTemplate;

impl PromptTemplate for RefactorTemplate {
    fn id(&self) -> &str {
        "refactor"
    }
    fn name(&self) -> &str {
        "代码重构"
    }
    fn category(&self) -> TemplateCategory {
        TemplateCategory::Refactor
    }
    fn description(&self) -> &str {
        "优化可读性、性能与安全性，补充注释并规范化格式"
    }
    fn system_prompt(&self) -> &str {
        "你是一名资深代码重构专家。请在不改变功能行为的前提下，优化代码的可读性、性能与安全性：补充必要注释、统一命名与格式规范、消除重复与坏味道，并逐条说明主要改动及其原因。"
    }
}

/// Bug 调试模板
pub struct DebugTemplate;

impl PromptTemplate for DebugTemplate {
    fn id(&self) -> &str {
        "debug"
    }
    fn name(&self) -> &str {
        "Bug 调试"
    }
    fn category(&self) -> TemplateCategory {
        TemplateCategory::Debug
    }
    fn description(&self) -> &str {
        "分析报错信息，给出排查思路与修复方案"
    }
    fn system_prompt(&self) -> &str {
        "你是一名资深调试专家。请分析用户给出的报错信息与相关代码，定位最可能的根因，给出分步排查思路、可落地的修复方案（含代码示例），并指出如何预防同类问题。"
    }
}

/// 代码评审模板
pub struct CodeReviewTemplate;

impl PromptTemplate for CodeReviewTemplate {
    fn id(&self) -> &str {
        "code_review"
    }
    fn name(&self) -> &str {
        "代码评审"
    }
    fn category(&self) -> TemplateCategory {
        TemplateCategory::Review
    }
    fn description(&self) -> &str {
        "从正确性、性能、可维护性、安全性四维度评审代码"
    }
    fn system_prompt(&self) -> &str {
        "你是一名资深代码评审专家。请从正确性、性能、可维护性、安全性四个维度评审代码，按「问题严重程度」排序输出，每条给出问题定位、原因分析与修改建议。"
    }
}

/// 接口设计模板
pub struct InterfaceDesignTemplate;

impl PromptTemplate for InterfaceDesignTemplate {
    fn id(&self) -> &str {
        "interface_design"
    }
    fn name(&self) -> &str {
        "接口设计"
    }
    fn category(&self) -> TemplateCategory {
        TemplateCategory::Design
    }
    fn description(&self) -> &str {
        "根据需求设计 RESTful API，输出接口清单与请求/响应示例"
    }
    fn system_prompt(&self) -> &str {
        "你是一名资深后端架构师。请根据用户需求设计一套 RESTful API：列出接口路径、HTTP 方法、鉴权方式、请求参数与响应示例（JSON），并补充状态码约定与错误处理规范。"
    }
}

/// 单元测试生成模板
pub struct UnitTestTemplate;

impl PromptTemplate for UnitTestTemplate {
    fn id(&self) -> &str {
        "unit_test"
    }
    fn name(&self) -> &str {
        "单元测试生成"
    }
    fn category(&self) -> TemplateCategory {
        TemplateCategory::Test
    }
    fn description(&self) -> &str {
        "为给定代码生成单元测试，覆盖边界与异常分支"
    }
    fn system_prompt(&self) -> &str {
        "你是一名资深测试工程师。请为给定代码生成完整的单元测试：覆盖正常路径、边界条件与异常分支，使用主流测试框架（Jest/Pytest/Go test/cargo test 等，按语言选择），并给出测试说明。"
    }
}

/// 需求拆解模板
pub struct RequirementBreakdownTemplate;

impl PromptTemplate for RequirementBreakdownTemplate {
    fn id(&self) -> &str {
        "requirement_breakdown"
    }
    fn name(&self) -> &str {
        "需求拆解"
    }
    fn category(&self) -> TemplateCategory {
        TemplateCategory::Plan
    }
    fn description(&self) -> &str {
        "将需求拆解为功能列表、技术方案、任务清单与验收标准"
    }
    fn system_prompt(&self) -> &str {
        "你是一名资深产品与技术专家。请将用户给出的需求拆解为：功能列表（含优先级）、技术方案选型建议、可执行的任务清单（含依赖关系与预估工时）、以及可量化的验收标准。"
    }
}

/// 用户自定义模板：包装持久化的 TemplateItem 实现 Trait
pub struct CustomTemplate {
    item: TemplateItem,
}

impl PromptTemplate for CustomTemplate {
    fn id(&self) -> &str {
        &self.item.id
    }
    fn name(&self) -> &str {
        &self.item.name
    }
    fn category(&self) -> TemplateCategory {
        self.item.category
    }
    fn description(&self) -> &str {
        &self.item.description
    }
    fn system_prompt(&self) -> &str {
        &self.item.system_prompt
    }
    fn render(&self, input: &str) -> String {
        // 模板正文中的 {input} 占位符替换为用户输入
        self.item.user_prompt.replace("{input}", input)
    }
}

// ==================== 模板库 ====================

/// 模板库：管理内置模板 + 持久化的自定义模板
pub struct PromptTemplateStore {
    custom: Vec<TemplateItem>,
    data_path: PathBuf,
}

impl PromptTemplateStore {
    pub fn new(data_path: PathBuf) -> Self {
        let custom = Self::load_custom(&data_path);
        Self { custom, data_path }
    }

    fn builtin_items() -> Vec<TemplateItem> {
        vec![
            TemplateItem {
                id: "code_gen".into(),
                name: "代码生成".into(),
                category: TemplateCategory::Codegen,
                description: "根据开发需求与目标语言生成可直接运行的代码".into(),
                system_prompt: "你是一名资深软件工程师。请严格根据用户需求输出可直接运行的代码，包含必要的注释，并在最后用简短列表说明关键实现要点。".into(),
                user_prompt: String::new(),
                is_builtin: true,
            },
            TemplateItem {
                id: "refactor".into(),
                name: "代码重构".into(),
                category: TemplateCategory::Refactor,
                description: "优化可读性、性能与安全性，补充注释并规范化格式".into(),
                system_prompt: "你是一名资深代码重构专家。请在不改变功能行为的前提下，优化代码的可读性、性能与安全性：补充必要注释、统一命名与格式规范、消除重复与坏味道，并逐条说明主要改动及其原因。".into(),
                user_prompt: String::new(),
                is_builtin: true,
            },
            TemplateItem {
                id: "debug".into(),
                name: "Bug 调试".into(),
                category: TemplateCategory::Debug,
                description: "分析报错信息，给出排查思路与修复方案".into(),
                system_prompt: "你是一名资深调试专家。请分析用户给出的报错信息与相关代码，定位最可能的根因，给出分步排查思路、可落地的修复方案（含代码示例），并指出如何预防同类问题。".into(),
                user_prompt: String::new(),
                is_builtin: true,
            },
            TemplateItem {
                id: "code_review".into(),
                name: "代码评审".into(),
                category: TemplateCategory::Review,
                description: "从正确性、性能、可维护性、安全性四维度评审代码".into(),
                system_prompt: "你是一名资深代码评审专家。请从正确性、性能、可维护性、安全性四个维度评审代码，按「问题严重程度」排序输出，每条给出问题定位、原因分析与修改建议。".into(),
                user_prompt: String::new(),
                is_builtin: true,
            },
            TemplateItem {
                id: "interface_design".into(),
                name: "接口设计".into(),
                category: TemplateCategory::Design,
                description: "根据需求设计 RESTful API，输出接口清单与请求/响应示例".into(),
                system_prompt: "你是一名资深后端架构师。请根据用户需求设计一套 RESTful API：列出接口路径、HTTP 方法、鉴权方式、请求参数与响应示例（JSON），并补充状态码约定与错误处理规范。".into(),
                user_prompt: String::new(),
                is_builtin: true,
            },
            TemplateItem {
                id: "unit_test".into(),
                name: "单元测试生成".into(),
                category: TemplateCategory::Test,
                description: "为给定代码生成单元测试，覆盖边界与异常分支".into(),
                system_prompt: "你是一名资深测试工程师。请为给定代码生成完整的单元测试：覆盖正常路径、边界条件与异常分支，使用主流测试框架（Jest/Pytest/Go test/cargo test 等，按语言选择），并给出测试说明。".into(),
                user_prompt: String::new(),
                is_builtin: true,
            },
            TemplateItem {
                id: "requirement_breakdown".into(),
                name: "需求拆解".into(),
                category: TemplateCategory::Plan,
                description: "将需求拆解为功能列表、技术方案、任务清单与验收标准".into(),
                system_prompt: "你是一名资深产品与技术专家。请将用户给出的需求拆解为：功能列表（含优先级）、技术方案选型建议、可执行的任务清单（含依赖关系与预估工时）、以及可量化的验收标准。".into(),
                user_prompt: String::new(),
                is_builtin: true,
            },
        ]
    }

    /// 返回全部模板（内置 + 自定义），供前端展示
    pub fn list(&self) -> Vec<TemplateItem> {
        let mut items = Self::builtin_items();
        items.extend(self.custom.clone());
        items
    }

    /// 按 id 查找模板，返回 Trait 对象（内置模板按 id 重建，自定义模板包装 TemplateItem）
    pub fn find(&self, id: &str) -> Option<Box<dyn PromptTemplate>> {
        match id {
            "code_gen" => Some(Box::new(CodeGenTemplate { language: "由用户输入指定".into() })),
            "refactor" => Some(Box::new(RefactorTemplate)),
            "debug" => Some(Box::new(DebugTemplate)),
            "code_review" => Some(Box::new(CodeReviewTemplate)),
            "interface_design" => Some(Box::new(InterfaceDesignTemplate)),
            "unit_test" => Some(Box::new(UnitTestTemplate)),
            "requirement_breakdown" => Some(Box::new(RequirementBreakdownTemplate)),
            _ => self
                .custom
                .iter()
                .find(|item| item.id == id)
                .map(|item| Box::new(CustomTemplate { item: item.clone() }) as Box<dyn PromptTemplate>),
        }
    }

    /// 新增或覆盖自定义模板并持久化
    pub fn save_custom(&mut self, mut item: TemplateItem) -> Result<(), String> {
        item.is_builtin = false;
        if let Some(existing) = self.custom.iter_mut().find(|i| i.id == item.id) {
            *existing = item;
        } else {
            self.custom.push(item);
        }
        self.persist()
    }

    /// 删除自定义模板并持久化
    pub fn delete_custom(&mut self, id: &str) -> Result<(), String> {
        self.custom.retain(|i| i.id != id);
        self.persist()
    }

    // ---- 持久化 ----

    fn load_custom(data_path: &PathBuf) -> Vec<TemplateItem> {
        fs::read_to_string(data_path)
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default()
    }

    fn persist(&self) -> Result<(), String> {
        if let Some(parent) = self.data_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("创建模板目录失败: {e}"))?;
        }
        let json = serde_json::to_string_pretty(&self.custom)
            .map_err(|e| format!("模板序列化失败: {e}"))?;
        fs::write(&self.data_path, json).map_err(|e| format!("模板写入失败: {e}"))
    }
}
