/*
 * @Author: coconut 1424392205@qq.com
 * @Date: 2023-01-07 17:14:43
 * @LastEditors: coconut 1424392205@qq.com
 * @LastEditTime: 2023-01-07 17:15:26
 * @FilePath: /rust-learning/complex-num/src/main.rs
 * @Description:
 *
 * Copyright (c) 2023 by coconut 1424392205@qq.com, All Rights Reserved.
 */
use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}
