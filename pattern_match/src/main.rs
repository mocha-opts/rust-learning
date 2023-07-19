// match target {
//     模式1 => 表达式1,
//     模式2 => {
//         语句1;
//         语句2;
//         表达式2
//     },
//     _ => 表达式3
// }
fn main() {
    println!("Hello, world!");
    shadowing();
}

enum Direction {
    East,
    West,
    North,
    South,
}

fn match_direction() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        _ => println!("West"),
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
            println!("Lucky Penny!!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum IpAddr {
    Ipv4,
    Ipv6,
}

fn match_set_value() {
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币
}

fn value_in_cents2(coin: Coins) -> u8 {
    match coin {
        Coins::Penny => 1,
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter(state) => {
            println!("state quarter from {:?}!", state);
            25
        }
    }
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn match_action() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 255),
    ];

    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("move to ({},{})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{},g:{},b:0)'", r, g);
            }
        }
    }
}

fn if_let() {
    let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = v {
        println!("three")
    }
}

enum MyEnum {
    Foo,
    Bar,
}
fn matches() {
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    v.iter().filter(|x| matches!(x, MyEnum::Foo));

    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));
}

fn shadowing() {
    let age = Some(30);
    println!("在匹配前，age是{:?}", age);
    if let Some(age) = age {
        println!("匹配出来的age是{}", age);
    }

    println!("在匹配后，age是{:?}", age);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn test_plus_one() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
