use std::io;
fn main() {
    println!("Enter year.");
    let mut string_year = String::new();

    io::stdin()
        .read_line(&mut string_year)
        .expect("Failed to read line");
    let year: u32 = string_year.trim().parse().expect("Please type a number!");
    if is_leap_year(year) {
        println!("This is a leap year");
    } else {
        println!("This is not a leap year");
    }
}

fn is_leap_year(year: u32) -> bool {
    return (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0);
}
