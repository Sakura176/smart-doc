//! 测试 tera::Error 到自定义 Error 的转换

use smart_doc_core::error::Error;
use tera;

fn main() {
    println!("=== 测试 tera::Error 转换 ===");

    // 测试1: Msg 类型的错误
    println!("\n1. 测试 Msg 类型错误转换:");
    let tera_error1 = tera::Error::msg("这是一个通用模板错误");
    let our_error1: Error = tera_error1.into();
    println!("   原始错误: 这是一个通用模板错误");
    println!("   转换后: {:?}", our_error1);
    println!("   期望: Error::Template(\"这是一个通用模板错误\")");

    // 测试2: TemplateNotFound 类型的错误
    println!("\n2. 测试 TemplateNotFound 类型错误转换:");
    let tera_error2 = tera::Error::template_not_found("missing_template.html");
    let our_error2: Error = tera_error2.into();
    println!("   原始错误: Template 'missing_template.html' not found");
    println!("   转换后: {:?}", our_error2);
    println!("   期望: Error::TemplateNotFound(\"missing_template.html\")");

    // 测试3: 语法错误
    println!("\n3. 测试语法错误转换:");
    let tera_error3 = tera::Error::msg("语法错误: {% invalid syntax %}");
    let our_error3: Error = tera_error3.into();
    println!("   原始错误: 语法错误: {{% invalid syntax %}}");
    println!("   转换后: {:?}", our_error3);
    println!("   期望: Error::TemplateSyntax(\"语法错误: {{% invalid syntax %}}\")");

    // 测试4: 渲染错误
    println!("\n4. 测试渲染错误转换:");
    let tera_error4 = tera::Error::msg("渲染错误: 变量 'user' 未定义");
    let our_error4: Error = tera_error4.into();
    println!("   原始错误: 渲染错误: 变量 'user' 未定义");
    println!("   转换后: {:?}", our_error4);
    println!("   期望: Error::TemplateRender(\"渲染错误: 变量 'user' 未定义\")");

    // 测试5: 循环继承错误
    println!("\n5. 测试循环继承错误转换:");
    let tera_error5 = tera::Error::circular_extend(
        "base.html",
        vec!["base.html".to_string(), "child.html".to_string()],
    );
    let our_error5: Error = tera_error5.into();
    println!("   原始错误: Circular extend detected for template 'base.html'...");
    println!("   转换后: {:?}", our_error5);
    println!("   期望: Error::Template(...) [通用模板错误]");

    // 测试6: 缺少父模板错误
    println!("\n6. 测试缺少父模板错误转换:");
    let tera_error6 = tera::Error::missing_parent("child.html", "parent.html");
    let our_error6: Error = tera_error6.into();
    println!("   原始错误: Template 'child.html' is inheriting from 'parent.html'...");
    println!("   转换后: {:?}", our_error6);
    println!("   期望: Error::Template(...) [通用模板错误]");

    // 测试7: 使用问号运算符自动转换
    println!("\n7. 测试问号运算符自动转换:");
    let result = test_auto_conversion();
    match result {
        Ok(_) => println!("   意外成功"),
        Err(err) => println!("   自动转换后的错误: {:?}", err),
    }

    // 测试8: 验证错误消息提取
    println!("\n8. 验证模板名称提取:");
    let test_cases = vec![
        "Template 'home.html' not found",
        "Template 'user/profile.html' not found in templates directory",
        "Template 'admin/dashboard.html' not found",
    ];

    for (i, msg) in test_cases.iter().enumerate() {
        let tera_error = tera::Error::msg(msg);
        let our_error: Error = tera_error.into();
        println!("   测试 {}: {}", i + 1, msg);
        println!("   转换结果: {:?}", our_error);
    }

    println!("\n=== 测试完成 ===");
}

/// 测试函数，演示问号运算符如何自动转换错误
fn test_auto_conversion() -> Result<(), Error> {
    // 模拟一个可能返回 tera::Error 的操作
    let tera_result: std::result::Result<(), tera::Error> = Err(tera::Error::msg("模拟的模板错误"));

    // 问号运算符会自动调用 From<tera::Error> for Error
    tera_result?;

    Ok(())
}
