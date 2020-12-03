use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read <R: Read> (io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then( |v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn read2 <R: Read> (io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        v.push (line?
                .trim()
                .parse()
                .map_err(|e| Error::new (ErrorKind::InvalidData, e))?);
    }
    Ok(v)
}

fn main() -> Result <(), Error> {
    let filename = "input.txt";
    let target = 2020;

    let file = File::open(filename)?;
    let vec = read2(file)?; 

    for i in 0..vec.len() {
        for j in i+1..vec.len() {
            if vec[i] + vec[j] == target {
                println! ("{} and {} sum to {}, their product is {}", vec[i], vec[j], target, vec[i]*vec[j]);
                break;
            }
        }
    }

    Ok(())
}
