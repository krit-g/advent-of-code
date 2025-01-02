use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = read_file().await?;

    let mut sum = 0;
    let mut sum_instructions = 0;
    sum += mul(input.to_string());
    sum_instructions += mul_instructions(input.to_string());

    println!("part1={sum}"); // part1=183380722
    println!("part2={sum_instructions}"); // part2=82733683

    Ok(())
}

async fn read_file() -> anyhow::Result<String> {
    let mut file = File::open("day3/input-1.txt").await?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).await?;

    Ok(buf)
}

fn mul(string: String) -> u32 {
    let mut sum = 0;

    for s in string.split("mul(") {
        let l: Vec<&str> = s.split(")").collect();
        if let Some(s) = l.first() {
            let nums = s.split(",");
            let nums_count = nums.clone().count();
            if nums_count == 2 {
                let mut local_mul = 1;
                let mut cont = false;
                for n in nums {
                    let res: Result<u32, _> = n.parse();
                    match res {
                        Ok(n) => local_mul *= n,
                        Err(_) => {
                            cont = true;
                            break;
                        }
                    }
                }
                if !cont {
                    sum += local_mul;
                }
            }
        }
    }

    sum
}

fn mul_instructions(string: String) -> u32 {
    let mut sum = 0;
    let mut is_do = true;

    for s in string.split("mul(") {
        let l: Vec<&str> = s.split(")").collect();
        if let Some(s) = l.first() {
            let nums = s.split(",");
            let nums_count = nums.clone().count();
            if nums_count == 2 {
                let mut local_mul = 1;
                let mut cont = false;
                for n in nums {
                    let res: Result<u32, _> = n.parse();
                    match res {
                        Ok(n) => local_mul *= n,
                        Err(_) => {
                            cont = true;
                            break;
                        }
                    }
                }
                if !cont && is_do {
                    sum += local_mul;
                }
            }
        }

        if s.contains("do()") {
            is_do = true;
        } else if s.contains("don't()") {
            is_do = false;
        }
    }

    sum
}
