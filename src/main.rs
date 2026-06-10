//! 本地知识库搜索系统 - 程序入口
use knowledge_search::cli::parse_cli;
use knowledge_search::document::Document;
use knowledge_search::error::SearchResult;
use knowledge_search::index::InvertedIndex;
use knowledge_search::search::{search_by_keyword, print_all_results};

// 索引持久化文件名称
const INDEX_FILE: &str = "index.json";

fn main() -> SearchResult<()> {
    // 1. 解析命令行参数
    let cli = parse_cli()?;

    // 2. 加载文档（支持单个文件 / 整个目录）
    let docs = if cli.path.is_file() {
        vec![Document::from_file(&cli.path)?]
    } else {
        Document::load_from_dir(&cli.path)?
    };
    println!("📚 成功加载 {} 份文档", docs.len());

    // 3. 加载/重建索引
    let mut index = InvertedIndex::new();
    if cli.rebuild || !std::path::Path::new(INDEX_FILE).exists() {
        println!("🔨 开始构建索引...");
        index.build(&docs)?;
        index.save_to_file(INDEX_FILE)?;
        println!("✅ 索引构建完成，已保存至 {}", INDEX_FILE);
    } else {
        println!("📂 加载已有索引...");
        index = InvertedIndex::load_from_file(INDEX_FILE)?;
    }

    // 4. 执行搜索（传入关键词则检索）
    if let Some(keyword) = cli.keyword {
        println!("\n🔍 正在搜索关键词：【{}】", keyword);
        let res = search_by_keyword(&index, &keyword)?;
        print_all_results(&res);
    }

    Ok(())
}
