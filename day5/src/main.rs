use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::cmp::Ordering;

#[derive(Eq)]
struct Seat {
    code: String,
    row: u32,
    col: u32,
    id: u32
}

impl Seat {
    fn new (code: String) -> Result<Seat, Error> {
        let row_code = &code[0..7];
        let col_code = &code[7..10];

        let mut row_max = 127;
        let mut row_min = 0;
        for side in row_code.chars() {
            let half = (row_max - row_min)/2;
            if side == 'F' {
                row_max = row_min + half;
            } else {
                row_min = row_min + half+1;
            }
        }

        let mut col_max = 7;
        let mut col_min = 0;
        for side in col_code.chars() {
            let half = (col_max - col_min)/2;
            if side == 'L' {
                col_max = col_min + half;
            } else {
                col_min = col_min + half+1;
            }
        }

        Ok (Seat{
            code: code,
            row: row_min,
            col: col_min,
            id: row_min*8+col_min
        })
    }
}

impl std::fmt::Display for Seat {
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write! (f, "Code: {} Row: {} Col: {}  ID: {}", self.code, self.row, self.col, self.id)
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Seat {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn read_lines (filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<(), Error>{
    let filename = Path::new("input.txt");
    let lines = read_lines(filename)?;

    let mut seats: Vec<Seat> =  vec![];

    for line in lines {
        if let Ok(ip) = line {
            if let Ok(seat) = Seat::new(ip) {
                seats.push(seat);
            }
        }
    }

    seats.sort();
    let mut last_id = 0;
    for seat in seats {
        if seat.id - last_id != 1 {
            println!("{}", seat);
        }
        last_id = seat.id;
    }

    //let max_value = match seats.iter().max() {
    //    Some(val)=>val,
    //    None => panic!("Vector was empty.")
    //};

    //println!("Max value is {}", max_value);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_good1() {
        let test = "FBFBBFFRLR";
        if let Ok(seat) = Seat::new(test.to_string()) {
            assert_eq!(seat.row, 44);
            assert_eq!(seat.col, 5);
            assert_eq!(seat.id, 357);
        } else {
            assert! (false);
        }
    }
}