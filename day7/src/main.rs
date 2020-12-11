use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::collections::HashMap;
use std::fs;

struct BagRule {
    bag_type: String,
    count: u32
}

impl BagRule {
    fn new( bag_type: String, count: u32 ) -> BagRule {
        BagRule { bag_type, count }
    }

    fn contains_recur(&self, bag: &str, collection: &HashMap<String, Vec<BagRule>>) -> bool {
        if self.bag_type == bag {
            return true;
        }
        collection
            .get(&self.bag_type)
            .unwrap()
            .iter()
            .any(|br| br.contains_recur(bag, collection))
    }

    fn bag_count(&self, collection: &HashMap<String, Vec<BagRule>>, prev_count: u32) -> u32 {
        let rules = collection.get(&self.bag_type).unwrap();
        if rules.is_empty() {
            prev_count
        } else {
            rules
                .iter()
                .map(|br| br.bag_count(collection, br.count * prev_count))
                .sum::<u32>()
                + prev_count
        }
    }
}

impl From<&str> for BagRule {
    fn from(s: &str)-> Self {
        match s.find(" ") {
            Some(n) => {
                let num: u32 = s[0..n].parse().unwrap();
                BagRule::new(
                    String::from(s[n + 1..].trim_end_matches("s")),
                    num,
                )
            },
            None => panic!("Dammit")
        }
    }
}

fn to_hashmap(input: &str) -> HashMap<String, Vec<BagRule>> {
    input.lines()
        .map(|line| {
            let mut split = line.split(" contain ");
            let bag = split.next().unwrap().trim_end_matches("s");
            let unparsed_rules = split.next().unwrap().trim_end_matches(".");
            let rules: Vec<BagRule> = if unparsed_rules == "no other bags" {
                vec![]
            } else {
                unparsed_rules.split(", ").map(|s| s.into()).collect()
            };
            (String::from(bag), rules)
        } )
        .collect() 
}

fn main() -> Result<(), Error>{
    let filename = Path::new("input.txt");
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let hash_map = to_hashmap(&contents);
    
    let count = hash_map.iter()
        .filter(|(bag, rules)| {
            if bag.as_str() == "shiny gold bag" {
                false
            } else {
                rules
                    .iter()
                    .any(|br| br.contains_recur("shiny gold bag", &hash_map))
            }
        })
        .count();
    
    println! ("{}", count);

    let rules = hash_map.get("shiny gold bag").unwrap();
    let count2 =  rules
        .iter()
        .map(|br| br.bag_count(&hash_map, br.count))
        .sum::<u32>();

    println! ("contains {}", count2);

    Ok(())
}