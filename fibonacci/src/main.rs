use std::io;
use std::io::Write;

fn main() {
    let mut num = String::new();

    println!("Which fibonacci number you want to know ? ");
    print!("Please enter a number : ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read num.");
    
    let num: u32 = num.trim().parse().expect("Please enter a valid number!!");

    let mut first: u32 = 1;
    let mut second: u32 = 1;
    let mut third: u32 = 1;
    if num == 1 {
        println!("The {num}th fibonacci number is : {first}");
        return;
    }else if num == 2 {
        println!("The {num}th fibonacci number is : {second}");
        return;
    }else if num == 0 {
        println!("The {num}th fibonacci number is : 0");
        return;
    }
    for _i in 3..=num {
        third = second + first;
        first = second;
        second = third;
    }
    println!("The {num}th fibonacci number is : {third}");
}

// Do it using recursion after learning about ownership

