use std::io;
use std::io::Write;

fn main() {
    let mut personalities: Vec<(String, f32)> = Vec::new();

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
            3 => println!("Remove personality"),
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
