use std::fs;
use std::collections::HashMap;

const FILE_PATH: &str = "./input.txt";

fn create_digit_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("one", "1");
    map.insert("two", "2");
    map.insert("three", "3");
    map.insert("four", "4");
    map.insert("five", "5");
    map.insert("six", "6");
    map.insert("seven", "7");
    map.insert("eight", "8");
    map.insert("nine", "9");
    map
}

fn main() {

    let digits: HashMap<&str, &str> = create_digit_map();
    let mut total_output: i32 = 0;

    match fs::read_to_string(FILE_PATH) {
        Ok(contents) => {
            
            for line in contents.lines() {

                let mut current_line = line.to_string();
                
                let mut i = 0;
                while i < current_line.len() {
                    let end = std::cmp::min(i + 5, current_line.len());
                    let segment = &current_line[i..end];

                    for (key, value) in &digits {
                        if segment.contains(key) {
                            current_line = current_line.replacen(key, value, 1);
                            break;
                        }
                    }

                    i += 1;
                }
                
                let mut number: String = "".to_string();
                let mut chars: Vec<_> = current_line.chars().collect();

                for c in &chars {
                    if c.is_digit(10) {
                        number.push(*c);
                        break;
                    }
                }

                chars.reverse();

                for c in &chars {
                    if c.is_digit(10) {
                        number.push(*c);
                        break;
                    }
                }
                let line_output = number.parse::<i32>().unwrap();
                total_output += line_output;            }
        }
        Err(err) => {
            eprintln!("Erreur de lecture du fichier : {}", err);
            std::process::exit(1);
        }
    }
    println!("{}", total_output);
}
