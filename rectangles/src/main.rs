// Defining an outer Debug attribute to be able to print values of Rectangle using 
// {:?} or {:#?} in println! macro
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32    
} 

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };        

    println!("\nThe area of rectangle is: {} square units", area(&rect));

    println!("\nrect is : {:#?}", rect);

    // Another alternative is dbg! macro 
    dbg!(&rect);

    println!("");

    let scale = 2;

    let rect2 = Rectangle {
        // dbg! macro takes ownership of expression, evaluates and prints it
        // and returns it's ownership back
        width : dbg!(30 * scale),
        height: 20
    };

    dbg!(&rect2);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

