//! 智能文档生成工具 - 主程序入口
//!
//! 这是一个基于 Rust 的个人学习项目，旨在通过开发文档生成工具来学习 Rust 编程。

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;

use smart_doc_core::cli;

/// 命令行参数定义
#[derive(Parser)]
#[command(name = "smart-doc")]
#[command(about = "智能文档生成工具", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// 子命令定义
#[derive(Subcommand)]
enum Commands {
    /// 生成文档
    Generate {
        /// 模板文件路径
        #[arg(short, long)]
        template: String,

        /// 数据文件路径（JSON格式）
        #[arg(short, long)]
        data: String,

        /// 输出文件路径
        #[arg(short, long)]
        output: String,

        /// 输出格式 [可选，默认: docx]
        #[arg(short, long, default_value = "docx")]
        format: String,
    },

    /// 查看模板信息
    Info {
        /// 模板文件路径
        #[arg(short, long)]
        template: String,
    },

    /// 验证模板语法
    Validate {
        /// 模板文件路径
        #[arg(short, long)]
        template: String,
    },

    /// 列出可用模板
    List,

    /// 查看版本信息
    Version,
}

/// 主函数
fn main() -> Result<()> {
    // 初始化日志系统
    env_logger::init();

    info!("启动智能文档生成工具 v{}", env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            template,
            data,
            output,
            format,
        } => {
            info!("开始生成文档");
            info!("模板: {}", template);
            info!("数据: {}", data);
            info!("输出: {}", output);
            info!("格式: {}", format);

            // 调用生成逻辑
            cli::generate_document(&template, &data, &output, &format)?;

            info!("文档生成成功: {}", output);
            Ok(())
        }

        Commands::Info { template } => {
            info!("查看模板信息: {}", template);
            cli::show_template_info(&template)?;
            Ok(())
        }

        Commands::Validate { template } => {
            info!("验证模板: {}", template);
            cli::validate_template(&template)?;
            info!("模板验证通过");
            Ok(())
        }

        Commands::List => {
            info!("列出可用模板");
            cli::list_templates()?;
            Ok(())
        }

        Commands::Version => {
            println!("smart-doc v{}", env!("CARGO_PKG_VERSION"));
            println!("作者: {}", env!("CARGO_PKG_AUTHORS"));
            println!("仓库: {}", env!("CARGO_PKG_REPOSITORY"));
            Ok(())
        }
    }
}
