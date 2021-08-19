fn main() {
    structs1();
}

fn structs1() {
    println!("= structs1 =");

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool
    }

    let user1 = User {
        email: String::from("user1@example.tld"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1
    };

    println!("user1.email: {}", user1.email);

    let mut userm = User {
        email: String::from("userm@example.tld"),
        username: String::from("userm"),
        active: true,
        sign_in_count: 1
    };

    userm.active = false;

    println!("userm.active: {}", userm.active);

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    let userb = build_user(String::from("userb@example.tld"), String::from("user2b"));
    println!("userb.username: {}", userb.username);

    let user2 = User {
        email: String::from("user2@example.tld"),
        username: String::from("user2"),
        ..user1
    };

    println!("user2.username: {}", user2.username);
    println!("user2.sign_in_count: {}", user2.sign_in_count);

    struct Color(i32, i32, i32);
    let red = Color(255,0,0);
    println!("red.0: {}", red.0);

    struct Point(i32, i32, i32);
    // error below: Color and Point tuple-structs are different types, regardless of fields
    //let point :Point = red;
}
