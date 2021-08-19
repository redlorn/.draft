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

fn menu() -> u32 {
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

    let option :u32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    option
}

fn fahrenheit_to_celsius() {
    println!("Temperature (F): ");
    let fahrenheit = read_f64();
    let celsius :f64 = (fahrenheit - 32.0) * (5.0/9.0);
    println!("{} C", celsius);
}

fn celsius_to_fahrenheit() {
    println!("Temperature (C): ");
    let celsius = read_f64();
    let fahrenheit :f64 = celsius * (9.0/5.0) + 32.0;
    println!("{} F", fahrenheit);
}

fn read_f64() -> f64 {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Invalid float");
    let n :f64 = n.trim().parse().expect("Invalid float");
    n
}

fn fibonacci(n: u32) -> u128 {
    if n < 2 {
        n as u128
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

fn fibonacci_number() {
    println!("Fibonacci #: ");
    let n = read_u32();
    let fibonacci = fibonacci(n);
    println!("{}", fibonacci);
}

fn read_u32() -> u32 {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Invalid number");
    let n :u32 = n.trim().parse().expect("Invalid number");
    n
}

fn day_of_christmas(day: u32) -> String {
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

    let day :u32 = read_u32();

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

fn number_name(n :u32) -> &'static str {
    const NUMS :[&str; 12] = [
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

    NUMS[n as usize - 1]
}
