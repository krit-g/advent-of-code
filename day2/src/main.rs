use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let list = read_file().await?;

    let safe_reports = list.iter().filter(|l| is_safe(l.to_vec())).count();
    println!("part1={safe_reports}"); // 257

    let safe_reports_dampener = list.iter().filter(|l| is_safe_dampener(l.to_vec())).count();
    println!("part2={safe_reports_dampener}");

    Ok(())
}

async fn read_file() -> anyhow::Result<Vec<Vec<i32>>> {
    let mut file = File::open("day2/input-1.txt").await?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).await?;

    let mut list: Vec<Vec<i32>> = vec![];

    buf.lines().for_each(|s| {
        let nums: Vec<_> = s.split(' ').collect();
        let mut inner_list: Vec<i32> = vec![];

        nums.iter()
            .for_each(|n| inner_list.push(n.to_string().parse().unwrap()));

        list.push(inner_list);
    });

    Ok(list)
}

///
/// 1/ The levels are either all increasing or all decreasing.
/// 2/ Any two adjacent levels differ by at least one and at most three.
///
fn is_safe(report: Vec<i32>) -> bool {
    let is_increasing = report.first().unwrap() < report.get(1).unwrap();

    for idx in 0..report.len() - 1 {
        let first = report.get(idx).unwrap();
        let second = report.get(idx + 1).unwrap();

        if is_increasing {
            if first > second {
                return false;
            }
        } else if first < second {
            return false;
        }

        let diff = (first - second).abs();
        if !(1..=3).contains(&diff) {
            return false;
        }
    }

    true
}

///
/// 1/ The levels are either all increasing or all decreasing.
/// 2/ Any two adjacent levels differ by at least one and at most three.
///
/// tolerate a single bad level
///
fn is_safe_dampener(report: Vec<i32>) -> bool {
    //
    // Attempt 3
    //

    if is_safe(report.clone()) {
        return true;
    }

    let mut safe = false;
    for idx in 0..report.len() {
        let mut clone = report.clone();
        clone.remove(idx);
        if is_safe(clone.clone()) {
            safe = true;
            break;
        }
    }
    safe

    //
    // Attempt 2
    //

    // let mut differences: Vec<i32> = vec![];
    // for idx in 0..report.len() - 1 {
    //     let first = report.get(idx).unwrap();
    //     let second = report.get(idx + 1).unwrap();
    //     differences.push(first - second);
    // }

    // let mut is_safe = true;

    // let num_inc = differences.iter().filter(|n| **n > 0).count();
    // let num_dec = differences.iter().filter(|n| **n < 0).count();

    // if num_inc > 1 && num_dec > 1 {
    //     is_safe = false;
    // }

    // if differences
    //     .iter()
    //     .filter(|n| !(1..=3).contains(&(*n).abs()))
    //     .count()
    //     > 1
    // {
    //     is_safe = false;
    // }

    // if !is_safe {
    //     println!();
    //     println!("Report: {:?}", report);
    //     println!("Differences: {:?}", differences);
    //     println!("Inc: {}, Dec: {}", num_inc, num_dec);
    // }

    // is_safe

    //
    // Attempt 1
    //

    // let is_increasing = report.first().unwrap() < report.get(1).unwrap();
    // let mut problem = usize::MAX;

    // for idx in 0..report.len() - 1 {
    //     let first = report.get(idx).unwrap();
    //     let second = report.get(idx + 1).unwrap();

    //     if is_increasing {
    //         if first > second {
    //             problem = idx + 1;
    //             break;
    //         }
    //     } else if first < second {
    //         problem = idx + 1;
    //         break;
    //     }

    //     let diff = (first - second).abs();
    //     if !(1..=3).contains(&diff) {
    //         problem = idx + 1;
    //         break;
    //     }
    // }

    // if problem == usize::MAX {
    //     true
    // } else {
    //     let mut dampened_report = report.clone();
    //     dampened_report.remove(problem);

    //     let safe = is_safe(dampened_report.clone());
    //     if !safe {
    //         println!();
    //         println!("Unsafe at {}: {:?}", problem, report);
    //         println!("IsSafe: {}: {:?}", safe, dampened_report);
    //     }
    //     safe
    // }
}
