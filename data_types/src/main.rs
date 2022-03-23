fn main() {
    let x = 2.0; // f64
    
    let y: f32 = 3.0; // f32
    
    // boolean types
    let t = true;

    let f: bool = false;

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let (x, y, z) = tup; // pattern matching (destructuring)

    println!("The value of y is: {}", y);

    // access individual values by index
    let one = tup.2;

    println!("one is: {}", one);

    // arrays

    let a: [i32; 5] = [1,2,3,4,5];
    let b = [3; 5]; //[3,3,3,3,3]
    
       
}
