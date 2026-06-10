//! 文档实体与文件读取模块
use std::fs;
use std::path::{Path, PathBuf};
use crate::error::{SearchError, SearchResult};

/// 文档类型枚举（enum 拓展考点）
#[derive(Debug, Clone, PartialEq)]
pub enum DocType {
    Txt,
    Markdown,
}

/// 文档结构体（struct 核心考点）
#[derive(Debug, Clone)]
pub struct Document {
    /// 文档完整路径
    pub path: PathBuf,
    /// 文档类型
    pub doc_type: DocType,
    /// 文档原始内容
    pub content: String,
    /// 文档文件名
    pub name: String,
}

impl Document {
    /// 从单个文件加载文档
    pub fn from_file(path: impl AsRef<Path>) -> SearchResult<Self> {
        let path = path.as_ref();
        // 校验路径是否存在
        if !path.exists() {
            return Err(SearchError::PathNotFound(path.to_path_buf()));
        }

        // 判断文件格式
        let doc_type = match path.extension().and_then(|s| s.to_str()) {
            Some("txt") => DocType::Txt,
            Some("md") => DocType::Markdown,
            _ => return Err(SearchError::UnsupportedFormat(path.to_path_buf())),
        };

        // 读取文件内容
        let content = fs::read_to_string(path)?;
        if content.trim().is_empty() {
            return Err(SearchError::EmptyDocument);
        }

        // 获取文件名
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(Self {
            path: path.to_path_buf(),
            doc_type,
            content,
            name,
        })
    }

    /// 从目录批量加载支持的文档
    pub fn load_from_dir(dir: impl AsRef<Path>) -> SearchResult<Vec<Self>> {
        let dir = dir.as_ref();
        if !dir.is_dir() {
            return Err(SearchError::PathNotFound(dir.to_path_buf()));
        }

        let mut docs = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            // 只处理文件，跳过子目录
            if path.is_file() {
                if let Ok(doc) = Self::from_file(&path) {
                    docs.push(doc);
                }
            }
        }
        Ok(docs)
    }

    /// 获取文档内容引用（借用 &str，所有权/借用考点）
    pub fn content_ref(&self) -> &str {
        &self.content
    }
}
