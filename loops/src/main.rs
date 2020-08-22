fn main() {
    let c = to_celsius(32);
    let f = to_fahrenheit(100);
    println!("32 degrees fahrenheit is {} degrees celsius.", c);
    println!("100 degrees celsius is {} degrees fahrenheit.", f);

    let n = 5;
    let fib = fibonacci(n);
    println!("The {}th fibonacci sequence is {}.", n, fib);

    days_of_christmas();
}

fn to_celsius(temp: i32) -> i32 {
    (temp - 32) * 5 / 9
}

fn to_fahrenheit(temp: i32) -> i32 {
    temp * 9 / 5 + 32
}

fn fibonacci(x: i32) -> i32 {
    let mut result: i32;

    if x == 1 {
        result = 1;
    } else if x == 2 {
        result = 2;
    } else {
        result = 1;
        for i in 3..x {
            result += i;
        }
    }

    result
}

fn days_of_christmas() {
    let presents = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 1..13 {
        println!(
            "On the {}{} day of Christmas my true love gave to me",
            i,
            match i {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            }
        );
        println!("{}", presents[i - 1]);
        for j in (0..(i - 1)).rev() {
            if j == 0 {
                print!("And ")
            }
            println!("{}", presents[j]);
        }
        println!("\n\n");
    }
}
