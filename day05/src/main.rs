use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let nums: Vec<isize> = lines.iter().map(|line| line.parse().unwrap()).collect();

    // println!("{:?}", nums);

    let ans_a = part_a(nums.clone());
    let ans_b = part_b(nums.clone());

    println!("{}\n{}", ans_a, ans_b);

    Ok(())
}

fn part_a(mut message: Vec<isize>) -> isize {
    let mut depth = 0;
    let mut position: isize = 0;

    while position < message.len().try_into().unwrap() {
        let jump = message[position as usize];
        depth += 1;
        message[position as usize] += 1;
        // println!("{}, {}", jump, position);
        position += jump;
    }

    depth
}

fn part_b(mut message: Vec<isize>) -> isize {
    let mut depth = 0;
    let mut position: isize = 0;

    while position < message.len().try_into().unwrap() {
        let jump = message[position as usize];
        depth += 1;
        if jump >= 3 {
            message[position as usize] -= 1;
        } else {
            message[position as usize] += 1;
        }
        // println!("{}, {}", jump, position);
        position += jump;
    }

    depth
}

#[cfg(test)]
mod tests {
    use crate::part_a;

    #[test]
    fn test_one() {
        assert_eq!(part_a(vec![0, 3, 0, 1, -3]), 5)
    }
}
