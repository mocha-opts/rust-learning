fn main() {
    //可变性
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //下划线开头忽略为使用的变量
    let _x = 5;
    let y = 10;

    //变量解构
    let (a, mut b): (bool, bool) = (true, false);
    println!("a= {:?},b= {:?}", a, b); //a = true, b = false
     b = true;
    assert_eq!(a, b);
    println!("a= {:?},b= {:?}", a, b);//a = true, b = true

    //在 Rust 1.59 版本后，可以在赋值语句的左式中使用元组、切片和结构体模式了
    let (c, d, e, f, g);
    (c,d) = (1,2);
    [e, .., f, _] = [1, 2, 3, 4, 5];
    Struct { g, .. } = Struct { g: 5 };
    assert_eq!([1, 2, 1, 4, 5], [c, d, e, f, g]);


}
struct Struct {
    g: i32
}