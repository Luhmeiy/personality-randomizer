use dialoguer::Select;
use rand;
use std::io::{self, Write};

use crate::file_io;

fn recalculate_percentages(personalities: &mut Vec<(String, f32)>) {
    let new_total = personalities.iter().map(|(_, value)| value).sum::<f32>();
    let scale = 100.0 / new_total;

    for (_, value) in personalities.iter_mut() {
        *value *= scale;
    }
}

pub fn get_random_personality(personalities: &mut Vec<(String, f32)>) {
    let mut random_personality_index = 0;

    let total_percentage = personalities.iter().map(|(_, value)| value).sum();
    let mut random_percentage = rand::random_range(0.0..total_percentage);

    for (index, (_, value)) in personalities.iter_mut().enumerate() {
        if random_percentage < *value {
            *value = (*value - 5.0).max(1.0);
            random_personality_index = index;
            break;
        }

        random_percentage -= *value;
    }

    recalculate_percentages(personalities);
    file_io::write_to_file(personalities);

    println!(
        "Bronco's personality now is: {}",
        personalities[random_personality_index].0
    )
}

pub fn add_personality(personalities: &mut Vec<(String, f32)>) {
    print!("Add personality: ");
    io::stdout().flush().unwrap();

    let mut personality = String::new();
    match io::stdin().read_line(&mut personality) {
        Ok(..) => {}
        Err(error) => {
            eprintln!("Failed to read from stdin: {}", error);
            return;
        }
    }

    let trimmed_personality = personality.trim();

    if trimmed_personality.is_empty() {
        println!("Empty personality not allowed.");
        return;
    }

    let personality_length = personalities.len();

    let percentage = if personality_length != 0 {
        100.0 / personality_length as f32
    } else {
        100.0
    };

    personalities.push((trimmed_personality.to_string(), percentage));

    recalculate_percentages(personalities);
    file_io::write_to_file(personalities);

    println!("Personality {} added.", trimmed_personality)
}

pub fn remove_personality(personalities: &mut Vec<(String, f32)>) {
    let display_items: Vec<&String> = personalities.iter().map(|(title, _)| title).collect();

    let selection = match Select::new()
        .with_prompt("Choose an option")
        .items(display_items)
        .default(0)
        .interact()
    {
        Ok(index) => index,
        Err(e) => {
            eprintln!("Error during selection: {}", e);
            return;
        }
    };

    let personality = personalities[selection].0.clone();

    personalities.remove(selection);

    recalculate_percentages(personalities);
    file_io::write_to_file(personalities);

    println!("Personality {} removed.", personality.trim())
}

pub fn reset_percentages(personalities: &mut Vec<(String, f32)>) {
    let percentage = 100.0 / personalities.len() as f32;

    for (_, value) in personalities.iter_mut() {
        *value = percentage
    }

    file_io::write_to_file(personalities);

    println!("Percentages reset.")
}
