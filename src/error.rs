//! 自定义错误模块
use std::fmt;
use std::io;
use std::path::PathBuf;

/// 系统统一错误枚举（enum 核心考点）
#[derive(Debug)]
pub enum SearchError {
    /// 文件读写错误
    Io(io::Error),
    /// 路径不存在
    PathNotFound(PathBuf),
    /// 不支持的文件格式
    UnsupportedFormat(PathBuf),
    /// 索引解析/序列化失败
    IndexError(String),
    /// 命令行参数非法
    InvalidArg(String),
    /// 文档内容为空
    EmptyDocument,
}

// 实现 Display 特征（trait 考点），用于友好打印错误信息
impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchError::Io(e) => write!(f, "文件IO错误：{}", e),
            SearchError::PathNotFound(p) => write!(f, "路径不存在：{}", p.display()),
            SearchError::UnsupportedFormat(p) => write!(f, "不支持的文件格式：{}", p.display()),
            SearchError::IndexError(msg) => write!(f, "索引操作失败：{}", msg),
            SearchError::InvalidArg(msg) => write!(f, "参数错误：{}", msg),
            SearchError::EmptyDocument => write!(f, "文档内容为空，无法建立索引"),
        }
    }
}

// 实现标准库 Error 特征，兼容标准错误链路
impl std::error::Error for SearchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SearchError::Io(e) => Some(e),
            _ => None,
        }
    }
}

// 错误类型自动转换：io::Error → SearchError（简化错误传播）
impl From<io::Error> for SearchError {
    fn from(e: io::Error) -> Self {
        SearchError::Io(e)
    }
}

/// 全局别名，简化代码书写
pub type SearchResult<T> = Result<T, SearchError>;
