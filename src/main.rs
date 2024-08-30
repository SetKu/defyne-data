use chrono::Local;
use serde_json::Value;
use std::{fs, path::Path};
use rayon::prelude::*;

fn main() {
    let data = fs::read_to_string(Path::new(
        "./wikitionary_english_2024-08-01_simplified.json",
    ))
    .expect("Unable to read data.");
    let word_list_raw: String =
        fs::read_to_string(Path::new("./word_frequency_of_333k_kaggle.txt"))
            .expect("Unable to read word list.");
    let word_list: Vec<&str> = word_list_raw.lines().collect();
    let json: Value = serde_json::from_str(&data).expect("Parsing error.");
    let json_array = json
        .as_array()
        .expect("Failed to take the parsed JSON as a mutable array.");

    // let number_of_entries = json_array.len();
    // println!("Number of Entries (Words): {}", number_of_entries);
    // let mut counter: usize = 0;
    // let prefix = "Progress:";
    // let fill = "█";
    // let remainder = "-";
    // let suffix = "Complete";
    // let length = 40;

    let filtered = json_array
        .into_par_iter() // Change back to just .into_iter to remove paralellism.
        .filter(|value| {
            // counter += 1;
            // 
            // if counter % 10000 == 1 {
            //     let percentage = counter as f32 / number_of_entries as f32;
            //     let bar = fill.repeat((length as f32 * percentage) as usize)
            //         + &remainder.repeat((length as f32 * (1. - percentage)) as usize);
            //     println!("{} |{}| {:.1}% {}", prefix, bar, percentage * 100., suffix);
            // }

            word_list.contains(
                &value
                    .get("word")
                    .expect("Failed to get the word for an entry.")
                    .as_str()
                    .expect("Failed to take the word for an entry as a string (&str)."),
            )
        })
        .cloned()
        .collect::<Vec<Value>>();
    let filtered_package = Value::Array(filtered);
    let serialized =
        serde_json::to_string(&filtered_package).expect("Failed to serialize the filtered data.");
    let file_name = format!("{}_wikitionary_english_simplified_filtered.txt", Local::now().format("%Y-%m-%dT%H-%M-%S").to_string());
    fs::write(file_name, serialized).expect("Failed to write the filtered data.");

    // for value in json.as_array() {
    //     let word = value
    //         .get("word")
    //         .expect("No word for entry.")
    //         .as_str()
    //         .expect("Failed to parse an entry's word as a string.");

    //     if !word_list.contains(word) {
    //         json.as_array_mut().expect("Failed to convert parsed JSON into a mutable array.").swap_remove(index)
    //     }
    // }
}
