use std::{panic::resume_unwind, result};



pub fn calculate_first_query(total_length_f32: f32, first_character: String, second_character: String, third_character: String, fourth_character: String,) /*-> Strin */ {


    //TODO: Calculate how many characters of each type there are in the array

    // let a:f32 = total_length_f32 / 12.0;

    // let b:i32 = a as i32;

    // let c:f32 = a - (b as f32);


    // let d = b * 12;

    // let result = format!("{} , {} , {}", a, b, c);

    let base_vector = vec![0; (total_length_f32 as i32).try_into().unwrap()];

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


    // for //(index, value) in result_vector.iter_mut().enumerate() {
    //     // Check if the current index (plus 1 for 1-based counting) is a multiple of n
    //     if (index + 1) % intone == 0
    //         *value = 1 // Modify the value, for example, set it to 0
    //     //}
    

    // for i in 2..result_vector.len() {
    //     // Check if the current index (plus 1 for 1-based counting) is a multiple of the interval
    //     if (i + 1) % intone == 2 {
    //         // Modify the value at the current index
    //         result_vector[i] = 1; // Set the value to 0 for demonstration
    //     }
    // }

    let result = (result_vector); //("temp").to_string();

    print!("{:?}", result)
   // return(result);
}