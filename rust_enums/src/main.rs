enum IpAddrKind {
    v4,
    v6,
}

struct IPAddr {
    kind: IpAddrKind,
    address: String,
}

// alternate & concise method by this method struct not needed
enum Ipadd {
    v4(String),
    v6(String),
}

enum Ipadd2 {
    v4(u8,u8,u8,u8),
    v6(String),
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage {String};
struct ChangeColorMessage(i32, i32, i32);

// above four are similar to defining as different-2 structs
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body here
    }
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    let home = IPAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    }

    let loopback = IPAddr {
        kind: IpAddrKind::v6,
        address: String::from("::1"),
    }

    let home2 = Ipadd::v4(String.from("127.0.0.1"));
    let loopback2 = Ipadd::v6(String.from("::1"));

    let home3 = Ipadd2::v4(127, 0, 0, 1);
    let loopback3 = Ipadd2::v6(String.from("::1"));

    let some_num = Some(5);
    let some_string = Some("A string");
    let absent_num = Option<i32> = None;
}

// this will accept both kind
fn routes(ip_type: IpAddrKind) {

}
