fn main() {
    ownership1();
    ownership2();
    references1();
    slices1();
}

fn ownership1() {
    println!("= ownership1 =");
    let x = 4;
    let y = x; // copied on to the stack, as i32 et al implement the Copy trait
    println!("x: {}", x);
    println!("y: {}", y);

    let a = String::from("aaa"); // allocates on the heap
    let b = a; // value moved to b, as String does not implement the Copy trait (it has a drop() fn)
    println!("b: {}", b);
    // error below: value is owned by b (moved), so a is invalid
    //println!("a: {}", a);
}

fn ownership2() {
    println!("= ownership2 =");
    let a = String::from("aaa"); // a is valid
    take_ownership(a); // ownership of a's value moves to a function parameter. a is now invalid.
    // error below: owner of value moved to take_ownership()'s s parameter, a is invalid
    //println!("a: {}", a);

}

fn take_ownership(s: String) {
    println!("ownership2(s): {}", s);
}

fn references1() {
    println!("= references1 =");

    let s = String::from("aaa");
    no_take_ownership(&s); // references do not own values
    println!("s: {}", s);

    let r = &s;
    no_take_ownership(r);
    println!("r: {}", r);
    println!("s: {}", s);
}

fn no_take_ownership(s: &str) {
    println!("no_take_ownership(s): {}", s);
}

fn slices1() {
    println!("= slices = ");
    let mut s = String::from("aaa bbb");
    let s_slice = &s[4..];
    no_take_ownership(s_slice); // slices (are a form of reference and) do not own values
    println!("{}", s_slice);

    s.clear(); // error because of immutable borrow with s_slice exists below ...
    //println!("{}", s_slice); // takes an immutable borrow
}
