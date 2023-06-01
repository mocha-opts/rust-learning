enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

struct PokerCard {
    suit: PokerSuit,
    value: u8,
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("Hello, world!");

    let heart = PokerSuit::Hearts;
    let diamonds = PokerSuit::Diamonds;
    print_suit(heart);
    print_suit(diamonds);
}

fn route(ip_type: IpAddrKind) {}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn print_message() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::ChangeColor(255, 255, 0);
}
