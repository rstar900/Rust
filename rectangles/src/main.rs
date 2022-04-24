// Defining an outer Debug attribute to be able to print values of Rectangle using 
// {:?} or {:#?} in println! macro
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32    
}

// All the associated functions (methods and non methods) are defined under impl blocks
// multiple impl blocks allowed
// self parameter is compulsory for methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // methods can have same name as fields
    fn width(&self) -> bool {
        self.width > 0
   }
} 

impl Rectangle {
    // method with additional parameters 
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // A non method associated function
    // self is not required in this case as it is like a static function in Java or C++
    fn square(width: u32) -> Rectangle {
        Rectangle {
            width,
            height: width
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };        

    // Calling a method of Rectangle instance
    println!("\nThe area of rectangle is: {} square units", rect1.area());
    println!("This rectangle has a non-zero width: {}", rect1.width());

    println!("\nrect1 is : {:#?}", rect1);

    // Another alternative is dbg! macro 
    dbg!(&rect1);

    println!("");

    let scale = 2;

    let rect2 = Rectangle {
        // dbg! macro takes ownership of expression, evaluates and prints it
        // and returns it's ownership back
        width : dbg!(30 * scale),
        height: 20
    };


    let rect3 = Rectangle {
        width: 10,
        height: 30
    };

    println!("\nrect1 can hold rect2 : {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3 : {}", rect1.can_hold(&rect3));
    println!("");
    dbg!(&rect2);


    // Calling a non method associated function
    let square = Rectangle::square(50);

    println!("");
    dbg!(&square);
}

