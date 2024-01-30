enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrWithDifferentTypes {
    V4(u8, u8, u8, u8),
    V6(String),
}
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}
enum IpAddrWithStruct {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4(String::from("127.0.0.1")),
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6(String::from("::1")),
        address: String::from("::1"),
    };
    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let loopback = IpAddrKind::V6(String::from("::1"));

    let home = IpAddrWithDifferentTypes::V4(127, 0, 0, 1);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    //Esse código irá gerar um erro pois o tipo Option pode ser um valor Nulo (None) e então como proteção rust não permite operações de Option<i8> com i8.
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;

}
