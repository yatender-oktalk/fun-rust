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
}

// this will accept both kind
fn routes(ip_type: IpAddrKind) {

}
