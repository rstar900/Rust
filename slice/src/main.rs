fn main() {
    let mut s = String::from("Hello, world!");
    let s_slice = get_second_word(&s);
    println!("Second word is: {}", s_slice);

    //using string literal as it is originally &str
    let s2 = "Hallo, Welt!";
    println!("Second word of string literal is: {}", get_second_word(s2));

    // The below two lines will give error as s.clear() will create 
    // mutable reference but s_slice (immutable) is still in scope
    // due to println! below 
    // s.clear();    
    // println!("\nSecond word is: {}", s_slice);    

    // Now making slice of array
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);
    println!("Slice is equal to expected output: [2,3]");
}

// &str is the slice reference of string we created
// used &str in input as well to allow all references and string literals
fn get_second_word(some_string: &str) -> &str {
    let string_bytes = some_string.as_bytes();

    for (i, &item) in string_bytes.iter().enumerate() {
        if item ==  b' '{
            return &some_string[i+1..]; // nothing after ".." means till end
            // we assume that 2nd word is the last word for this example
        }
    }
    
    &some_string[0..0]
}

