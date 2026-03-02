use std::io;

mod display;
mod file_io;
mod personality;

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
            1 => personality::get_random_personality(&mut personalities),
            2 => personality::add_personality(&mut personalities),
            3 => personality::remove_personality(&mut personalities),
            4 => display::display_personalities(&personalities),
            5 => personality::reset_percentages(&mut personalities),
            _ => println!("Invalid option."),
        }
    }
}
