use dialoguer::Select;
use rand;
use std::io::{self, Write};

mod display;
mod file_io;

fn main() {
    let mut personalities: Vec<(String, f32)> = Vec::new();
    file_io::read_file(&mut personalities);

    loop {
        println!("Bronco Personality Randomizer");
        println!("1 - Random personality");
        println!("2 - Add personality");
        println!("3 - Remove personality");
        println!("4 - Display personality table");
        println!("5 - Reset personalities percentages");

        let mut option = String::new();
        match io::stdin().read_line(&mut option) {
            Ok(..) => {}
            Err(error) => {
                eprintln!("Failed to read from stdin: {}", error);
                continue;
            }
        }

        let num_option: i32 = match option.trim().parse() {
            Ok(value) => value,
            Err(..) => {
                println!("Invalid option.");
                continue;
            }
        };

        match num_option {
            1 => get_random_personality(&mut personalities),
            2 => add_personality(&mut personalities),
            3 => remove_personality(&mut personalities),
            4 => display::display_personalities(&personalities),
            5 => reset_percentages(&mut personalities),
            _ => println!("Invalid option."),
        }
    }
}

fn recalculate_percentages(personalities: &mut Vec<(String, f32)>) {
    let new_total = personalities.iter().map(|(_, value)| value).sum::<f32>();
    let scale = 100.0 / new_total;

    for (_, value) in personalities.iter_mut() {
        *value *= scale;
    }
}

fn get_random_personality(personalities: &mut Vec<(String, f32)>) {
    if personalities.is_empty() {
        println!("No personalities found.");
        return;
    }

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

fn add_personality(personalities: &mut Vec<(String, f32)>) {
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

    let percentage = 100.0 / personalities.len() as f32;
    personalities.push((trimmed_personality.to_string(), percentage));

    recalculate_percentages(personalities);
    file_io::write_to_file(personalities);

    println!("Personality {} added.", trimmed_personality)
}

fn remove_personality(personalities: &mut Vec<(String, f32)>) {
    if personalities.is_empty() {
        println!("No personalities found.");
        return;
    }

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

fn reset_percentages(personalities: &mut Vec<(String, f32)>) {
    if personalities.is_empty() {
        println!("No personalities found.");
        return;
    }

    let percentage = 100.0 / personalities.len() as f32;

    for (_, value) in personalities.iter_mut() {
        *value = percentage
    }

    file_io::write_to_file(personalities);

    println!("Percentages reset.")
}
