use std::io;
use std::io::{stdin, stdout, Write};


pub fn first_queries() -> (f32, String, String, String, String) {
    
    println!("Hello and welcome to NinePennies! I am your helpful CLI array compute agent!");

    let mut user_name = String::new();
    print!("Please enter your name: ");
    let _ = stdout().flush(); 
    stdin().read_line(&mut user_name).expect("Failed to read line");

    user_name = user_name.trim_end_matches('\n').trim_end_matches('\r').to_string();

    println!("Hello, {}! Nine in binary is 1001, My real name is 1001Pennies!", user_name);
    print!("");

    println!("I replace characters in an array and give you the total of each occurrence.");
    println!("First I replace every second character, then every third character, and finally every fourth character.");
    println!("Once I have the result, I tell you how many times each replaced character occurred in the array I made.");
    println!("Let's get started!");
    print!("");


    
    // Total length of the array
        //let mut total_length:i32;
        //print!("First, I need the total length of your array: ");
        //let _ = stdout().flush(); 
        //stdin().read_line(total_length).expect("Failed to read line");
        //total_length = total_length.trim_end_matches('\n').trim_end_matches('\r');

    print!("First, I need the total length of your array: ");

    let mut total_length = String::new();
    io::stdin()
        .read_line(&mut total_length)
        .expect("Failed to read line");

    let total_length_f32: f32 = total_length
        .trim()
        .parse()
        .expect("Please enter a valid number");


    print!("");

    
    // First Character Used
    let mut first_character = String::new();
    print!("Next, I need the first character you want me to use. This will be the base character that then gets replaced by the other ones: ");
    let _ = stdout().flush(); 
    stdin().read_line(&mut first_character).expect("Failed to read line");
    first_character = first_character.trim_end_matches('\n').trim_end_matches('\r').to_string();

    // Second Character Used
    let mut second_character = String::new();
    print!("Now, I need the second character. This replaces every second character in the array: ");
    let _ = stdout().flush(); 
    stdin().read_line(&mut second_character).expect("Failed to read line");
    second_character = second_character.trim_end_matches('\n').trim_end_matches('\r').to_string();

    // Third Character Used
    let mut third_character = String::new();
    print!("After that I of course need the third character. You get how this works by now, right? This replaces every third character in the array: ");
    let _ = stdout().flush(); 
    stdin().read_line(&mut third_character).expect("Failed to read line");
    third_character = third_character.trim_end_matches('\n').trim_end_matches('\r').to_string();

    // Fourth Character Used
    let mut fourth_character = String::new();
    print!("Finally, I need the fourth character: ");
    let _ = stdout().flush(); 
    stdin().read_line(&mut fourth_character).expect("Failed to read line");
    fourth_character = fourth_character.trim_end_matches('\n').trim_end_matches('\r').to_string();

    return (total_length_f32, first_character, second_character, third_character, fourth_character);
}