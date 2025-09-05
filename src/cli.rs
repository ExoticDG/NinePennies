use std::io;
use std::io::{stdin, stdout, Write};


pub fn first_queries() -> (i16, String, String, String, String, String) {
    
    println!("Hello and welcome to NinePennies! I am your helpful CLI array compute agent!");

    let mut user_name = String::new();
    print!("Please enter your name: ");
    let _ = stdout().flush(); 
    stdin().read_line(&mut user_name).expect("Failed to read line");

    user_name = user_name.trim_end_matches('\n').trim_end_matches('\r').to_string();

    println!("Hello, {}! Nine in binary is 1001, My real name is 1001Pennies!", user_name);


    println!("I replace characters in an array and give you the total of each occurrence.");
    println!("First I replace every second character, then every third character, and finally every fourth character.");
    println!("Once I have the result, I tell you how many times each replaced character occurred in the array I made.");
    println!("Let's get started!");



    
    // Total length of the array
        //let mut total_length:i32;
        //print!("First, I need the total length of your array: ");
        //let _ = stdout().flush(); 
        //stdin().read_line(total_length).expect("Failed to read line");
        //total_length = total_length.trim_end_matches('\n').trim_end_matches('\r');

    println!("First, I need the total length of your array: ");

    let mut total_length = String::new();
    io::stdin()
        .read_line(&mut total_length)
        .expect("Failed to read line");

    let total_length: i16 = total_length
        .trim()
        .parse()
        .expect("Please enter a valid number");

    
    // First Character Used
    let mut first_character = String::new();
    //println!("");
    println!("Next, I need the first character you want me to use. This will be the base character that then gets replaced by the other ones: ");
    //println!("");
    let _ = stdout().flush(); 
    stdin().read_line(&mut first_character).expect("Failed to read line");
    first_character = first_character.trim_end_matches('\n').trim_end_matches('\r').to_string();

    // Second Character Used
    let mut second_character = String::new();
    //println!("");
    println!("Now, I need the second character. This replaces every second character in the array: ");
    //println!("");
    let _ = stdout().flush(); 
    stdin().read_line(&mut second_character).expect("Failed to read line");
    second_character = second_character.trim_end_matches('\n').trim_end_matches('\r').to_string();

    // Third Character Used
    let mut third_character = String::new();
    //println!("");
    println!("After that I of course need the third character. You get how this works by now, right? This replaces every third character in the array: ");
    //println!("");
    let _ = stdout().flush(); 
    stdin().read_line(&mut third_character).expect("Failed to read line");
    third_character = third_character.trim_end_matches('\n').trim_end_matches('\r').to_string();

    // Fourth Character Used
    let mut fourth_character = String::new();
    //println!("");
    println!("Finally, I need the fourth character: ");
    //println!("");
    let _ = stdout().flush(); 
    stdin().read_line(&mut fourth_character).expect("Failed to read line");
    fourth_character = fourth_character.trim_end_matches('\n').trim_end_matches('\r').to_string();

    return (total_length, first_character, second_character, third_character, fourth_character, user_name);
}




pub fn response_first(first_result:String, user_name:String, first_character:String, second_character:String, third_character:String, fourth_character:String) -> (i16, i16, i16, i16) {

    println!("Alright {user_name}, here's how many times each character appears in the array: {first_result}");

    let mut continue_to_total = String::new();
    println!("Would you like to continue calculations? This would solve for a total based on inputs you give for each character. (y/n)");
    let _ = stdout().flush(); 
    stdin().read_line(&mut continue_to_total).expect("Failed to read line");
    continue_to_total = continue_to_total.trim_end_matches('\n').trim_end_matches('\r').to_string();

    if continue_to_total == ("y").trim_end_matches('\n').trim_end_matches('\r').to_string() {


        println!("Alright! Now I need the value for the first character you gave me ({first_character}): ");

        let mut first_character_value = String::new();
            io::stdin()
            .read_line(&mut first_character_value)
            .expect("Failed to read line");

        let first_character_value: i16 = first_character_value
            .trim()
            .parse()
            .expect("Please enter a valid number");


        println!("Next, I need the value for the second character you gave me ({second_character}): ");

        let mut second_character_value = String::new();
            io::stdin()
            .read_line(&mut second_character_value)
            .expect("Failed to read line");

        let second_character_value: i16 = second_character_value
            .trim()
            .parse()
            .expect("Please enter a valid number");


        println!("I'll need the third character value now ({third_character}): ");

        let mut third_character_value = String::new();
            io::stdin()
            .read_line(&mut third_character_value)
            .expect("Failed to read line");

        let third_character_value: i16 = third_character_value
            .trim()
            .parse()
            .expect("Please enter a valid number");


        println!("Finaly, I need the last value for the last character ({fourth_character}):");

        let mut fourth_character_value = String::new();
            io::stdin()
            .read_line(&mut fourth_character_value)
            .expect("Failed to read line");

        let fourth_character_value: i16 = fourth_character_value
            .trim()
            .parse()
            .expect("Please enter a valid number");




        // let mut first_character_value = String::new();
        // println!("Alright! Now I need the value for the first character you gave me ({first_character}):");
        // let _ = stdout().flush(); 
        // stdin().read_line(&mut first_character_value).expect("Failed to read line");
        // first_character_value = first_character_value.trim_end_matches('\n').trim_end_matches('\r').to_string();

        // let mut second_character_value = String::new();
        // println!("Next, I need the value for the second character you gave me ({second_character}):");
        // let _ = stdout().flush(); 
        // stdin().read_line(&mut second_character_value).expect("Failed to read line");
        // second_character_value = second_character_value.trim_end_matches('\n').trim_end_matches('\r').to_string();

        // let mut third_character_value = String::new();
        // println!("I'll need the third character value now ({third_character}):");
        // let _ = stdout().flush(); 
        // stdin().read_line(&mut third_character_value).expect("Failed to read line");
        // third_character_value = third_character_value.trim_end_matches('\n').trim_end_matches('\r').to_string();

        // let mut fourth_character_value = String::new();
        // println!("Finaly, I need the last value for the last character ({fourth_character}):");
        // let _ = stdout().flush(); 
        // stdin().read_line(&mut fourth_character_value).expect("Failed to read line");
        // fourth_character_value = fourth_character_value.trim_end_matches('\n').trim_end_matches('\r').to_string();

        return (first_character_value, second_character_value, third_character_value, fourth_character_value)
    } 
    
    else {

        std::process::exit(0);

    };

    //println!("Would you like to continue the calculation? This would check to see if there are any looping characters. (y/n)");
}

pub fn responce_second (total:i32, first_character_total:i32, second_character_total:i32, third_character_total:i32, fourth_character_total:i32, user_name:String) {

    println!("Alright {user_name}, here's how many times each character appears in the array: {first_result}");


}