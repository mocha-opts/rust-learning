# rust-learning

## 基本类型

Rust 有多种常见的类型：

- 布尔型 - bool 表示 true 或 false
- 无符号整型- u8 u32 u64 u128 表示正整数
- 有符号整型 - i8 i32 i64 i128 表示正负整数
- 指针大小的整数 - usize isize 表示内存中内容的索引和大小
- 浮点数 - f32 f64
- 元组（tuple） - (value, value, ...) 用于在栈上传递固定序列的值
- 数组 - 在编译时已知的具有固定长度的相同元素的集合
- 切片（slice） - 在运行时已知长度的相同元素的集合
- str(string slice) - 在运行时已知长度的文本

```Rust
fn main() {
    let x = 12; // 默认情况下，这是i32
    let a = 12u8;
    let b = 4.3; // 默认情况下，这是f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );
}
```

## 基本类型转换

当涉及到数字类型时，Rust 要求明确:不能想当然地把“u8”用在“u32”上。

但Rust提供了 as 关键字， 使数字类型转换非常容易。

```Rust
fn main() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
}


```
