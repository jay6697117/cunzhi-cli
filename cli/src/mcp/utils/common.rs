/// 生成请求ID
pub fn generate_request_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// 验证项目路径
pub fn validate_project_path(path: &str) -> anyhow::Result<()> {
    let path = std::path::Path::new(path);
    if !path.exists() {
        return Err(anyhow::anyhow!("项目路径不存在: {}", path.display()));
    }
    if !path.is_dir() {
        return Err(anyhow::anyhow!("项目路径不是目录: {}", path.display()));
    }
    Ok(())
}
