#!/bin/bash

# 智能文档生成工具 - 启动脚本
# 这个脚本帮助您快速开始使用智能文档生成工具

set -e  # 遇到错误时退出

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 函数：打印带颜色的消息
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 函数：检查命令是否存在
check_command() {
    if ! command -v $1 &> /dev/null; then
        print_error "命令 '$1' 未找到，请先安装"
        exit 1
    fi
}

# 函数：显示帮助信息
show_help() {
    echo "智能文档生成工具 - 启动脚本"
    echo ""
    echo "使用方法:"
    echo "  ./start.sh [命令]"
    echo ""
    echo "命令:"
    echo "  setup       设置开发环境"
    echo "  build       构建项目"
    echo "  test        运行测试"
    echo "  run         运行程序"
    echo "  clean       清理构建文件"
    echo "  docs        生成文档"
    echo "  example     运行示例"
    echo "  all         执行所有步骤（setup -> build -> test -> run）"
    echo "  help        显示此帮助信息"
    echo ""
    echo "示例:"
    echo "  ./start.sh setup     # 设置开发环境"
    echo "  ./start.sh all       # 执行完整流程"
    echo "  ./start.sh           # 显示帮助信息"
}

# 函数：设置开发环境
setup_environment() {
    print_info "设置开发环境..."

    # 检查 Rust 是否已安装
    if ! command -v rustc &> /dev/null; then
        print_warning "Rust 未安装，正在安装..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
        print_success "Rust 安装完成"
    else
        print_success "Rust 已安装: $(rustc --version)"
    fi

    # 安装开发工具
    print_info "安装开发工具..."
    rustup component add rustfmt clippy rust-analyzer

    # 安装 cargo 扩展
    print_info "安装 cargo 扩展..."
    cargo install cargo-watch 2>/dev/null || print_warning "cargo-watch 安装失败或已安装"
    cargo install cargo-tarpaulin 2>/dev/null || print_warning "cargo-tarpaulin 安装失败或已安装"

    # 创建必要的目录
    print_info "创建项目目录..."
    mkdir -p templates output logs examples/data

    # 创建示例文件
    print_info "创建示例文件..."

    # 创建示例模板
    cat > templates/example_template.txt << 'EOF'
# {{title}}

作者: {{author}}
日期: {{date}}

{{content}}

## 章节列表
{% for section in sections %}
- {{section}}
{% endfor %}

---
生成时间: {{now}}
EOF

    # 创建示例数据
    cat > examples/data/sample.json << 'EOF'
{
    "title": "示例文档",
    "author": "智能文档工具",
    "date": "2024-01-15",
    "content": "这是一个通过智能文档生成工具创建的示例文档。\n\n该工具支持多种文档格式和模板引擎，可以快速生成专业的文档。",
    "sections": [
        "介绍",
        "功能特点",
        "使用方法",
        "总结"
    ]
}
EOF

    print_success "开发环境设置完成！"
}

# 函数：构建项目
build_project() {
    print_info "构建项目..."

    # 检查依赖
    print_info "检查依赖..."
    cargo check

    # 构建项目
    print_info "编译项目..."
    cargo build

    # 构建发布版本
    print_info "编译发布版本..."
    cargo build --release

    print_success "项目构建完成！"
    echo "  调试版本: target/debug/smart-doc"
    echo "  发布版本: target/release/smart-doc"
}

# 函数：运行测试
run_tests() {
    print_info "运行测试..."

    # 运行单元测试
    print_info "运行单元测试..."
    cargo test --lib

    # 运行集成测试
    print_info "运行集成测试..."
    cargo test --tests

    # 运行所有测试
    print_info "运行所有测试..."
    cargo test

    print_success "测试完成！"
}

# 函数：运行程序
run_program() {
    print_info "运行程序..."

    # 检查是否已构建
    if [ ! -f "target/debug/smart-doc" ]; then
        print_warning "程序未构建，正在构建..."
        cargo build
    fi

    # 显示帮助信息
    echo ""
    print_info "显示帮助信息:"
    ./target/debug/smart-doc --help

    # 运行示例命令
    echo ""
    print_info "运行示例命令:"

    # 列出模板
    ./target/debug/smart-doc list || true

    # 生成示例文档
    echo ""
    print_info "生成示例文档..."
    ./target/debug/smart-doc generate \
        --template templates/example_template.txt \
        --data examples/data/sample.json \
        --output output/example_document.txt \
        --format txt || print_warning "文档生成失败（这是预期的，因为模板引擎尚未完全实现）"

    print_success "程序运行完成！"
}

# 函数：清理构建文件
clean_project() {
    print_info "清理构建文件..."

    cargo clean

    # 清理输出目录
    if [ -d "output" ]; then
        rm -rf output/*
        print_info "已清理输出目录"
    fi

    print_success "清理完成！"
}

# 函数：生成文档
generate_docs() {
    print_info "生成文档..."

    # 生成 Rust 文档
    cargo doc --no-deps

    # 打开文档
    if [[ "$OSTYPE" == "darwin"* ]]; then
        open target/doc/smart_doc_core/index.html
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        xdg-open target/doc/smart_doc_core/index.html 2>/dev/null || \
        print_info "文档已生成: file://$(pwd)/target/doc/smart_doc_core/index.html"
    else
        print_info "文档已生成: file://$(pwd)/target/doc/smart_doc_core/index.html"
    fi

    print_success "文档生成完成！"
}

# 函数：运行示例
run_example() {
    print_info "运行示例..."

    # 检查是否已构建
    if [ ! -f "target/debug/smart-doc" ]; then
        print_warning "程序未构建，正在构建..."
        cargo build
    fi

    # 运行示例
    cargo run --example basic_usage

    print_success "示例运行完成！"
}

# 函数：执行所有步骤
run_all() {
    print_info "开始执行完整流程..."
    echo ""

    setup_environment
    echo ""

    build_project
    echo ""

    run_tests
    echo ""

    run_program
    echo ""

    print_success "所有步骤执行完成！"
    echo ""
    echo "下一步建议:"
    echo "  1. 阅读 README.md 了解项目详情"
    echo "  2. 查看 docs/ 目录中的开发文档"
    echo "  3. 运行 ./start.sh docs 查看API文档"
    echo "  4. 开始编写你的第一个功能！"
}

# 主程序
main() {
    # 显示标题
    echo ""
    echo "========================================"
    echo "  智能文档生成工具 - 启动脚本"
    echo "========================================"
    echo ""

    # 如果没有参数，显示帮助
    if [ $# -eq 0 ]; then
        show_help
        exit 0
    fi

    # 解析命令
    case $1 in
        setup)
            setup_environment
            ;;
        build)
            build_project
            ;;
        test)
            run_tests
            ;;
        run)
            run_program
            ;;
        clean)
            clean_project
            ;;
        docs)
            generate_docs
            ;;
        example)
            run_example
            ;;
        all)
            run_all
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            print_error "未知命令: $1"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

# 运行主程序
main "$@"
