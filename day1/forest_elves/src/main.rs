// Part 1 -- Done
// Part 2 -- Done
use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let answer = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .coalesce(|a, b| match (a, b) {
            (None, None) => Ok(None),
            (None, Some(b)) => Ok(Some(b)),
            (Some(a), Some(b)) => Ok(Some(a + b)),
            (Some(a), None) => Err((Some(a), None)),
        })
        .flatten()
        .sorted_by_key(|&v| u64::MAX - v)
        .take(3)
        .sum::<u64>();
    println!("{answer:?}");

    Ok(())
}
