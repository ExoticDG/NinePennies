use std::borrow::Borrow;



pub fn calculate_first_query(total_length: i16, first_character: String, second_character: String, third_character: String, fourth_character: String,) -> (String, Vec<i32>, usize, usize, usize, usize) {



    // let a:f32 = total_length_f32 / 12.0;

    // let b:i32 = a as i32;

    // let c:f32 = a - (b as f32);


    // let d = b * 12;

    // let result = format!("{} , {} , {}", a, b, c);

    let base_vector = vec![0; (total_length as i16).try_into().unwrap()];

// ** ↓ Base Array Calculation **

    let intone = 2;
    let inttwo = 3;
    let intthree = 4;

    let mut result_vector = base_vector;


    for (index, value) in result_vector.iter_mut().enumerate() {
        // Check if the current index (plus 1 for 1-based counting) is a multiple of n
        if (index + 1) % intone == 0 {
            *value = 1; // Modify the value, for example, set it to 0
        }
    }

    for (index, value) in result_vector.iter_mut().enumerate() {
        // Check if the current index (plus 1 for 1-based counting) is a multiple of n
        if (index + 1) % inttwo == 0 {
            *value = 2; // Modify the value, for example, set it to 0
        }
    }

    for (index, value) in result_vector.iter_mut().enumerate() {
        // Check if the current index (plus 1 for 1-based counting) is a multiple of n
        if (index + 1) % intthree == 0 {
            *value = 3; // Modify the value, for example, set it to 0
        }
    }

// ** ↓ Figure Out How Many Instances Each Number Appears **


    let first_count = result_vector
        .iter() // Create an iterator over the elements of the vector
        .filter(|&n| *n == 0) // Filter elements that match the target value
        .count(); // Count the remaining elements in the iterator

    let second_count = result_vector
        .iter() // Create an iterator over the elements of the vector
        .filter(|&n| *n == 1) // Filter elements that match the target value
        .count(); // Count the remaining elements in the iterator

    let third_count = result_vector
        .iter() // Create an iterator over the elements of the vector
        .filter(|&n| *n == 2) // Filter elements that match the target value
        .count(); // Count the remaining elements in the iterator

    let fourth_count = result_vector
        .iter() // Create an iterator over the elements of the vector
        .filter(|&n| *n == 3) // Filter elements that match the target value
        .count(); // Count the remaining elements in the iterator


         //println!("{first_character}: {first_count}, {second_character}: {second_count}, {third_character}: {third_count}, {fourth_character}: {fourth_count}");

    let first_result = format!("{first_character}: {first_count}, {second_character}: {second_count}, {third_character}: {third_count}, {fourth_character}: {fourth_count}");


   return (first_result, result_vector, first_count, second_count, third_count, fourth_count);
}

pub fn calculate_second_query(first_character_value: i16, second_character_value: i16, third_character_value: i16, fourth_character_value: i16, first_count: usize, second_count: usize, third_count: usize, fourth_count: usize) -> (i16, i16, i16, i16, i16) {

    let first_count_i16 = first_count as i16;
    let second_count_i16 = second_count as i16;
    let third_count_i16 = third_count as i16;
    let fourth_count_i16 = fourth_count as i16;

    let first_character_total:i16 = first_character_value * first_count_i16;
    let second_character_total:i16 = second_character_value * second_count_i16;
    let third_character_total:i16 = third_character_value * third_count_i16;
    let fourth_character_total:i16 = fourth_character_value * fourth_count_i16;

    let total:i16 = first_character_total + second_character_total + third_character_total + fourth_character_total;

    return(total, first_character_total, second_character_total, third_character_total, fourth_character_total)
}