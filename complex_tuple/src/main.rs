fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("x:{},y:{},z:{}", x, y, z);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;
    println!("{}-{}-{}", five_hundred, six_point_four, one);
    test_tuple();
}

fn test_tuple() {
    let s1 = String::from("hello"); //String

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
