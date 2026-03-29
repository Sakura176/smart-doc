#!/bin/bash

# 智能文档生成工具 - 示例测试脚本（简化版）
# 只进行基本文件检查和语法验证

set -e

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

# 主函数
main() {
    echo ""
    echo "========================================"
    echo "  智能文档生成工具 - 示例文件基本检查"
    echo "========================================"
    echo ""

    print_info "开始检查示例文件..."
    echo ""

    local errors=0
    local warnings=0

    # 1. 检查示例文件是否存在
    print_info "1. 检查示例文件是否存在"
    echo "----------------------------------------"

    # 定义要检查的文件列表
    declare -A files=(
        ["basic_usage.rs"]="Rust代码示例"
        ["config.yaml"]="配置文件示例"
        ["simple_template.txt"]="复杂模板示例"
        ["basic_template.txt"]="简单模板示例"
        ["sample_data.json"]="复杂数据示例"
        ["basic_data.json"]="简单数据示例"
        ["README.md"]="示例说明文档"
    )

    for file in "${!files[@]}"; do
        description="${files[$file]}"
        if [ -f "examples/$file" ]; then
            print_success "✓ $description: examples/$file"
        else
            print_error "✗ $description: examples/$file 不存在"
            ((errors++))
        fi
    done

    echo ""

    # 2. 检查JSON文件语法
    print_info "2. 检查JSON文件语法"
    echo "----------------------------------------"

    # 检查JSON文件
    json_files=("examples/sample_data.json" "examples/basic_data.json")

    for json_file in "${json_files[@]}"; do
        if [ -f "$json_file" ]; then
            # 使用简单的语法检查
            if python3 -c "import json; json.load(open('$json_file'))" 2>/dev/null; then
                print_success "✓ JSON语法正确: $json_file"
            else
                print_error "✗ JSON语法错误: $json_file"
                ((errors++))
            fi
        fi
    done

    echo ""

    # 3. 检查模板文件基本结构
    print_info "3. 检查模板文件基本结构"
    echo "----------------------------------------"

    template_files=("examples/simple_template.txt" "examples/basic_template.txt")

    for template_file in "${template_files[@]}"; do
        if [ -f "$template_file" ]; then
            file_size=$(wc -c < "$template_file")
            if [ "$file_size" -gt 0 ]; then
                print_success "✓ 模板文件非空: $template_file ($file_size 字节)"

                # 检查是否包含模板变量
                if grep -q '{{' "$template_file"; then
                    var_count=$(grep -o '{{[^}]*}}' "$template_file" | wc -l)
                    print_info "  包含 $var_count 个模板变量"
                else
                    print_warning "⚠ 未找到模板变量: $template_file"
                    ((warnings++))
                fi
            else
                print_error "✗ 模板文件为空: $template_file"
                ((errors++))
            fi
        fi
    done

    echo ""

    # 4. 总结
    print_info "4. 检查总结"
    echo "----------------------------------------"

    if [ $errors -eq 0 ] && [ $warnings -eq 0 ]; then
        print_success "所有检查通过！"
        echo ""
        print_info "示例文件状态:"
        echo "  - 所有文件都存在"
        echo "  - JSON文件语法正确"
        echo "  - 模板文件结构正常"
    elif [ $errors -eq 0 ]; then
        print_success "基本检查通过，但有 $warnings 个警告"
        echo ""
        print_info "警告说明:"
        echo "  - 某些模板文件可能没有使用模板变量"
        echo "  - 这是正常的，取决于模板设计"
    else
        print_error "检查失败：$errors 个错误，$warnings 个警告"
        echo ""
        print_info "需要修复的问题:"
        echo "  - 检查缺失的文件"
        echo "  - 修复JSON语法错误"
        echo "  - 确保模板文件不为空"
    fi

    echo ""
    print_info "检查完成时间: $(date '+%Y-%m-%d %H:%M:%S')"

    # 返回状态
    if [ $errors -gt 0 ]; then
        exit 1
    else
        exit 0
    fi
}

# 显示帮助信息
show_help() {
    echo "智能文档生成工具 - 示例文件基本检查脚本"
    echo ""
    echo "使用方法:"
    echo "  ./test_examples.sh"
    echo ""
    echo "功能:"
    echo "  1. 检查所有示例文件是否存在"
    echo "  2. 验证JSON文件语法"
    echo "  3. 检查模板文件基本结构"
    echo "  4. 生成检查报告"
    echo ""
    echo "环境要求:"
    echo "  - bash"
    echo "  - python3 (用于JSON验证)"
    echo ""
    echo "注意:"
    echo "  - 此脚本只进行基本文件检查"
    echo "  - 不运行Rust代码或测试CLI功能"
    echo "  - 需要从项目根目录运行"
}

# 检查是否在项目根目录
if [ ! -f "Cargo.toml" ]; then
    print_error "错误：请在项目根目录运行此脚本"
    echo ""
    print_info "项目根目录应该包含 Cargo.toml 文件"
    echo "当前目录: $(pwd)"
    exit 1
fi

# 解析命令行参数
if [[ "$1" == "-h" ]] || [[ "$1" == "--help" ]]; then
    show_help
    exit 0
fi

# 运行主函数
main
