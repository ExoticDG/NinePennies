

pub fn calculate_first_query(total_length_f32: f32, first_character: String, second_character: String, third_character: String, fourth_character: String,) -> (String, Vec<i32>) {


    //TODO: Calculate how many characters of each type there are in the array

    // let a:f32 = total_length_f32 / 12.0;

    // let b:i32 = a as i32;

    // let c:f32 = a - (b as f32);


    // let d = b * 12;

    // let result = format!("{} , {} , {}", a, b, c);

    let base_vector = vec![0; (total_length_f32 as i32).try_into().unwrap()];

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



    //     // Create a new mutable HashMap to store the counts
    // let mut counts: HashMap<i32, usize> = HashMap::new//();

    // // Iterate over each element in the vector
    // for &value in &result_vector {
    //     // Use the entry API to get a mutable reference to the value associated with the key
    //     // If the key doesn't exist, insert it with a default value of 0
    //     let count = counts.entry(value).or_insert(0);
    //     // Increment the count for the current value
    //     *count += 1;
    // //}


         //println!("{first_character}: {first_count}, {second_character}: {second_count}, {third_character}: {third_count}, {fourth_character}: {fourth_count}");

    let result = format!("{first_character}: {first_count}, {second_character}: {second_count}, {third_character}: {third_count}, {fourth_character}: {fourth_count}");


   return (result, result_vector);
}

// pub fn find_pattern(data: &[char]) -> Option<Vec<char>> {
//     let n = data.len();
//     // A pattern must repeat at least once, so its max length can only be n / 2.
//     for len in 1..=n / 2 {
//         // For the whole vector to be a repeating pattern, its length
//         // must be divisible by the pattern's length.
//         if n % len == 0 {
//             let potential_pattern = &data[0..len];
//             // Check if the rest of the data is made of repetitions of the potential pattern.
//             if data
//                 .chunks_exact(len)
//                 .all(|chunk| chunk == potential_pattern)
//             {
//                 // We found the smallest pattern because we started checking from len = 1.
//                 return Some(potential_pattern.to_vec());
//             }
//         }
//     }

//     // No repeating pattern was found.
//     None
// }