use pretty_assertions::assert_eq;

fn main() {
    // 左边的生成的 Rust 代码 (这里使用简单字符串模拟)
    let left = "fn main() {\n    println!(\"Hello, world!\");\n}\n";
    // 右边的占位符字符串
    let right = "asd";

    // 进行断言比较
    assert_eq!(left, right);
}
