use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::ops::Index;

const CELL_WIDTH: usize = 31;

struct Horizontal {
    row: [Cell; CELL_WIDTH]
}

#[derive(Copy, Clone, std::cmp::PartialEq)]
enum Cell {
    Empty,
    Tree
}

impl Horizontal {
    fn new(line: String) -> Horizontal {
        let mut horizontal = Horizontal { row: [Cell::Empty; CELL_WIDTH] };
        let line_iter = line.chars();
        for (i, a_char) in line_iter.enumerate() {
            horizontal.row[i] = match a_char {
                '.' => Cell::Empty,
                '#' => Cell::Tree,
                _ => panic!("Unexpected Character in input.")
            };
        }
        
        horizontal
    }
}

impl Index<usize> for Horizontal {
    type Output = Cell;

    fn index(&self, i: usize) ->  &Self::Output {
        &self.row[i % CELL_WIDTH]
    }
}


fn read_lines (filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<(), Error>{
    let filename = Path::new("input.txt");
    let lines = read_lines(filename)?;

    let mut hill: Vec<Horizontal> =  vec![];

    for line in lines {
        if let Ok(ip) = line {
            let horizontal = Horizontal::new(ip);
            hill.push(horizontal);   
        }
    }

    let mut hits = 0;
    let mut horiz_posn = 0;

    let vert: [u32; 5] = [1, 1, 1, 1, 2];
    let horiz: [u32; 5] = [1, 3, 5, 7, 1];
    let mut total: u64 = 1;
    for (horiz_step, vert_step) in horiz.iter().zip(vert.iter()) {

    
        for row_idx in (*vert_step as usize..hill.len() as usize).step_by(*vert_step as usize) {
            horiz_posn += *horiz_step as usize;
            if hill[row_idx][horiz_posn] == Cell::Tree {
                hits += 1;
            }
        }

        horiz_posn = 0;
        total *= hits;
        hits = 0

    }

    println! ("There were {} hits.", total);
    Ok(())
}
