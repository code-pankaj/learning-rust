fn main() {

    // VARIABLES -->

    let mut x = 5; // use mut keyword to make a variable mutable
    println!("Value of x is : {}", x);
    x = 6; 
    println!("Value of x is : {}", x);

    let x = "seven"; // previous x is shadowed by this x
    println!("Value of x is : {}", x);

    const PI_VALUE : f64 = 3.14; // these can't be mutable 

    print!("{}", PI_VALUE);
    // DATA TYPES -->

    let a = 98_22; // decimal
    let b = 0x22; // hex
    let c = 0o77; //octal
    let d = 0b1111_000; // binary
    let e = b'A'; // byte

    let f: u8 = 255;

    println!("{}",a);
    println!("{}",b);
    println!("{}",c);
    println!("{}",d);
    println!("{}",e);
    println!("{}",f);

    let tup = ("Pankaj", 20);

    let (name, age) = tup;

    println!("Name is : {}", name);
    println!("Age is : {}", age);

    let sum = my_function(11, 22);
    println!("Sum is : {}", sum); 

    let number = 5;

    if number < 10 {
        println!("Number is less than 10.");
    }
    else {
        println!("Number is greater than 10.");
    }

    let collection = [10, 20, 30, 40, 50];

    for element in collection.iter() {
        println!("{}", element);
    }

    for number in 1..4 {
        println!("{}", number);
    }
}

fn my_function(x : i32, y: i32) -> i32{
    println!("Another Function.");
    println!("x+y : {}", x+y);
    x+y // by not putting semicolon and return in these function automatically return this.
}