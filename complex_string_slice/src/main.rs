/*
 * @Author: coconut 1424392205@qq.com
 * @Date: 2023-01-09 22:28:33
 * @LastEditors: coconut 1424392205@qq.com
 * @LastEditTime: 2023-05-18 17:33:19
 * @FilePath: /rust-learning/complex_type/src/main.rs
 * @Description:
 *
 * Copyright (c) 2023 by coconut 1424392205@qq.com, All Rights Reserved.
 */
fn main() {
    // s.clear(); // error!可变借用

    // let s1 = "中国人";
    // let a = &s1[0..3];

    // let mut string_replace_range = String::from("i like rust");
    // string_replace_range.insert(5, 's');
    // println!("{}1{}", string_replace_range, world);
    // test_string_insert();
    test_string_unicode();
}

fn hello_world() {
    let mut s = String::from("hello world");
    //左闭右开
    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[0..2];
    let slice2 = &s[..2];
    let len = s.len();
    let slice3 = &s[4..len];
    let slice4 = &s[4..];
    let slice5 = &s[..];
    let slice6 = &s[0..len];
    let word = first_word(&s);
    println!("the first word is: {}", word); //不可变借用
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

fn test_string_insert() {
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);
}
fn test_string_push() {
    let mut s = String::from("hello");
    s.push('!');
    println!("追加字符 push() -> {}", s);

    s.push_str("1");
    println!("追加字符串 push_str() -> {}", s);
}

fn test_string_replace() {
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);
}

fn test_string_replacen() {
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);
}

fn test_string_replace_range() {
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);
}

fn test_string_pop() {
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
}

fn test_string_remove() {
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    string_remove.remove(0);
    //下面的注释会报错
    //string_remove.remove(1);
    //string_remove.remove(3);
    dbg!(string_remove);
}

fn test_string_truncate() {
    let mut string_truncate = String::from("测试truncate方法");
    string_truncate.truncate(3);
    dbg!(string_truncate);
}

fn test_string_clear() {
    let mut string_clear = String::from("value");
    string_clear.clear();
    dbg!(string_clear);
}

fn test_string_concatenate() {
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result: String = string_append + &string_rust;
    let mut result: String = result + "!"; // `result + "!"` 中的 `result` 是不可变的
    result += "!!!";

    println!("连接字符串 + -> {}", result);
}

fn test_string_format() {
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
}

fn test_string_unicode() {
    for c in "中国人".chars() {
        println!("{}", c);
    }
    for b in "中国人".bytes() {
        println!("{}", b);
    }
}
