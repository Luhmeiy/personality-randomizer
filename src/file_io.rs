use std::fs::{self, File};
use std::io::Write;

pub fn read_file(personalities: &mut Vec<(String, f32)>) {
    match fs::read_to_string("personalities.txt") {
        Ok(contents) => {
            let tuples = contents
                .trim()
                .trim_start_matches('[')
                .trim_end_matches(']');

            for tuple in tuples.split("), (") {
                let tuple = tuple.trim_start_matches('(').trim_end_matches(')');
                let split: Vec<&str> = tuple.split(',').collect();

                let title = split[0].trim_matches('"').to_string();

                let value: f32 = match split[1].trim().parse() {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Failed to parse float: {}", e);
                        continue;
                    }
                };

                personalities.push((title, value));
            }
        }
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}

pub fn write_to_file(personalities: &mut Vec<(String, f32)>) {
    match File::create("personalities.txt") {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{:?}", personalities) {
                eprintln!("Failed to write to file: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to create file: {}", e),
    }
}
