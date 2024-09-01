use chrono::Local;
use serde_json::{Map, Value};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};
// use rayon::prelude::*;

fn main() {
    let word_list_raw: String =
        fs::read_to_string(Path::new("./kaggle_word_list.txt")).expect("Unable to read word list.");
    let word_list: Vec<&str> = word_list_raw.lines().collect();
    let input_argument = env::args().skip(1).next().expect("No input was provided.");

    println!("Using jq to convert the .jsonl file to .json. This may take a while...");

    let output = Command::new("jq")
        .args(["-s", ".", &input_argument])
        .output()
        .expect("jq failed to execute.");
    let output_string = String::from_utf8_lossy(&output.stdout);
    let mut json: Value = serde_json::from_str(&output_string).expect("Parsing error.");
    let json_array = json
        .as_array_mut()
        .expect("Failed to take the parsed JSON as a mutable array.");

    let number_of_entries = json_array.len();
    println!("Number of Entries (Words): {}", number_of_entries);
    let mut counter: usize = 0;
    let prefix = "Progress:";
    let fill = "█";
    let remainder = "-";
    let suffix = "Complete";
    let length = 40;

    let filtered = json_array
        // Change into .into_par_iter to add paralellism. This prevents the current reporting solution from working.
        .into_iter()
        .filter(|value| {
            counter += 1;

            if counter % 10000 == 1 {
                let percentage = counter as f32 / number_of_entries as f32 / 2.;
                let bar = fill.repeat((length as f32 * percentage) as usize)
                    + &remainder.repeat((length as f32 * (1. - percentage)) as usize);
                println!("{} |{}| {:.1}% {}", prefix, bar, percentage * 100., suffix);
            }

            word_list.contains(
                &value
                    .get("word")
                    .expect("Failed to get the word for an entry.")
                    .as_str()
                    .expect("Failed to take the word for an entry as a string (&str)."),
            )
        })
        .collect::<Vec<&mut Value>>();

    let filter_words_count = counter;
    let acceptable_keys = [
        "pos",
        "head_templates",
        "forms",
        "sounds",
        "etymology_text",
        "word",
        "senses",
        "synonyms",
        "hypernyms",
        "hyponyms",
        "derived",
        "translations",
    ];

    let modified = filtered
        .into_iter()
        .map(|value| {
            let object = value
                .as_object_mut()
                .expect("Failed to take an object as mutable.");

            let keys: Vec<String> = object.keys().cloned().collect();

            for key in keys {
                if !acceptable_keys.contains(&key.as_ref()) {
                    object.remove(&key);
                }
            }

            //
            // More specific edits.
            //

            {
                if let Some(head_templates) = object.get_mut("head_templates") {
                    let mut expansion: Option<Value> = None;

                    for item in head_templates.as_array_mut().unwrap() {
                        expansion = item.get("expansion").and_then(|v| Some(v.to_owned()));
                        break; // To headoff a potential entry with two expansions (though I've found none).
                    }

                    object.remove("head_templates");

                    if let Some(v) = expansion {
                        object.insert("expansion".to_owned(), v);
                    }
                }

                if let Some(sounds) = object.get("sounds") {
                    let array = sounds.as_array().unwrap();
                    let mut map = Map::new();

                    for value in array {
                        if let Some(ipa) = value.get("ipa") {
                            map.insert(
                                "ipa".to_owned(),
                                Value::String(ipa.as_str().unwrap().to_owned()),
                            );
                        }

                        if let Some(rhymes) = value.get("rhymes") {
                            map.insert(
                                "rhymes".to_owned(),
                                Value::String(rhymes.as_str().unwrap().to_owned()),
                            );
                        }

                        if let Some(mp3_url) = value.get("mp3_url") {
                            map.insert(
                                "mp3_url".to_owned(),
                                Value::String(mp3_url.as_str().unwrap().to_owned()),
                            );
                        }
                    }

                    let new_object = Value::Object(map);
                    object.insert("sounds".to_owned(), new_object);
                }

                if let Some(senses) = object.get_mut("senses") {
                    for sense in senses.as_array_mut().unwrap() {
                        let sense_object = sense.as_object_mut().unwrap();
                        sense_object.remove("tags");
                        sense_object.remove("id");
                    }
                }

                let embedded_word_fixes = ["derived", "synonyms", "hypernyms", "hyponyms"];

                for embedded_heading in embedded_word_fixes {
                    if let Some(synonyms) = object.get(embedded_heading) {
                        let mut words = vec![];

                        for synonym in synonyms.as_array().unwrap() {
                            words.push(Value::String(
                                synonym
                                    .as_object()
                                    .unwrap()
                                    .get("word")
                                    .expect(&format!(
                                        "No word was provided in a {} item.",
                                        embedded_heading
                                    ))
                                    .as_str()
                                    .unwrap()
                                    .to_owned(),
                            ));
                        }

                        object.insert(embedded_heading.to_owned(), Value::Array(words));
                    }
                }
            }

            counter += 1;

            if counter % 10000 == 1 {
                let percentage = counter as f32 / filter_words_count as f32 / 2. + 0.5;
                let bar = fill.repeat((length as f32 * percentage) as usize)
                    + &remainder.repeat((length as f32 * (1. - percentage)) as usize);
                println!("{} |{}| {:.1}% {}", prefix, bar, percentage * 100., suffix);
            }

            value
        })
        .map(|value| value.clone())
        .collect::<Vec<Value>>();

    let filtered_package = Value::Array(modified);
    let serialized =
        serde_json::to_string(&filtered_package).expect("Failed to serialize the filtered data.");
    let file_name = format!(
        "{}_wikitionary_english_modified.json",
        Local::now().format("%Y-%m-%dT%H-%M-%S").to_string()
    );
    let mut path_buffer = PathBuf::from_str(&input_argument).unwrap();
    path_buffer.pop();
    path_buffer.push(file_name);

    fs::write(path_buffer.clone(), serialized).expect("Failed to write the filtered data.");
    println!("Done! Wrote to {:?}", path_buffer);
}
