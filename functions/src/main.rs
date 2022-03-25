fn main() {
    print_labeled_measurement(5, 'h');

    // expressions can be assigned to variables
    // as they return values
    let y = {
        let x = 3;
        x + 1  // no semicolon as it is an expression
    };

    println!("y is: {}", y);

    //function call with parameter and expression inside and return type
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
