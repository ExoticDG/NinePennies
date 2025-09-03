
mod cli;
use cli::first_queries;

mod calculate;
use calculate::calculate_first_query;

fn main() {

    first_queries();

    calculate_first_query(total_length::string, first_character::string, second_character::string, third_character::string, fourth_character::string);

}