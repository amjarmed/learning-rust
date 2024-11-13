/// Converts a temperature from Fahrenheit to Celsius.
///
/// # Examples
///
///
fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * (5.0 / 9.0)
}

/// Converts a temperature from Celsius to Fahrenheit.
///
/// # Examples
///
///
fn celsius_to_fahrenheit(c: f32) -> f32 {
    c * (9.0 / 5.0) + 32.0
}

/// Calculates the nth Fibonacci number.
///
/// # Examples
///
///
fn nth_fibonacci_number(n: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    for _ in 2..=n {
        let next = first + second;
        first = second;
        second = next;
    }
    second
}

/// Prints the traditional song "The Twelve Days of Christmas" to the console.
///
/// It does this by looping through an array of the days of Christmas, and
/// looping backwards through an array of the gifts for each day, printing
/// each gift on a new line. The gifts are indented, and the last gift is
/// prefixed with "and " if it is not the first day of Christmas.
///
/// # Examples
///
///
fn the_twelve_days_of_christmas() {
    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth",
        "sixth", "seventh", "eighth", "ninth", "tenth",
        "eleventh", "twelfth",
    ];
    const GIFTS: [&str; 12] = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    // for day in 0..DAYS.len() {
    //     println!("On the {} day of Christmas\nmy true love gave to me\n{}", DAYS[day] , GIFTS[day]);
    // }

    for i in 0..DAYS.len() {
        println!("On the {} day of Christmas,\nmy true love gave to me", DAYS[i]);

        // loop backwards through gifts. up to the current day
        for j in (0..=i).rev() {
            if j == 0 && i > 0 {
                println!("and {}", GIFTS[j]);
            } else {
                println!("{}", GIFTS[j]);
            }
        }
        // Add a blank line between verses for readability
        println!();
    }
}
