fn main() {
    ip_enum1();
    msg_enum1();
}

fn ip_enum1() {
    println!("= ip_enum1 =");

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

fn msg_enum1() {
    println!("= msg_enum1 =");

    enum Message {
        Quit,
        Move{ x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32)
    }

    struct QuitMessage;
    struct MoveMessage {
        x: i32,
        y: i32
    }

    struct WriteMessage(String);
    struct ChangeColorMessage(i32, i32, i32);

    impl Message {
        fn call(&self) {
            println!("call");
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}