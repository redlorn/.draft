fn main() {
    variables();
    tuples();
    structs();
    methods();
}

fn variables() {
    println!("= variables =");

    // purely functional representation of rectangle data and area calculation

    let width1 = 30;
    let height1 = 50;

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    println!("area: {}", area(width1, height1));
}

fn tuples() {
    println!("= tuples =");

    // functional representation of rectangle data and area calculation using tuples and/
    // destructuring

    let rect1 = (30,50);

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    println!("area: {}", area(rect1));
}

fn structs() {
    println!("= structs =");

    // object-oriented representation of rectangle data, functional area calculation using instance

    // printing the object as a whole requires std::fmt::Display and std::fmt::Debug, macro provided
    #[derive(Debug)] // allows println! {:?} and {:#?} against an instance of this struct
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle { width: 30, height: 50 };

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    println!("area: {}", area(&rect1));

    println!("rect1 pretty-print: {:?}", rect1); // pretty-print
    println!("rect1 pretty-print extended: {:#?}", rect1); // pretty-print extended
}

fn methods() {
    println!("= methods =");

    // object-oriented representation of rectangle data and area calculation

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 { // methods can take ownership of self if ref isn't used
            self.width * self.height
        } // rare reasons to use self instead of &self: transform instance or deny its use after

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle { width: 30, height: 50 };

    println!("area: {}", rect1.area()); // method syntax

    rect1.area(); // this uses automatic referencing / dereferencing and is equivalent to:
    (&rect1).area(); // above example is an implicit borrow. mutable borrow: (&mut rect1)

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // multiple implementation blocks per struct are possible ....
    impl Rectangle {
        // associated function. doesn't use self, acts like a static method in other languages
        fn square(size: u32) -> Rectangle {
            Rectangle { width: size, height: size }
        }
    }

    // :: syntax for associated fns and namespaces
    println!("Square: {:?}", Rectangle::square(32));
}