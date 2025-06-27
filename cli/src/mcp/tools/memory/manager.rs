use anyhow::Result;
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};

use super::types::{MemoryEntry, MemoryCategory, MemoryMetadata};

/// 记忆管理器
pub struct MemoryManager {
    memory_dir: PathBuf,
    project_path: String,
}

impl MemoryManager {
    /// 创建新的记忆管理器
    pub fn new(project_path: &str) -> Result<Self> {
        // 规范化项目路径
        let normalized_path = Self::normalize_project_path(project_path)?;
        let memory_dir = normalized_path.join(".cunzhi-memory");

        // 创建记忆目录，如果失败则说明项目不适合使用记忆功能
        fs::create_dir_all(&memory_dir)
            .map_err(|e| anyhow::anyhow!(
                "无法在项目中创建记忆目录: {}\n错误: {}\n这可能是因为项目目录没有写入权限。",
                memory_dir.display(),
                e
            ))?;

        let manager = Self {
            memory_dir,
            project_path: normalized_path.to_string_lossy().to_string(),
        };

        // 初始化记忆文件结构
        manager.initialize_memory_structure()?;

        Ok(manager)
    }

    /// 规范化项目路径
    fn normalize_project_path(project_path: &str) -> Result<PathBuf> {
        let path = Path::new(project_path);

        // 转换为绝对路径
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };

        // 规范化路径
        let canonical_path = absolute_path.canonicalize()
            .map_err(|e| anyhow::anyhow!("项目路径不存在或无法访问: {}\n错误: {}", project_path, e))?;

        Ok(canonical_path)
    }

    /// 初始化记忆文件结构
    fn initialize_memory_structure(&self) -> Result<()> {
        // 创建分类目录
        for category in &["rules", "preferences", "patterns", "context"] {
            let category_dir = self.memory_dir.join(category);
            fs::create_dir_all(&category_dir)?;
        }

        // 创建或更新元数据文件
        let metadata_file = self.memory_dir.join("metadata.json");
        if !metadata_file.exists() {
            let metadata = MemoryMetadata {
                project_path: self.project_path.clone(),
                last_organized: Utc::now(),
                total_entries: 0,
                version: env!("CARGO_PKG_VERSION").to_string(),
            };
            let metadata_json = serde_json::to_string_pretty(&metadata)?;
            fs::write(&metadata_file, metadata_json)?;
        }

        Ok(())
    }

    /// 添加记忆条目
    pub fn add_memory(&self, content: &str, category: MemoryCategory) -> Result<String> {
        let entry_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        let entry = MemoryEntry {
            id: entry_id.clone(),
            content: content.to_string(),
            category,
            created_at: now,
            updated_at: now,
        };

        // 保存到对应分类目录
        let category_name = match category {
            MemoryCategory::Rule => "rules",
            MemoryCategory::Preference => "preferences",
            MemoryCategory::Pattern => "patterns",
            MemoryCategory::Context => "context",
        };

        let entry_file = self.memory_dir
            .join(category_name)
            .join(format!("{}.json", entry_id));

        let entry_json = serde_json::to_string_pretty(&entry)?;
        fs::write(&entry_file, entry_json)?;

        // 更新元数据
        self.update_metadata()?;

        Ok(entry_id)
    }

    /// 获取所有记忆条目
    pub fn get_all_memories(&self) -> Result<Vec<MemoryEntry>> {
        let mut memories = Vec::new();

        for category_name in &["rules", "preferences", "patterns", "context"] {
            let category_dir = self.memory_dir.join(category_name);
            if !category_dir.exists() {
                continue;
            }

            for entry in fs::read_dir(&category_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    let content = fs::read_to_string(&path)?;
                    if let Ok(memory) = serde_json::from_str::<MemoryEntry>(&content) {
                        memories.push(memory);
                    }
                }
            }
        }

        // 按创建时间排序
        memories.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(memories)
    }

    /// 按分类获取记忆条目
    pub fn get_memories_by_category(&self, category: MemoryCategory) -> Result<Vec<MemoryEntry>> {
        let category_name = match category {
            MemoryCategory::Rule => "rules",
            MemoryCategory::Preference => "preferences",
            MemoryCategory::Pattern => "patterns",
            MemoryCategory::Context => "context",
        };

        let category_dir = self.memory_dir.join(category_name);
        let mut memories = Vec::new();

        if !category_dir.exists() {
            return Ok(memories);
        }

        for entry in fs::read_dir(&category_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)?;
                if let Ok(memory) = serde_json::from_str::<MemoryEntry>(&content) {
                    memories.push(memory);
                }
            }
        }

        // 按创建时间排序
        memories.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(memories)
    }

    /// 更新元数据
    fn update_metadata(&self) -> Result<()> {
        let metadata_file = self.memory_dir.join("metadata.json");
        let total_entries = self.get_all_memories()?.len();

        let metadata = MemoryMetadata {
            project_path: self.project_path.clone(),
            last_organized: Utc::now(),
            total_entries,
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        fs::write(&metadata_file, metadata_json)?;

        Ok(())
    }

    /// 获取项目信息摘要
    pub fn get_project_summary(&self) -> Result<String> {
        let memories = self.get_all_memories()?;
        
        if memories.is_empty() {
            return Ok(format!("项目路径: {}\n暂无记忆条目", self.project_path));
        }

        let mut summary = format!("项目路径: {}\n总记忆条目: {}\n\n", self.project_path, memories.len());

        // 按分类统计
        let mut rule_count = 0;
        let mut preference_count = 0;
        let mut pattern_count = 0;
        let mut context_count = 0;

        for memory in &memories {
            match memory.category {
                MemoryCategory::Rule => rule_count += 1,
                MemoryCategory::Preference => preference_count += 1,
                MemoryCategory::Pattern => pattern_count += 1,
                MemoryCategory::Context => context_count += 1,
            }
        }

        summary.push_str(&format!("分类统计:\n"));
        summary.push_str(&format!("- 规范规则: {} 条\n", rule_count));
        summary.push_str(&format!("- 用户偏好: {} 条\n", preference_count));
        summary.push_str(&format!("- 最佳实践: {} 条\n", pattern_count));
        summary.push_str(&format!("- 项目上下文: {} 条\n\n", context_count));

        // 显示最近的几条记忆
        summary.push_str("最近记忆:\n");
        for (i, memory) in memories.iter().take(5).enumerate() {
            summary.push_str(&format!("{}. [{}] {}\n", 
                i + 1, 
                match memory.category {
                    MemoryCategory::Rule => "规范",
                    MemoryCategory::Preference => "偏好",
                    MemoryCategory::Pattern => "实践",
                    MemoryCategory::Context => "上下文",
                },
                memory.content.chars().take(50).collect::<String>()
            ));
        }

        Ok(summary)
    }
}
