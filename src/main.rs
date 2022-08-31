#![allow(dead_code)]
use fetch_data::{ctor, FetchData, FetchDataError};
#[cfg(test)]
use rand::seq::SliceRandom;
#[cfg(test)]
use rand::SeedableRng;
use rand::{Rng, RngCore};
#[cfg(test)]
use rand_xoshiro::Xoshiro256PlusPlus;
use std::error::Error;
#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Write;
#[cfg(test)]
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
#[cfg(test)]
use std::time::Instant;
use tailcall::tailcall;
use thousands::Separable;

#[ctor]
static STATIC_FETCH_DATA: FetchData = FetchData::new(
    "200.txt 89c1562844d4cfa23605d3390a73a5254f6515c90700b2a8d673b56a311b8709",
    "https://www.gutenberg.org/files/200/",
    "RANDOM_LINE_DATA_DIR", // env_key
    "github.com",           // qualifier
    "CarlKCarlK",           // organization
    "random-line",          // application
);

/// Download a data file.
pub fn sample_file<P: AsRef<Path>>(path: P) -> Result<PathBuf, FetchDataError> {
    STATIC_FETCH_DATA.fetch_file(path)
}

#[test]
fn algo_read_all() -> Result<(), anyhow::Error> {
    // Uses fetch-data to download https://www.gutenberg.org/files/200/200.txt
    // if not already downloaded.
    let path = sample_file("200.txt")?;

    // Pick a random line from a file

    // Based on GitHub Copilot's Python suggestion:
    let full_contents = fs::read_to_string(path)?;
    let lines: Vec<&str> = full_contents.lines().collect();
    println!("{:?}", lines.choose(&mut rand::thread_rng()));
    Ok(())
}

#[test]
fn algo_read_all_empty() -> Result<(), anyhow::Error> {
    let lines: Vec<&str> = vec![];
    let result = lines.choose(&mut rand::thread_rng());
    println!("{:?}", result);
    Ok(())
}

#[test]
fn algo_two_pass() -> Result<(), anyhow::Error> {
    // fetch-data from https://www.gutenberg.org/files/200/200.txt
    let path = sample_file("200.txt")?;

    // count lines in file
    let reader = BufReader::new(File::open(&path)?);
    let mut line_count = 0;
    for item_result in reader.lines() {
        item_result?; // check each item for an error
        line_count += 1;
    }

    // pick a random line index
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
    let random_index0 = rng.gen_range(0..line_count);

    // print the line at that index
    let mut random_line: Option<String> = None;
    let reader = BufReader::new(File::open(&path)?);
    for (index0, item_result) in reader.lines().enumerate() {
        let item = item_result?; // check each item for an error
        if index0 == random_index0 {
            random_line = Some(item);
            break;
        }
    }
    println!(
        "{} of {}: {:?}",
        random_index0.separate_with_commas(),
        line_count.separate_with_commas(),
        random_line
    );

    Ok(())
}

#[test]
fn algo_two_pass_functional() -> Result<(), anyhow::Error> {
    // fetch-data from https://www.gutenberg.org/files/200/200.txt
    let path = sample_file("200.txt")?;

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
    let file = File::open(&path)?;
    let line_count = try_count(BufReader::new(&file).lines())?;
    let random_index0 = rng.gen_range(0..line_count);
    let random_line = try_nth(BufReader::new(File::open(&path)?).lines(), random_index0)?;
    println!(
        "{} of {}: {:?}",
        random_index0.separate_with_commas(),
        line_count.separate_with_commas(),
        random_line
    );

    Ok(())
}

#[inline]
fn try_count<I, T, E>(mut iterator: I) -> Result<usize, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    iterator.try_fold(0, |acc, item_result| item_result.map(|_| acc + 1))
}

fn try_nth<I, T, E>(mut iterator: I, n: usize) -> Result<Option<T>, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    for _ in 0..n {
        if iterator.next().transpose()?.is_none() {
            return Ok(None);
        }
    }
    iterator.next().transpose()
}

#[test]
fn algo_one_line() -> Result<(), anyhow::Error> {
    // fetch-data from https://www.gutenberg.org/files/200/200.txt
    let path = sample_file("200.txt")?;

    let random_line = BufReader::new(File::open(&path)?).lines().next().unwrap()?;
    println!("{random_line}");

    Ok(())
}

#[test]
fn algo_two_line() -> Result<(), anyhow::Error> {
    // fetch-data from https://www.gutenberg.org/files/200/200.txt
    let path = sample_file("200.txt")?;

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
    let mut lines = BufReader::new(File::open(&path)?).lines();
    let mut random_line = lines.next().unwrap()?;
    let line = lines.next().unwrap()?;
    if rng.gen::<f64>() < 0.5 {
        random_line = line;
    }
    println!("{random_line}");

    Ok(())
}

#[test]
fn algo_three_line() -> Result<(), anyhow::Error> {
    // fetch-data from https://www.gutenberg.org/files/200/200.txt
    let path = sample_file("200.txt")?;

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
    let mut lines = BufReader::new(File::open(&path)?).lines();
    let mut random_line = lines.next().unwrap()?;
    let line = lines.next().unwrap()?;
    if rng.gen::<f64>() < 0.5 {
        random_line = line;
    }
    let line = lines.next().unwrap()?;
    if rng.gen::<f64>() < 1.0 / 3.0 {
        random_line = line;
    }
    println!("{random_line}");

    Ok(())
}

