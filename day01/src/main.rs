use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let nums: Vec<u32> = lines[0]
        .chars()
        .map(|x| x.to_digit(10))
        .map(|x| x.unwrap())
        .collect();

    let ans_a = part_a(&nums);
    let ans_b = part_b(&nums);

    println!("{}\n{}", ans_a, ans_b);

    Ok(())
}

fn part_a(nums: &Vec<u32>) -> u32 {
    let mut prev = &nums[0];
    let mut count = 0;

    for curr in nums.iter().skip(1) {
        if curr == prev {
            count += curr;
        }
        prev = curr;
    }

    if nums[nums.len() - 1] == nums[0] {
        count += nums[0];
    }

    count
}

fn part_b(nums: &Vec<u32>) -> u32 {
    let mut count = 0;

    for (i, curr) in nums.iter().enumerate() {
        if curr == &nums[(i + (nums.len() / 2)) % nums.len()] {
            count += curr;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::part_a;
    use crate::part_b;

    #[test]
    fn practice_one() {
        let ans = part_a(&[1, 1, 2, 2].to_vec());
        assert_eq!(ans, 3)
    }

    #[test]
    fn practice_two() {
        let ans = part_a(&[1, 1, 1, 1].to_vec());
        assert_eq!(ans, 4)
    }

    #[test]
    fn practice_three() {
        let ans = part_a(&[1, 2, 3, 4].to_vec());
        assert_eq!(ans, 0)
    }

    #[test]
    fn practice_four() {
        let ans = part_a(&[9, 1, 2, 1, 2, 1, 2, 9].to_vec());
        assert_eq!(ans, 9)
    }

    #[test]
    fn practice_one_b() {
        let ans = part_b(&[1, 2, 1, 2].to_vec());
        assert_eq!(ans, 6)
    }

    #[test]
    fn practice_two_b() {
        let ans = part_b(&[1, 2, 2, 1].to_vec());
        assert_eq!(ans, 0)
    }

    #[test]
    fn practice_three_b() {
        let ans = part_b(&[1, 2, 3, 4, 2, 5].to_vec());
        assert_eq!(ans, 4)
    }

    #[test]
    fn practice_four_b() {
        let ans = part_b(&[1, 2, 3, 1, 2, 3].to_vec());
        assert_eq!(ans, 12)
    }

    #[test]
    fn practice_five_b() {
        let ans = part_b(&[1, 2, 1, 3, 1, 4, 1, 5].to_vec());
        assert_eq!(ans, 4)
    }
}
