use std::io;

fn main()
{
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter a number.");

    io::stdin().read_line(&mut num1).expect("Failed");
    let num1:i32 = num1.trim().parse().expect("Failed");

    println!("Enter a second number.");

    io::stdin().read_line(&mut num2).expect("Failed");
    let num2:i32 = num2.trim().parse().expect("Failed");

    println!(" ");
    println!("Your number is: {}", num1 + num2)

}