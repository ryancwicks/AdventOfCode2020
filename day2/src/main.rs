use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;

#[derive(Debug)]
struct Password {
    letter: char,
    min_count: u32,
    max_count: u32,
    password: String
}

impl Password {
    fn new(input: String) -> Option<Password> {
        
        let line_parts: Vec<&str> = input.split(" ").collect();

        let password = line_parts[2];
        let letter = match line_parts[1].chars().next() {
            Some(val) => val,
            None => return None
        }; //grab first character
        let range: Vec<&str> = line_parts[0].split("-").collect();

        let min_count = match range[0].parse::<u32>() {
            Ok(val) => val,
            Err(_e) => {
                println!("{} could not be parsed.", range[0]);
                return None
            }
        };
        let max_count = match range[1].parse::<u32>(){
            Ok(val) => val,
            Err(_e) => {
                println!("{} could not be parsed.", range[0]);
                return None
            }
        };

        Some(Password {
            letter: letter,
            min_count: min_count,
            max_count: max_count,
            password: password.to_string()
        })
    }

    fn test_password (&self) -> bool {
        let mut char_count = 0;
        for a_char in self.password.chars() {
            if a_char == self.letter {
                char_count += 1;
            }
        }

        char_count >= self.min_count && char_count <= self.max_count
    }

    fn test_password2(&self) -> bool {
        let char_vec: Vec<char> = self.password.chars().collect();
        let index1 = (self.min_count - 1) as usize ;
        let index2 = (self.max_count -1) as usize;

        let pos1 = char_vec[index1] == self.letter;
        let pos2 = char_vec[index2] == self.letter;

        (pos1 || pos2) && (!pos1 || !pos2)
    }
}

impl std::fmt::Display for Password {
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write! (f, "{}:{} {}: {}", self.min_count, self.max_count, self.letter, self.password)
    }
}

fn main() -> Result<(), Error> {
    let filename = Path::new("./input.txt");

    let lines = read_lines(filename)?;
    let mut password_vec: Vec<Password> = vec![];
    let mut valid_count = 0;

    for line in lines {
        if let Ok(ip) = line {
            if let Some(password) = Password::new(ip) {
                if password.test_password2() {
                    valid_count += 1;
                }
                password_vec.push(password);
            } 
            
        }
    }

    println!("There are {} valid passwords.", valid_count);
    Ok(())
}

fn read_lines (filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

