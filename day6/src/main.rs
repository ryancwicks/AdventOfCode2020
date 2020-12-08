use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::collections::HashMap;
use std::cmp::{PartialEq, Ordering};

#[derive(Eq)]
struct GroupAnswers {
    map: HashMap<char, u32>,
    people: u32
}

impl GroupAnswers {
    fn new(input: String, people: u32) -> GroupAnswers {
        let mut map = GroupAnswers { map: HashMap::new(), people};

        for a_char in input.chars() {
            if a_char != ' ' {
                let entry = map.map.entry(a_char).or_insert(0);
                *entry += 1;
            }
        }

        map
    }

    fn value(&self) -> usize {
        self.map.len()
    }

    fn matched_value(&self) -> usize {
        let mut count = 0;
        for (_, val) in self.map.iter() {
            if *val == self.people {
                count += 1;
            }
        }
        count
    }

}

impl PartialEq for GroupAnswers {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for GroupAnswers {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GroupAnswers {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

fn read_lines (filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<(), Error> {
    
    let filename = Path::new("input.txt");
    let lines = read_lines(filename)?;

    let mut group_string = String::new();
    let mut people_count = 0;
    let mut groups: Vec<GroupAnswers> = vec![];

    for line in lines {
        if let Ok(ip) = line {
            if ip != "" {
                group_string.push_str(&ip);
                people_count += 1;
            } else {
                groups.push(GroupAnswers::new(group_string, people_count));
                group_string = String::new();
                people_count = 0;
            }
        }
    }

    let mut sum = 0;
    let mut matched_sum = 0;
    for group in groups {
        sum += group.value();
        matched_sum += group.matched_value();
    }

    println! ("Total: {}, Matched: {}", sum, matched_sum);

    Ok(())
}
