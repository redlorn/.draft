fn main() {
   ip_enum1();
}

fn ip_enum1() {
    enum IpAddrKind {
        V4 (u8, u8, u8, u8),
        V6 (String),
    }

    let localhost = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    fn route(ip_kind: IpAddrKind) { }

    route(localhost);
    route(loopback);
}
