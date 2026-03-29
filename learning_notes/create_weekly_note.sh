#!/bin/bash

# 智能文档生成工具 - 每周笔记自动生成脚本
# 根据当前时间自动生成文件名和内容

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

# 函数：显示帮助信息
show_help() {
    echo "智能文档生成工具 - 每周笔记自动生成脚本"
    echo ""
    echo "使用方法:"
    echo "  ./create_weekly_note.sh [选项]"
    echo ""
    echo "选项:"
    echo "  -w, --week <数字>     指定周数（可选，默认使用当前周数）"
    echo "  -y, --year <数字>     指定年份（可选，默认当前年份）"
    echo "  -m, --month <数字>    指定月份（可选，默认当前月份）"
    echo "  -t, --template <文件> 指定模板文件（可选）"
    echo "  -o, --output <文件>   指定输出文件（可选）"
    echo "  -h, --help            显示此帮助信息"
    echo ""
    echo "示例:"
    echo "  ./create_weekly_note.sh                    # 自动创建当前周笔记"
    echo "  ./create_weekly_note.sh -w 1              # 创建第1周笔记"
    echo "  ./create_weekly_note.sh -w 2 -y 2024      # 创建2024年第2周笔记"
    echo "  ./create_weekly_note.sh -o my_note.md     # 自动创建并指定输出文件"
    echo ""
    echo "模板文件:"
    echo "  默认使用: weekly_goal_template.md"
}

# 函数：获取当前年份
get_current_year() {
    date +%Y
}

# 函数：获取当前周数（ISO周数）
get_current_week() {
    date +%V
}

# 函数：获取当前月份
get_current_month() {
    date +%m
}

# 函数：获取当前日期
get_current_date() {
    date +%Y-%m-%d
}

# 函数：获取当前日期时间
get_current_datetime() {
    date +"%Y-%m-%d %H:%M"
}

# 函数：根据年份和周数计算日期范围
calculate_date_range() {
    local week=$1
    local year=$2

    # 使用ISO周数计算日期范围
    # 计算该年的第一个周四（ISO周定义：第一周包含该年的第一个周四）
    local first_thursday=$(date -d "$year-01-01 +$(( (4 - $(date -d "$year-01-01" +%u) + 7) % 7 )) days" +%Y-%m-%d)

    # 计算该周的第一天（周一）
    local monday=$(date -d "$first_thursday +$(( (week - 1) * 7 - 3 )) days" +%Y-%m-%d)

    # 计算该周的最后一天（周日）
    local sunday=$(date -d "$monday +6 days" +%Y-%m-%d)

    echo "$monday 至 $sunday"
}

# 函数：根据日期获取月份
get_month_from_date() {
    local date_str=$1
    date -d "$date_str" +%m 2>/dev/null || echo "01"
}

# 函数：根据年份和周数获取主要月份
get_month_from_week() {
    local week=$1
    local year=$2
    local date_range=$(calculate_date_range "$week" "$year")

    # 提取周一日期
    local monday=$(echo "$date_range" | cut -d' ' -f1)

    # 获取月份
    get_month_from_date "$monday"
}

# 函数：验证周数
validate_week() {
    local week=$1
    if ! [[ "$week" =~ ^[0-9]+$ ]]; then
        print_error "周数必须是数字: $week"
        exit 1
    fi

    if [ "$week" -lt 1 ] || [ "$week" -gt 53 ]; then
        print_warning "周数 $week 可能不在有效范围内 (1-53)"
    fi
}

# 函数：验证年份
validate_year() {
    local year=$1
    if ! [[ "$year" =~ ^[0-9]{4}$ ]]; then
        print_error "年份必须是4位数字: $year"
        exit 1
    fi

    local current_year=$(get_current_year)
    if [ "$year" -lt 2020 ] || [ "$year" -gt $((current_year + 1)) ]; then
        print_warning "年份 $year 可能不在合理范围内"
    fi
}

# 函数：验证月份
validate_month() {
    local month=$1
    if ! [[ "$month" =~ ^[0-9]+$ ]]; then
        print_error "月份必须是数字: $month"
        exit 1
    fi

    if [ "$month" -lt 1 ] || [ "$month" -gt 12 ]; then
        print_error "月份必须在1-12之间: $month"
        exit 1
    fi
}

# 函数：检查模板文件
check_template() {
    local template=$1
    if [ ! -f "$template" ]; then
        print_error "模板文件不存在: $template"
        echo "可用的模板文件:"
        ls -la *.md | grep -E "(template|Template)" || echo "  没有找到模板文件"
        exit 1
    fi
}

# 函数：生成自动文件名
generate_auto_filename() {
    local week=$1
    local year=$2
    local month=$3

    # 根据周数计算该周的主要月份
    local week_month=$(get_month_from_week "$week" "$year")

    # 格式：YYYY-MM_week_W.md (使用该周对应的月份)
    echo "${year}-${week_month}_week_${week}.md"
}

