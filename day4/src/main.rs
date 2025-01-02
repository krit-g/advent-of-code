use itertools::Itertools;
use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = to_2d_array(read().await?);

    println!("part1={}", search(input)); // part1=2545

    Ok(())
}

async fn read() -> anyhow::Result<String> {
    let mut file = File::open("day4/input-1.txt").await?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).await?;

    Ok(buf)
}

fn to_2d_array(input: String) -> Vec<Vec<char>> {
    input
        .lines()
        .collect_vec()
        .iter()
        .map(|s| s.chars().collect_vec())
        .collect_vec()
}

fn search(input: Vec<Vec<char>>) -> usize {
    let mut num = 0;

    let mut marked_input = input.clone();
    for x in 0..marked_input.first().unwrap().len() {
        for y in 0..marked_input.len() {
            marked_input[x][y] = '0';
        }
    }

    for x in 0..input.first().unwrap().len() {
        for y in 0..input.clone().len() {
            if input[x][y] == 'X' {
                for n in (0..8) {
                    println!("----------");
                    if is_xmas(input.clone(), x as i32, y as i32, n, 'X') {
                        println!("found xmas starting at x={x} y={y} direction={n}");
                        num += 1;

                        mark_direction(&mut marked_input, x as i32, y as i32, n, 4);
                    }
                }
            }
        }
    }

    println!("{:?}", marked_input);

    num
}

fn is_xmas(input: Vec<Vec<char>>, x: i32, y: i32, direction: usize, c: char) -> bool {
    println!(
        "is_xmas: x={x}, y={y}, direction={direction}, search={c}, at={}",
        input[x as usize][y as usize]
    );
    if input[x as usize][y as usize] != c {
        return false;
    }

    if c == 'S' {
        return true;
    }

    fn x_out_of_bounds(input: &Vec<Vec<char>>, x: i32) -> bool {
        x < 0 || x >= input.first().unwrap().len() as i32
    }

    fn y_out_of_bounds(input: &Vec<Vec<char>>, y: i32) -> bool {
        y < 0 || y >= input.len() as i32
    }

    let (nx, ny) = next_direction(x, y, direction);
    println!("next_direction: x={nx}, y={ny}");
    if x_out_of_bounds(&input, nx) || y_out_of_bounds(&input, ny) {
        println!("out of bounds: x={nx}, y={ny}");
        return false;
    }

    match c {
        'X' => is_xmas(input, nx, ny, direction, 'M'),
        'M' => is_xmas(input, nx, ny, direction, 'A'),
        'A' => is_xmas(input, nx, ny, direction, 'S'),
        'S' => true,
        _ => false,
    }
}

///
/// [x-1][y-1]   [x][y-1]   [x+1][y-1]
///  [x-1][y]     [x,y]      [x+1][y]
/// [x-1][y+1]   [x][y+1]   [x+1][y+1]
///
fn next_direction(x: i32, y: i32, direction: usize) -> (i32, i32) {
    match direction {
        0 => (x - 1, y - 1),
        1 => (x, y - 1),
        2 => (x + 1, y - 1),
        3 => (x + 1, y),
        4 => (x + 1, y + 1),
        5 => (x, y + 1),
        6 => (x - 1, y + 1),
        7 => (x - 1, y),
        _ => unreachable!(),
    }
}

fn mark_direction(marked_input: &mut Vec<Vec<char>>, x: i32, y: i32, direction: usize, num: usize) {
    marked_input[x as usize][y as usize] = '1';

    if num == 1 {
        return;
    }

    let (nx, ny) = match direction {
        0 => (x - 1, y - 1),
        1 => (x, y - 1),
        2 => (x + 1, y - 1),
        3 => (x + 1, y),
        4 => (x + 1, y + 1),
        5 => (x, y + 1),
        6 => (x - 1, y + 1),
        7 => (x - 1, y),
        _ => unreachable!(),
    };

    mark_direction(marked_input, nx, ny, direction, num - 1);
}

#[cfg(test)]
mod test {
    use crate::{search, to_2d_array};

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[tokio::test]
    async fn test() {
        let array = to_2d_array(INPUT.to_string());
        println!("{:#?}", array);
        println!("{:?}", search(array));
    }
}
