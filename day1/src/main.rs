use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // Read our list
    let f = std::fs::File::open("input.txt")?;

    let mut numbers: Vec<u32> = vec![];

    for line in BufReader::new(f).lines() {
        let parsed: Result<u32, _> = line?
            .parse()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e));
        numbers.push(parsed?);
    }

    // sort the list so we can search it quickly
    numbers.sort();
    let mut ix_upperbound = numbers.len();

    // look for matching pairs
    for &x in &numbers {
        match numbers[..ix_upperbound].binary_search(&(2020 - x)) {
            Ok(ix) => {
                let y = numbers[ix];
                println!(
                    "Found matching pair! The product of {} and {} is {}.",
                    x,
                    y,
                    x * y
                );
                break;
            }
            Err(ix) => {
                ix_upperbound = ix;
            }
        }
    }

    // look for matching triplets
    'outer: for i in 0..numbers.len() {
        ix_upperbound = numbers.len();

        for j in (i + 1)..numbers.len() {
            let x = numbers[i];
            let y = numbers[j];

            if y + x > 2020 {
                break 'outer;
            }

            match numbers[(j + 1)..ix_upperbound].binary_search(&(2020 - (x + y))) {
                Ok(ix) => {
                    let z = numbers[j + 1 + ix];
                    println!(
                        "Found matching triplet! The product of {}, {} and {} is {}.",
                        x,
                        y,
                        z,
                        x * y * z
                    );
                    break 'outer;
                }
                Err(ix) => {
                    ix_upperbound = ix + j + 1;
                    if ix_upperbound < j + 2 {
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
