mod cli;
use cli::first_queries;
use cli::response_first;
use cli::responce_second;

mod calculate;
use calculate::calculate_first_query;
use calculate::calculate_second_query;


fn main() {

    let (total_length, first_character, second_character, third_character, fourth_character, user_name) = first_queries();

    //calculate_first_query(total_length, first_character, second_character, third_character, fourth_character);

    let (first_result, result_vector, first_count, second_count, third_count, fourth_count) = calculate_first_query(total_length, first_character.clone(), second_character.clone(), third_character.clone(), fourth_character.clone());

    let (first_character_value, second_character_value, third_character_value, fourth_character_value) = response_first(first_result, user_name.clone(), first_character.clone(), second_character.clone(), third_character.clone(), fourth_character.clone());
   
    let (total, first_character_total, second_character_total, third_character_total, fourth_character_total) = calculate_second_query(first_character_value.clone(), second_character_value.clone(), third_character_value.clone(), fourth_character_value.clone(), first_count.clone(), second_count.clone(), third_count.clone(), fourth_count.clone());
   
    responce_second(total, first_character_total, second_character_total, third_character_total, fourth_character_total, user_name.clone());

   // response_first(result, user_name, first_character, second_character, third_character, fourth_character);

    //println!(" The result of the calculation is: {}", result);

}

//TODO: Make it possible to set how many characters you want to replace, not just 4s