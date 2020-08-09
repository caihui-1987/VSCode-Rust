//将IP地址的数据和IpAddrKind成员存储在一个struct中

enum IpAddrKind {
    v4,
    v6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::v6,
        address: String::from("::1"),
    };

    println!("{}\n{}", home.address,loopback.address);
}
