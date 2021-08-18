use std::io;

fn main() {
    loop {
        match menu() {
            0 => {
                break;
            },
            1 => {
                fahrenheit_to_celsius();
            },
            2 => {
                celsius_to_fahrenheit();
            },
            3 => {
                fibonacci_number();
            },
            4 => {
                sing_days_of_christmas();
            },
            _ => {
                break;
            }

        }
    }
}

fn menu() -> i32 {
    const MENU :[&str; 4] = [
        "Convert from Fahrenheit to Celsius",
        "Convert from Celsius to Fahrenheit",
        "Fibonacci number",
        "Day of Christmas",
    ];

    for i in 0..4 {
        println!("[{}] {}", i+1, MENU[i]);
    }

    println!("Menu (q to quit): ");

    let mut option = String::new();
    match io::stdin().read_line(&mut option) {
        Ok(_) => (),
        Err(_) => return 0
    };

    let option :i32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    option
}

fn fahrenheit_to_celsius() {
    println!("Temperature (F): ");

    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Invalid temperature");
    let fahrenheit :f64 = fahrenheit.trim().parse().expect("Invalid temperature");

    let celsius :f64 = (fahrenheit - 32.0) * (5.0/9.0);
    println!("{} C", celsius);
}

fn celsius_to_fahrenheit() {
    println!("Temperature (C): ");

    let mut celsius = String::new();
    io::stdin().read_line(&mut celsius).expect("Invalid temperature");
    let celsius :f64 = celsius.trim().parse().expect("Invalid temperature");

    let fahrenheit :f64 = celsius * (9.0/5.0) + 32.0;
    println!("{} F", fahrenheit);
}

fn fibonacci(n: u32) -> u32 {
    if n < 2 {
       n
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

fn fibonacci_number() {
    println!("Fibonacci #: ");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Invalid number");
    let n :u32 = n.trim().parse().expect("Invalid number");

    let fibonacci = fibonacci(n);
    println!("{}", fibonacci);
}

fn day_of_christmas(day: i32) -> String {
    const GIFTS :[&str; 12] = [
        "And a partridge in a pear tree.",
        "Two turtle doves,",
        "Three french hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-singing,",
        "Eight maids a milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ];

    if day < 1 || day > 12 {
        panic!("Invalid day");
    }

    GIFTS[day as usize - 1].to_string()
}

fn sing_days_of_christmas() {
    println!("Which day? ");

    let mut day = String::new();
    io::stdin().read_line(&mut day).expect("Invalid day");
    let day :i32 = day.trim().parse().expect("Invalid day");

    if day < 1 || day > 12 {
        println!("Not one of the twelve days of Christmas");
        return;
    }

    for d in 1..day+1 {
        println!("On the {} day of Christmas, my true love sent to me", number_name(d));

        if d == 1 {
            println!("A partridge in a pear tree.");
        } else {
            for l in (1..d + 1).rev() {
                println!("{}", day_of_christmas(l));
            }
        }

        println!();
    }
}

fn number_name(n :i32) -> String {
    const NUMS :[&str;12] = [
        "First",
        "Second",
        "Third",
        "Fourth",
        "Fifth",
        "Sixth",
        "Seventh",
        "Eighth",
        "Ninth",
        "Tenth",
        "Eleventh",
        "Twelfth"
    ];

    if n < 1 || n > 12 {
        panic!("No name for number");
    }

    NUMS[n as usize - 1].to_string()
}
