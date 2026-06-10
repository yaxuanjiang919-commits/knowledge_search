//! 索引构建、持久化、加载模块
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

use crate::document::Document;
use crate::error::{SearchError, SearchResult};

/// 自定义特征：可构建索引（Trait 核心考点）
pub trait Indexable {
    /// 提取文档关键词，用于构建索引
    fn extract_words(&self) -> Vec<String>;
}

/// 索引条目：单个关键词对应的文档ID列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub doc_ids: Vec<usize>,
}

/// 全局倒排索引
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvertedIndex {
    /// 词 -> 索引条目
    pub word_map: HashMap<String, IndexEntry>,
    /// 文档ID -> 文档路径
    pub doc_list: Vec<PathBuf>,
}

impl InvertedIndex {
    /// 新建空索引
    pub fn new() -> Self {
        Self {
            word_map: HashMap::new(),
            doc_list: Vec::new(),
        }
    }

    /// 批量构建索引（用到 借用、泛型、生命周期）
       pub fn build(&mut self, docs: &[Document]) -> SearchResult<()> {
        self.word_map.clear();
        self.doc_list.clear();

        for (doc_id, doc) in docs.iter().enumerate() {
            // 存入文档路径
            self.doc_list.push(doc.path.clone());
            let words = doc.extract_words();

            // 构建倒排索引
            for word in words {
                let entry = self.word_map.entry(word).or_insert_with(|| IndexEntry {
                    doc_ids: Vec::new()
                });
                if !entry.doc_ids.contains(&doc_id) {
                    entry.doc_ids.push(doc_id);
                }
            }
        }
        Ok(())
    }


    /// 将索引保存到本地文件（持久化）
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> SearchResult<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| SearchError::IndexError(format!("序列化索引失败: {}", e)))?;
        fs::write(path, json)?;
        Ok(())
    }

    /// 从文件加载索引
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> SearchResult<Self> {
        let content = fs::read_to_string(path)?;
        let index = serde_json::from_str(&content)
            .map_err(|e| SearchError::IndexError(format!("反序列化索引失败: {}", e)))?;
        Ok(index)
    }

    /// 根据关键词查询索引
    pub fn search_word(&self, word: &str) -> Vec<&PathBuf> {
        let word = word.to_lowercase();
        self.word_map
            .get(&word)
            .map(|entry| {
                entry.doc_ids
                    .iter()
                    .filter_map(|&id| self.doc_list.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
}

// 为 Document 实现 Indexable 特征（Trait 实现考点）
impl Indexable for Document {
    fn extract_words(&self) -> Vec<String> {
        // 简单分词：按空白分割 + 转小写
        self.content
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect()
    }
}

