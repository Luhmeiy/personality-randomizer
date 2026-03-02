use std::io;

mod display;
mod file_io;
mod personality;

enum MenuAction {
    Random,
    Add,
    Remove,
    Display,
    Reset,
    Invalid,
}

impl MenuAction {
    fn from_number(option: i32) -> Self {
        match option {
            1 => MenuAction::Random,
            2 => MenuAction::Add,
            3 => MenuAction::Remove,
            4 => MenuAction::Display,
            5 => MenuAction::Reset,
            _ => MenuAction::Invalid,
        }
    }

    fn requires_non_empty(&self) -> bool {
        match self {
            MenuAction::Random | MenuAction::Remove | MenuAction::Display | MenuAction::Reset => {
                true
            }
            MenuAction::Add | MenuAction::Invalid => false,
        }
    }

    fn execute(self, personalities: &mut Vec<(String, f32)>) {
        if self.requires_non_empty() && personalities.is_empty() {
            println!("No personalities found.");
            return;
        }

        match self {
            MenuAction::Random => personality::get_random_personality(personalities),
            MenuAction::Add => personality::add_personality(personalities),
            MenuAction::Remove => personality::remove_personality(personalities),
            MenuAction::Display => display::display_personalities(personalities),
            MenuAction::Reset => personality::reset_percentages(personalities),
            MenuAction::Invalid => println!("Invalid option."),
        }
    }
}

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

        MenuAction::from_number(num_option).execute(&mut personalities);
    }
}
