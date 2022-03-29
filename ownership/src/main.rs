fn main() {

    // this is mutable string
    let mut s1 = String::from("Hello");

    // append something to this string
    s1.push_str(", World!");

    println!("{}", s1);

    let s2 = s1;

    //now s1 is invalid as it is moved to s2
    // (String does not have copy trait)
    // basically s2 takes ownership of s1's data

    // println!("s1 is: {}", s1); //does not work

    let x = 23;
    let y = x;
    
    println!("x is {}, y is {}", x,y); // works because of copy trait 
    
    takes_ownership(s2);
    // this function takes ownership of s2 and gives it to some_string
    //println!("s2 is {}", s2); //does not work

    //use tuples to return more data
    let s3 = String::from("Hello");
    let (s3, s3_size) = takes_and_gives_back_more_data(s3);

    println!("s3 is \"{}\" with size: {}", s3, s3_size);         

}

fn takes_ownership(some_string: String) {
    println!("Inside takes_ownership() some_string is: {}", some_string);
}

fn takes_and_gives_back_more_data(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}
