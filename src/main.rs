use std::io::stdin;

fn main() {
    println!("Bronco Personality Randomizer");
    println!("1 - Random personality");
    println!("2 - Add personality");
    println!("3 - Remove personality");
    println!("4 - Display personality table");

    let mut option = String::new();

    stdin().read_line(&mut option).unwrap();
    let num_option: i32 = option.trim().parse().unwrap();

    match num_option {
        1 => println!("Personality"),
        2 => println!("Add personality"),
        3 => println!("Remove personality"),
        4 => println!("Display personality table"),
        _ => {
            println!("Invalid option.");
            main()
        }
    }
}