# 函数：生成笔记内容
generate_note_content() {
    local template=$1
    local week=$2
    local year=$3
    local month=$4
    local date_range=$5

    # 读取模板内容
    local content=$(cat "$template")

    # 替换占位符
    content="${content//\[X\]/$week}"  # 周数
    content="${content//\[Y\]/$month}" # 月份
    content="${content//\[YYYY-MM-DD\] 至 \[YYYY-MM-DD\]/$date_range}"
    content="${content//\[YYYY-MM-DD HH:MM\]/$(get_current_datetime)}"

    # 替换年份相关占位符
    content="${content//2024/$year}"
    content="${content//2025/$year}"
    content="${content//2026/$year}"

    # 替换其他常见占位符
    content="${content//\[提交次数\]/0}"
    content="${content//\[学习时长\]/0}"
    content="${content//\[总学习时长\]/0小时}"

    # 添加文件头注释
    local header="# 智能文档生成工具 - ${year}年第${week}周学习笔记\n"
    header+="# 自动生成于: $(get_current_datetime)\n"
    header+="# 日期范围: $date_range\n"
    header+="# 模板: $(basename "$template")\n\n"

    echo -e "${header}${content}"
}

# 函数：检查文件是否存在
check_file_exists() {
    local file=$1
    if [ -f "$file" ]; then
        print_warning "文件 $file 已存在"
        read -p "是否覆盖？(y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_info "操作已取消"
            exit 0
        fi
        print_warning "将覆盖已存在的文件: $file"
    fi
}

# 函数：显示当前时间信息
show_current_time_info() {
    local current_year=$(get_current_year)
    local current_week=$(get_current_week)
    local current_month=$(get_current_month)
    local current_date=$(get_current_date)

    print_info "当前时间信息:"
    echo "  日期: $current_date"
    echo "  年份: $current_year"
    echo "  月份: $current_month"
    echo "  周数: $current_week (ISO周)"
}

# 主函数
main() {
    # 显示标题
    echo ""
    echo "========================================"
    echo "  智能文档生成工具 - 每周笔记自动生成"
    echo "========================================"
    echo ""

    # 显示当前时间信息
    show_current_time_info
    echo ""

    # 默认值 - 使用当前时间
    local week=$(get_current_week)
    local year=$(get_current_year)
    local month=$(get_current_month)
    local template="weekly_goal_template.md"
    local output=""

    # 解析命令行参数
    while [[ $# -gt 0 ]]; do
        case $1 in
            -w|--week)
                week="$2"
                shift 2
                ;;
            -y|--year)
                year="$2"
                shift 2
                ;;
            -m|--month)
                month="$2"
                shift 2
                ;;
            -t|--template)
                template="$2"
                shift 2
                ;;
            -o|--output)
                output="$2"
                shift 2
                ;;
            -h|--help)
                show_help
                exit 0
                ;;
            *)
                print_error "未知选项: $1"
                show_help
                exit 1
                ;;
        esac
    done

    # 验证参数
    validate_week "$week"
    validate_year "$year"
    validate_month "$month"
    check_template "$template"

    # 计算日期范围
    local date_range=$(calculate_date_range "$week" "$year")
    print_info "日期范围: $date_range"

    # 根据周数计算正确的月份（覆盖用户输入的月份）
    local correct_month=$(get_month_from_week "$week" "$year")
    if [ "$month" != "$correct_month" ]; then
        print_info "调整月份: $month → $correct_month (基于周数计算)"
        month="$correct_month"
    fi

    # 确定输出文件名
    if [ -z "$output" ]; then
        output=$(generate_auto_filename "$week" "$year" "$month")
    fi

    print_info "输出文件: $output"

    # 检查文件是否存在
    check_file_exists "$output"

    # 生成笔记内容
    print_info "正在生成笔记..."
    local note_content=$(generate_note_content "$template" "$week" "$year" "$month" "$date_range")

    # 保存到文件
    echo -e "$note_content" > "$output"

    # 验证文件
    if [ -f "$output" ]; then
        local line_count=$(wc -l < "$output")
        local file_size=$(wc -c < "$output")

        print_success "笔记生成成功！"
        echo ""
        echo "📋 文件信息:"
        echo "  文件: $output"
        echo "  行数: $line_count"
        echo "  大小: $file_size 字节"
        echo "  年份: $year"
        echo "  月份: $month (基于周数自动计算)"
        echo "  周数: $week"
        echo "  日期范围: $date_range"
        echo ""

        # 显示文件预览
        print_info "📄 文件预览:"
        echo "--------------------------------------------------"
        head -15 "$output" | sed 's/^/  /'

        if [ $line_count -gt 15 ]; then
            echo "  ... (共 $line_count 行)"
        fi
        echo "--------------------------------------------------"
    else
        print_error "文件创建失败: $output"
        exit 1
    fi
}

# 运行主函数
main "$@"

# 使用说明
echo ""
print_info "📝 使用说明:"
echo "  1. 编辑生成的笔记文件: vim $output"
echo "  2. 填写本周的学习目标和计划"
echo "  3. 在学习过程中更新完成情况"
echo "  4. 周末进行总结和反思"
echo ""
print_info "🔍 相关命令:"
echo "  # 查看所有笔记"
echo "  ls -la *_week_*.md"
echo ""
echo "  # 按日期排序查看笔记"
echo "  ls -la *_week_*.md | sort"
echo ""
echo "  # 搜索笔记内容"
echo "  grep -r \"所有权\" *_week_*.md"
echo ""
echo "  # 统计笔记数量"
echo "  find . -name \"*_week_*.md\" | wc -l"
echo ""
print_success "🚀 开始记录你的学习之旅吧！"
echo ""
