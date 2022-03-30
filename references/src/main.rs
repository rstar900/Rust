fn main() {

    let s1 = String::from("Hello");
    let r1 = &s1; // reference to s1

    // getting length without taking away the ownership of s1 (borrowing)
    let length_s1 = get_length(r1);

    println!("length of s1: {}", length_s1);

    let mut s2 = String::from("Haha");
    let r2 = &mut s2; // for modifying via reference, reference also mutable

    modify_string(r2); 

    println!("s2 after modification: {}", s2);

    let r3 = &mut s2; // working as r2 went out of scope due to no usage
    // let r4 = &mut s2; // won't work as r3 is still in scope (only 1 &mut)
    //let r5 = &s2; // won't work as can't have mut and immutable& 
    //println!("r3 is: {}, r5 is: {}", r3, r5);

    println!("r3 is: {}", r3);

    // now can make immutable as r3's scope is over
    let r6 = &s2;
    println!("r6 is: {}", r6);

    // won't work
    // println!("dangling string is: {}", dangle());
        
}

fn get_length(some_string: &String) -> usize {
    some_string.len()
}

fn modify_string(some_string: &mut String) {
    some_string.push_str(", Lol!");
}


// won't work
//fn dangle() -> &String {
//    let s = String::from("Won't work");
//    &s // dangling reference not allowed
//}