#[test]
fn algo_one_pass() -> Result<(), anyhow::Error> {
    // fetch-data from https://www.gutenberg.org/files/200/200.txt
    let path = sample_file("200.txt")?;

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
    let mut random_line: Option<String> = None;
    let lines = BufReader::new(File::open(&path)?).lines();

    for (index0, line_result) in lines.enumerate() {
        let line = line_result?; // check each item for an error
        if rng.gen::<f32>() < 1.0 / (index0 + 1) as f32 {
            random_line = Some(line);
        }
    }
    println!("{random_line:?}");

    Ok(())
}

#[tailcall]
fn try_iterator_choose_recur<T, I, R, E>(
    mut iterator: I,
    rng: &mut R,
    index1: usize,
    mut random_item: Option<T>,
) -> Result<Option<T>, E>
where
    I: Iterator<Item = Result<T, E>>,
    R: RngCore,
    E: Error,
{
    match iterator.next() {
        None => Ok(random_item),
        Some(item_result) => {
            let item = item_result?; // check each item for an error
            if rng.gen_range(0..index1) == 0 {
                random_item = Some(item);
            }
            try_iterator_choose_recur(iterator, rng, index1 + 1, random_item)
        }
    }
}

fn try_iterator_choose_one_pass<T, I, R, E>(iterator: I, rng: &mut R) -> Result<Option<T>, E>
where
    I: Iterator<Item = Result<T, E>>,
    R: RngCore,
    E: Error,
{
    let mut random_item: Option<T> = None;
    for (index0, item_result) in iterator.enumerate() {
        let item = item_result?; // check each item for an error
        if rng.gen_range(0..=index0) == 0 {
            random_item = Some(item);
        }
    }
    Ok(random_item)
}

#[test]
fn algo_one_pass_recursive() -> Result<(), anyhow::Error> {
    // fetch-data from https://www.gutenberg.org/files/200/200.txt
    let path = sample_file("200.txt")?;

    let lines = BufReader::new(File::open(&path)?).lines();
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
    let random_item = try_iterator_choose_recur(lines, &mut rng, 1, None)?;
    println!("{random_item:?}");

    Ok(())
}

#[test]
fn time_algo_one_pass_recursive() -> Result<(), anyhow::Error> {
    let start = Instant::now();
    let iterator = (0..10_000_000).map(Ok::<_, std::io::Error>);

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
    let random_item = try_iterator_choose_recur(iterator, &mut rng, 1, None)?;
    println!("{random_item:?}");
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("recursive {:?}", duration);

    let start = Instant::now();
    let iterator = (0..10_000_000).map(Ok::<_, std::io::Error>);

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
    let random_item = try_iterator_choose_one_pass(iterator, &mut rng)?;
    println!("{random_item:?}");
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("iterative {:?}", duration);

    let start = Instant::now();
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
    let iterator = BufReader::new(File::open(&sample_file("200.txt")?)?).lines();
    let random_item = try_iterator_choose_one_pass(iterator, &mut rng)?;
    println!("{random_item:?}");
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("from file {:?}", duration);

    Ok(())
}

fn try_iterator_choose_print<T, I, R, E>(iterator: I, rng: &mut R) -> Result<Option<T>, E>
where
    I: Iterator<Item = Result<T, E>>,
    R: RngCore,
    E: Error,
{
    let mut random_item: Option<T> = None;
    for (index0, item_result) in iterator.enumerate() {
        let item = item_result?; // check each item for an error
        if rng.gen_range(0..=index0) == 0 {
            print!("{} ", (index0 + 1).separate_with_commas());
            random_item = Some(item);
        }
    }
    println!();
    Ok(random_item)
}

#[test]
fn algo_one_pass_print() -> Result<(), anyhow::Error> {
    let iterator = (0..1_000_000).map(Ok::<_, std::io::Error>);

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
    try_iterator_choose_print(iterator, &mut rng)?;

    Ok(())
}

fn try_iterator_choose_skip<T, I, R, E>(mut iterator: I, rng: &mut R) -> Result<Option<T>, E>
where
    I: Iterator<Item = Result<T, E>>,
    R: RngCore,
    E: Error,
{
    let mut offset = 1;
    let mut index1 = 1;
    let mut random_item = None;
    while let Some(item) = try_nth(&mut iterator, offset - 1)? {
        random_item = Some(item);
        let r: f64 = rng.gen();
        offset = ((r * (index1 as f64) / (1.0 - r)).ceil() as usize).max(1);
        index1 += offset;
    }

    Ok(random_item)
}

#[test]
fn algo_one_pass_skip() -> Result<(), anyhow::Error> {
    // fetch-data from https://www.gutenberg.org/files/200/200.txt
    let path = sample_file("200.txt")?;

    let lines = BufReader::new(File::open(&path)?).lines();
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
    let random_item = try_iterator_choose_skip(lines, &mut rng)?;

    println!("{random_item:?}");
    Ok(())
}

#[test]
fn plot_100s() -> Result<(), anyhow::Error> {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
    let item_iterator = (0..100_000).map(|_| {
        try_iterator_choose_skip((0..100).map(Ok::<_, std::io::Error>), &mut rng)
            .unwrap()
            .unwrap()
    });
    // Write iterator to file
    let mut file = File::create("100s.txt")?;
    for item in item_iterator {
        writeln!(file, "{}", item)?;
    }
    Ok(())
}
fn main() {}
