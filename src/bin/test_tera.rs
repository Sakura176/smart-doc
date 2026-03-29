//! 测试 tera::Error 结构的详细分析

use std::error::Error as StdError;
use tera::{Context, Tera};

fn main() {
    println!("=== 测试 tera::Error 结构 ===");

    // 测试1: 使用 tera::Error::msg 关联函数
    println!("\n1. 测试 tera::Error::msg() 关联函数:");
    let error1 = tera::Error::msg("这是一个通用错误消息");
    println!("   error1: {:?}", error1);
    println!("   error1.to_string(): {}", error1.to_string());

    // 测试2: 使用 tera::Error::render 关联函数
    println!("\n2. 测试 tera::Error::render() 关联函数:");
    let context = Context::new();
    let error2 = tera::Error::msg("渲染过程中发生错误");
    println!("   error2: {:?}", error2);
    println!("   error2.to_string(): {}", error2.to_string());

    // 测试3: 使用 tera::Error::template_not_found 关联函数
    println!("\n3. 测试 tera::Error::template_not_found() 关联函数:");
    let error3 = tera::Error::template_not_found("missing_template.html");
    println!("   error3: {:?}", error3);
    println!("   error3.to_string(): {}", error3.to_string());

    // 测试4: 检查错误类型信息
    println!("\n4. 检查错误类型信息:");
    println!("   error1 type: {}", std::any::type_name::<tera::Error>());

    // 测试5: 检查错误来源
    println!("\n5. 检查错误来源:");
    if let Some(source) = error1.source() {
        println!("   error1.source(): {:?}", source);
    } else {
        println!("   error1.source(): None");
    }

    // 测试6: 尝试从 Tera 实例获取错误
    println!("\n6. 测试从 Tera 实例获取错误:");
    match Tera::new("non_existent_directory/**/*") {
        Ok(_) => println!("   意外成功"),
        Err(err) => {
            println!("   预期错误: {:?}", err);
            println!("   错误消息: {}", err.to_string());
        }
    }

    // 测试7: 检查 tera::Error 是否实现了 std::error::Error trait
    println!("\n7. 检查 trait 实现:");
    use std::error::Error as StdError;
    let error_ref: &dyn StdError = &error1;
    println!("   实现了 std::error::Error: {}", true);
    println!("   错误描述: {}", error_ref.to_string());

    // 测试8: 尝试模式匹配（应该会失败）
    // 尝试检查错误类型
    println!("\n8. 检查错误类型信息:");
    println!("   error1 类型: {}", std::any::type_name::<tera::Error>());
    println!("   error1 描述: {}", error1);

    // 测试从字符串创建错误
    println!("\n9. 测试从字符串创建错误:");
    let error_msg = "模板语法错误: {{ invalid }}";
    let error4 = tera::Error::msg(error_msg);
    println!("   error4: {}", error4);

    // 测试10: 尝试访问内部 kind 字段（如果可能）
    println!("\n10. 尝试访问 tera::Error 内部结构:");
    println!("   Debug 输出: {:?}", error1);
    println!("   Display 输出: {}", error1);

    // 测试11: 检查是否有公开的方法获取错误类型
    println!("\n11. 检查 tera::Error 的公开方法:");
    // 尝试调用可能的方法
    println!("   to_string(): {}", error1.to_string());

    // 测试12: 尝试使用 downcast 检查错误类型
    println!("\n12. 尝试 downcast 检查:");
    let std_error: &dyn StdError = &error1;
    println!("   作为 std::error::Error: {}", std_error);

    // 测试13: 创建不同类型的错误
    println!("\n13. 创建不同类型的 tera 错误:");
    let syntax_error = tera::Error::msg("语法错误: {% invalid %}");
    println!("   语法错误: {}", syntax_error);

    let circular_error = tera::Error::circular_extend(
        "base.html",
        vec!["base.html".to_string(), "child.html".to_string()],
    );
    println!("   循环继承错误: {}", circular_error);

    let missing_parent_error = tera::Error::missing_parent("child.html", "parent.html");
    println!("   缺少父模板错误: {}", missing_parent_error);

    println!("\n=== 测试完成 ===");
}
