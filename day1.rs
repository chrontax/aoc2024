use std::{collections::HashMap, error::Error, fs::read_to_string, ops::Sub};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("day1.in")?;
    let len = input.lines().count();
    let mut left = vec![0; len];
    let mut right = vec![0; len];
    for ((mut row, left), right) in input
        .lines()
        .map(|l| l.split_whitespace().map(str::parse::<i32>))
        .zip(left.iter_mut())
        .zip(right.iter_mut())
    {
        *left = row.next().unwrap()?;
        *right = row.next().unwrap()?;
    }

    println!(
        "{}",
        // part1(&mut left, &mut right)?
        part2(&left, &right)
    );

    Ok(())
}

fn part1(left: &mut [i32], right: &mut [i32]) -> Result<i32, Box<dyn Error>> {
    left.sort();
    right.sort();

    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(&a, &b)| a.sub(b).abs())
        .sum::<i32>())
}

fn part2(left: &[i32], right: &[i32]) -> i32 {
    let map = right.iter().fold(HashMap::<_, i32>::new(), |mut acc, &n| {
        *acc.entry(n).or_default() += 1;
        acc
    });

    left.iter()
        .map(|n| n * map.get(n).copied().unwrap_or_default())
        .sum()
}
