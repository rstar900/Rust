// Defining a structure
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    // making an instance of structure
    let user1 = User {
        active: true,
        username: String::from("Lol"),
        email: String::from("lol@example.com"),
        sign_in_count: 1
    };

    // Since it is not mutable, we cannot change any value
    // But we can read them
    println!("This is the email of user1: {}", user1.email);

    // Mutable struct allows to change these values as well, but cannot set mutability for individual fields
    let mut user2 = User {
        active: true,
        username: String::from("Paul"),
        email: String::from("pl@example.com"),
        sign_in_count: 1
    };
    
    user2.email = String::from("paul@example.com");
    println!("This is the email of user2: {}", user2.email);

    let user3 = build_user(String::from("User@3"), String::from("u3@example.com"));
    println!("This is the email of user3: {}", user3.email);

    // When we create another User instance using Struct update syntax
    // the values like String do not have a copy trait are moved to new
    // instance, so we cannot use old instance's moved values instance anymore as it becomes invalid
    let user4 = User {
        email: String::from("pl2@example.com"),
        ..user2
    };

    println!("This is the email of user4: {}", user4.email);

    // This will work
    println!("This is the sign in count of user2: {}", user2.sign_in_count);

    // This will not work
    //println!("This is the name of user2: {}", user2.username);

    // Creating tuple structs (no need to give )
    struct Colour(u8, u8, u8);
    
    //Instantiating it
    let background_colour = Colour(128, 255, 5); // some rgb value

    // Values are accessed just like tuples (By index)
    println!("\nR channel is {}, G channel is {}, B channel is {}", background_colour.0, background_colour.1, background_colour.2);

    // Unit like structs (without any fields) 
    struct AlwaysEqual;
    let x = AlwaysEqual;

}

// Function to build an User instance
fn build_user(username: String, email: String) -> User {
    //using Field Init shorthand instead of using username: username, because parameter and field name is same
    User{
        username,
        email,
        active: true,
        sign_in_count: 1
    }
}
