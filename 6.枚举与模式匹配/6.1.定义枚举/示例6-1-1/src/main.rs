enum IpAddr{
    v4(u8,u8,u8,u8),
    v6(String),
}

fn main() {
    let home =IpAddr::v4(192,168,1,1);
let loopback =IpAddr::v6(String::from("::1"));
    
}
