use dialoguer::Select;
use std::io::{self, Write};

fn main() {
    let mut personalities: Vec<(String, f32)> =
        Vec::from([(String::from("a"), 50.0), (String::from("b"), 50.0)]);

    loop {
        println!("Bronco Personality Randomizer");
        println!("1 - Random personality");
        println!("2 - Add personality");
        println!("3 - Remove personality");
        println!("4 - Display personality table");

        let mut option = String::new();

        io::stdin().read_line(&mut option).unwrap();
        let num_option: i32 = option.trim().parse().unwrap();

        match num_option {
            1 => println!("Personality"),
            2 => add_personality(&mut personalities),
            3 => remove_personality(&mut personalities),
            4 => println!("Display personality table"),
            _ => println!("Invalid option."),
        }
    }
}

fn add_personality(personalities: &mut Vec<(String, f32)>) {
    print!("Add personality: ");
    io::stdout().flush().unwrap();

    let mut personality = String::new();
    io::stdin().read_line(&mut personality).unwrap();

    let percentage = 100.0 / (personalities.len() as f32 + 1.0);

    for item in personalities.iter_mut() {
        item.1 = (item.1 / 100.0) * (100.0 - percentage)
    }

    personalities.push((personality.trim().to_string(), percentage));

    println!("Personality {} added.", personality.trim())
}

fn remove_personality(personalities: &mut Vec<(String, f32)>) {
    if personalities.len() == 0 {
        println!("No personalities found.");
        return;
    }

    let display_items: Vec<&String> = personalities.iter().map(|(title, _)| title).collect();

    let selection = Select::new()
        .with_prompt("Choose an option")
        .items(display_items)
        .default(0)
        .interact()
        .unwrap();

    let personality = personalities[selection].0.clone();

    personalities.remove(selection);

    println!("Personality {} removed.", personality.trim())
}
