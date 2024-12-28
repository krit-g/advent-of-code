use itertools::Itertools;
use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (left, right) = read_file().await?;

    part1(left.clone(), right.clone()).await;

    part2(left.clone(), right.clone()).await;

    Ok(())
}

async fn read_file() -> anyhow::Result<(Vec<i32>, Vec<i32>)> {
    let mut file = File::open("../input-1.txt").await?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).await?;

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    buf.lines().for_each(|s| {
        let nums: Vec<_> = s.split(' ').collect();
        left.push(nums.first().unwrap().to_string().parse().unwrap());
        right.push(nums.last().unwrap().to_string().parse().unwrap());
    });

    left.sort();
    right.sort();

    Ok((left, right))
}

async fn part1(left: Vec<i32>, right: Vec<i32>) {
    let mut sum = 0;
    left.iter().zip(right).for_each(|(f, s)| {
        sum += (f - s).abs();
    });

    println!("part1={sum}");
}

async fn part2(left: Vec<i32>, right: Vec<i32>) {
    let map = right.iter().counts();
    let mut sum: usize = 0;

    left.iter().for_each(|n| {
        let count = map.get(n).unwrap_or(&0);
        sum += (*n as usize) * count;
    });

    println!("part2={sum}");
}
