struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
//元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//没有任何字段的类单元结构体
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("[[");
            1
        }
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter => 4,
    }
}
fn main() {
    //1
    let mut user1 = User {
        email: String::from("test@example.com"),
        username: String::from("test"),
        active: false,
        sign_in_count: 0,
    };
    //改变 User 实例 email 字段的值
    user1.email = String::from("hahah@example.com");
    println!("{}", user1.email);
    //2.
    //使用..语法指定剩余参数
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user2.username);

    //3.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //4.
    let subject = AlwaysEqual;

    //5.
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);

    println!("rect1 is {:?}", rect1.area());

    //6.
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username, //同名赋值优化写法
        active: true,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
