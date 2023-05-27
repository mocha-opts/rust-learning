struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let mut user1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // println!("Hello, world!");
    // user1.email = String::from("anotheremail@example.com");
    // let user2 = User {
    //     email: String::from("12"),
    //     ..user1
    // };
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    dbg!(&rect1);

    // println!("rect1 is {:?}", rect1);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn test_tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct AlwaysEqual;
trait SomeTrait {}
impl SomeTrait for AlwaysEqual {}

fn test_unit_like_struct() {
    let subject = AlwaysEqual;
    let origin = Point(0, 0, 0);
    // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
}
