//! 命令行参数解析模块
use clap::Parser;
use std::path::PathBuf;
use crate::error::{SearchError, SearchResult};

/// 知识库搜索系统命令行参数
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// 文档目录/单个文档路径
    #[arg(short, long, required = true)]
    pub path: PathBuf,

    /// 搜索关键词
    #[arg(short, long)]
    pub keyword: Option<String>,

    /// 重建索引（忽略旧索引文件）
    #[arg(short, long, default_value_t = false)]
    pub rebuild: bool,
}

/// 解析并校验命令行参数
pub fn parse_cli() -> SearchResult<Cli> {
    let cli = Cli::parse();

    // 基础参数校验
    if cli.keyword.as_ref().map(|k| k.trim().is_empty()).unwrap_or(false) {
        return Err(SearchError::InvalidArg("搜索关键词不能为空".to_string()));
    }

    Ok(cli)
}
