use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let nums: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| {
            line.split('\t')
                .map(|x| x.parse())
                .map(|x| x.unwrap())
                .collect()
        })
        .collect();

    // println!("{:?}", nums);

    let ans_a = part_a(&nums);
    let ans_b = part_b(&nums);

    println!("{}\n{}", ans_a, ans_b);

    Ok(())
}

fn part_a(nums: &Vec<Vec<u32>>) -> u32 {
    nums.iter()
        .map(|num| {
            let max = num.iter().max().unwrap();
            let min = num.iter().min().unwrap();
            max - min
        })
        .into_iter()
        .sum()
}

fn part_b(nums: &Vec<Vec<u32>>) -> u32 {
    let mut diffs: Vec<u32> = vec![];

    for row in nums {
        for (i, num_a) in row.iter().enumerate() {
            for num_b in row.iter().skip(i + 1) {
                if num_a % num_b == 0 {
                    diffs.push(num_a / num_b);
                }
                if num_b % num_a == 0 {
                    diffs.push(num_b / num_a);
                }
            }
        }
    }
    diffs.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::part_a;
    use crate::part_b;

    #[test]
    fn practice_one() {
        let ans = part_a(&vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]]);
        assert_eq!(ans, 18)
    }

    #[test]
    fn practice_two() {
        let ans = part_b(&vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]]);
        assert_eq!(ans, 9)
    }
}
