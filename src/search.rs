//! 搜索逻辑与结果处理模块
use std::path::PathBuf;
use crate::index::InvertedIndex;
use crate::error::{SearchError, SearchResult};

/// 搜索结果结构体（struct 考点）
#[derive(Debug, Clone)]
pub struct SearchResultItem {
    /// 匹配的文档路径
    pub file_path: PathBuf,
}

impl SearchResultItem {
    /// 格式化输出单条结果
    pub fn print(&self) {
        println!("📄 命中文档：{}", self.file_path.display());
    }
}

/// 执行关键词搜索
pub fn search_by_keyword(index: &InvertedIndex, keyword: &str) -> SearchResult<Vec<SearchResultItem>> {
    if keyword.trim().is_empty() {
        return Err(SearchError::InvalidArg("搜索关键词不能为空".to_string()));
    }

    // 从索引中查询匹配路径
    let path_list = index.search_word(keyword);
    let mut result = Vec::new();

    for path in path_list {
        result.push(SearchResultItem {
            file_path: path.clone()
        });
    }

    Ok(result)
}

/// 批量打印所有搜索结果
pub fn print_all_results(items: &[SearchResultItem]) {
    if items.is_empty() {
        println!("🔍 未找到匹配内容");
        return;
    }

    println!("=====================================");
    println!("✅ 共找到 {} 条匹配结果", items.len());
    println!("=====================================");
    for item in items {
        item.print();
    }
    println!("=====================================");
}
