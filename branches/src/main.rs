fn main() {
    let number = 7;
    
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number2 = 6;
    
    if number2 % 4 == 0 {
        println!("number2 is divisible by 4");
    } else if number2 % 3 == 0 {
        println!("number2 is divisible by 3");
    } else if number2 % 2 == 0 {
        println!("number2 is divisible by 2");
    } else {
        println!("number2 is not divisible by 2, 3 or 4");
    }

    // assigning value using if else (expression inside if and else same type)
    let five = if true {5} else {6};
    println!("The value of five is: {}", five);
}
