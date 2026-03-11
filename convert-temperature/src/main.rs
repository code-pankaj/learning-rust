use std::io;
use std::io::Write;

fn main() {
    let mut option = String::new();
    println!("How do you want to convert?");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");
    println!("");
    print!("Please Choose an Option : ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read option!!!");

    let option: u8 = option.trim().parse().expect("Please enter a number!");
    if option > 2 || option < 1 {
        println!("Please choose a valid option!!");
        return;
    }

    let mut temp = String::new();

    print!("Please Enter the temperature : ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut temp)
        .expect("Falied to read temperature!!!");

    let temp: f32 = temp.trim().parse().expect("Please enter a valid number!");

    println!("");

    let ans: f32;
    if option == 1 {
        ans = (temp - 32.0) * (5.0 / 9.0);
        println!("Temperature will be : {ans}\u{00B0}C");
    } else if option == 2 {
        ans = (temp * (9.0 / 5.0)) + 32.0;
        println!("Temperature will be : {ans}\u{00B0}F");
    }
}
