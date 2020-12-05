use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::collections::HashMap;

const REQUIRED_KEYS: [&'static str; 7]=["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];

struct PassportRecord {
    map: HashMap<String, String>
}

impl PassportRecord {
    fn new(input_string: &String) -> Result<PassportRecord, &'static str> {
        let mut record = PassportRecord{map: HashMap::new()};
        
        for part in input_string.split_whitespace() {
            let parts:Vec<&str> = part.split(':').collect();
            if parts.len() != 2 {
                return Err("Failed to parse passport record");
            }
            record.map.insert(parts[0].to_string(), parts[1].to_string());
        }

        for key in REQUIRED_KEYS.iter() {
            if !record.map.contains_key(*key) {
                return Err("Missing required key.");
            }
        }

        Ok(record)
    }

    fn is_valid(&self) -> bool {
        self.check_birth_year() &&
        self.check_issue_year() &&
        self.check_expiry_year() &&
        self.check_height() &&
        self.check_hair_colour() &&
        self.check_eye_colour() &&
        self.check_passport_id()
    }

    /// 4 digits, between 1920 and 2020 inclusive
    fn check_birth_year(&self) -> bool {
        if self.map["byr"].len() != 4 {
            return false;
        }
        let year = match self.map["byr"].parse::<u32>() {
            Ok(val) => val,
            Err(_) => 0,
        };
        if year < 1920 || year > 2020 {
            return false;
        }
        true
    }

    //4 digits between 2010 and 2020 inclusive
    fn check_issue_year(&self) -> bool {
        if self.map["iyr"].len() != 4 {
            return false;
        }
        let year = match self.map["iyr"].parse::<u32>() {
            Ok(val) => val,
            Err(_) => 0,
        };
        if year < 2010 || year > 2020 {
            return false;
        }
        true
    }

    fn check_expiry_year(&self) -> bool {
        if self.map["eyr"].len() != 4 {
            return false;
        }
        let year = match self.map["eyr"].parse::<u32>() {
            Ok(val) => val,
            Err(_) => 0,
        };
        if year < 2020 || year > 2030 {
            return false;
        }
        true
    }

    fn check_height(&self) -> bool {
        let height = self.map["hgt"].clone();
        if height.len() < 4 {
            return false;
        }
        if height.ends_with("cm") {
            let hnum = match height.trim_end_matches("cm").parse::<u32>() {
                Ok(val) => val,
                Err(_) => 0
            };
            if hnum >= 150 && hnum <= 193 {
                return true;
            }

        } else if height.ends_with("in") {
            let hnum = match height.trim_end_matches("in").parse::<u32>() {
                Ok(val) => val,
                Err(_) => 0
            };
            if hnum >= 59 && hnum <= 76 {
                return true;
            }
        } 
        return false;
    }

    fn check_hair_colour(&self) -> bool {
        if self.map["hcl"].len() != 7 {
            return false;
        }
        if self.map["hcl"].chars().nth(0).unwrap() != '#' {
            return false
        }

        let mut value_check = self.map["hcl"].chars().all(|c| c.is_alphanumeric());
        value_check |= !self.map["hcl"].chars().any(|c| matches! (c, 'A'..='Z'));

        value_check
    }

    fn check_eye_colour(&self) -> bool {
        let eye_colours = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        eye_colours.iter().any(|&i| i == self.map["ecl"])
    }

    fn check_passport_id(&self) -> bool {
        self.map["pid"].len() == 9
    }


}

fn main() -> Result <(), Error> {
    let filename = Path::new("input.txt");
    let lines = read_lines(filename)?;

    let mut records: Vec<PassportRecord> = vec!(); 
    let mut record_string = String::new();
    let mut count = 0;
    let mut valid = 0;
    for line in lines {
        
        if let Ok(ip) = line {
            let trimmed = ip.trim_end();
            if ip.trim_end() == "" {
                println!("{}", record_string);
                match PassportRecord::new(&record_string) {
                    Ok(record) => records.push(record),
                    Err(e) => println!("{}", e),
                };
                record_string.clear();
                count += 1;
            } else {
                record_string.push_str(" ");
                record_string.push_str(trimmed);
            }
        }
    }

    let total_length = records.len();
    for record in records {
        if record.is_valid() {
            valid += 1;
        }
    }

    println!("There are {} accepted passports from {} records. {} of those are valid.", total_length, count, valid);

    Ok(())
}

fn read_lines (filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_good1() {
        let test = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
        let _ = PassportRecord::new(&test.to_string());
    }
    #[test]
    fn case_bad1() {
        let test = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884        hcl:#cfa07d byr:1929";
        let record = PassportRecord::new(&test.to_string());

        if let Err(e) = record {
            assert_eq! (e, "Missing required key.");
        } else {
            assert!(false);
        }
        
    }

    #[test]    
    fn case_good2() {
        let test_array: [&str; 2] = ["hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm",
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm"
        ];
        for test in test_array.iter() {
        if let Ok(_) = PassportRecord::new(&test.to_string()) {
            ()
        } else {
            assert!(false);
        }
    }
    }
    
    #[test]
    fn case_bad2() {
        let test_array = ["hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in",
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929"];
        for test in test_array.iter() {
        let record = PassportRecord::new(&test.to_string());

        if let Err(e) = record {
            assert_eq! (e, "Missing required key.");
        } else {
            assert!(false);
        }
    }
    }

    #[test]
    fn fields_true () {
        let test_array: [&str; 4] = ["pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f",
        "eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
        "hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022",
        "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
        ];
        for test in test_array.iter() {
        if let Ok(record) = PassportRecord::new(&test.to_string()) {
            assert!(record.check_birth_year());
            assert!(record.check_issue_year());
            assert!(record.check_expiry_year());
            assert!(record.check_height());
            assert!(record.check_hair_colour());
            assert!(record.check_eye_colour());
            assert!(record.check_passport_id());
            
            assert!(record.is_valid())
        } else {
            assert!(false);
        }
    }
    }

    #[test]
    fn fields_false () {
        let test_array: [&str; 4] = ["eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",        
        "iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946",        
        "hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
        "hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007"
        ];
        for test in test_array.iter() {
            if let Ok(record) = PassportRecord::new(&test.to_string()) {
                assert!(!record.is_valid())
            } else {
                assert!(false);
            }
        }
    }
    
}