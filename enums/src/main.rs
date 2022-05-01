// Defining an enum with different types of values
enum Message {
    Quit, // just like a constant
    Move {x: i32, y: i32}, // with fields like a struct
    Write(String), // with one value
    ChangeColor(u8, u8, u8) //with multiple values
}

// Defining methods for the enum type
impl Message {
    fn display_values(&self) {
        // Displaying appropriate values by using match to determine type
        // Need to cover all possible values, otherwise it won't compile
        match self {
            Message::Quit => println!("Quit Message!"),
            Message::Move{x,y} => println!("Move Message : {{x: {}, y: {}}}", x, y),
            Message::Write(s) => println!("Write Message: \"{}\"", s),
            Message::ChangeColor(r, g, b) => println!("ChangeColor Message: {{R: {}, G: {}, B: {}}}", r,g,b),
        }
    }
}

fn main() {

    // Making instances of Message type
    let m1 = Message::Quit;
    let m2 = Message::Move{x : 100, y : -100};
    let m3 = Message::Write(String::from("Hello"));
    let m4 = Message::ChangeColor(0,100,255);

    // Method calls on each instance
    println!();
    m1.display_values();
    m2.display_values();
    m3.display_values();
    m4.display_values();

    // The Option enum is already inluded by efault in code
    let some_num = Some(5);  //Option<i32> is inferred
    let some_string = Some("Hello"); // Option<str> is inferred
    let no_num: Option<i32> = None; 

    // let y: i32 = 5 + sopme_num will not work as Option type needs to be
    // converted into proper type like i32 to do any operations  
    

    let some_num_plus_one = plus_one(some_num);
    let no_num_plus_one = plus_one(no_num);

    // Derived debug trait is already defined for Option types
    println!("\nsome_num_plus_one: {:?}", some_num_plus_one);
    println!("no_num_plus_one: {:?}", no_num_plus_one);
    println!("some_string: {:?}", some_string);

    // An example where only on dice roll of 3 and 7 something happens
    // We can use 'other' to use all other values in something, or
    // _ to completely do nothing and ignore all other values
    // We use unit value '()' (empty tuple) to say nothing to do
    println!();

    let dice = 3;
   
    match dice {
        3 => println!("3 rolled!"),
        7 => println!("7 rolled!"),
        _ => ()
    }

    // with 'if let', we can do something if the value of such expression
    // matches only a certain value and ignore others
    // or we execute some statements using 'else'
    println!();
    
    if let 3 = dice {
        println!("Prost! You got a 3");
    } else {
        println!("Better luck next time!");
    }
}

// Increment the value inside the Option type if it has valid integer value
fn plus_one(some_value: Option<i32>) -> Option<i32> {
    match some_value {
        Some(x) => Some(x + 1),
        None => None
    }
}
