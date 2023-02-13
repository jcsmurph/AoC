// Part 1 -- Done
// Part 2 -- Done
use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let answer = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .sorted_by_key(|&v| u64::MAX - v)
        .take(3)
        .sum::<u64>();
    println!("{answer:?}");

    Ok(())
}
