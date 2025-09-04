mod cli;
use cli::first_queries;
use cli::response_first;

mod calculate;
use calculate::calculate_first_query;

fn main() {

    let (total_length, first_character, second_character, third_character, fourth_character, user_name) = first_queries();

    //calculate_first_query(total_length, first_character, second_character, third_character, fourth_character);

    let (result, result_vector) = calculate_first_query(total_length, first_character, second_character, third_character, fourth_character);

    response_first(result, user_name);

    //println!(" The result of the calculation is: {}", result);

}

//TODO: Make it possible to set how many characters you want to replace, not just 4s