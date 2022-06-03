use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let nums: Vec<Vec<String>> = lines
        .iter()
        .map(|line| {
            line.split(' ')
                .map(|x| x.parse())
                .map(|x| x.unwrap())
                .collect()
        })
        .collect();

    // println!("{:?}", nums);

    let ans_a = part_a(&nums);
    // let ans_b = part_b(&nums);

    println!("{}\n{}", ans_a, false);

    Ok(())
}

fn part_a(lines: &Vec<Vec<String>>) -> u32 {
    lines
        .iter()
        .map(valid_passphrase)
        .fold(0, |acc, e| if e { acc + 1 } else { acc })
}

fn valid_passphrase(phrase: &Vec<String>) -> bool {
    let mut p = phrase.clone(); // this seems sketchy lol
    p.sort();
    let mut q = p.clone();
    q.pop();
    !q.iter()
        .enumerate()
        .map(|(index, word)| word == &p[index + 1])
        .reduce(|acc, e| acc || e)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::valid_passphrase;

    #[test]
    fn no_duplicates() {
        assert!(valid_passphrase(&vec![
            String::from("aa"),
            String::from("bb"),
        ]))
    }

    #[test]
    fn duplicates() {
        assert!(!valid_passphrase(&vec![
            String::from("aa"),
            String::from("bb"),
            String::from("aa"),
        ]))
    }

    #[test]
    fn case_a_one() {
        assert!(valid_passphrase(&vec![
            String::from("aa"),
            String::from("bb"),
            String::from("cc"),
            String::from("dd"),
            String::from("ee"),
        ]))
    }

    #[test]
    fn case_a_two() {
        assert!(!valid_passphrase(&vec![
            String::from("aa"),
            String::from("bb"),
            String::from("cc"),
            String::from("dd"),
            String::from("aa"),
        ]))
    }

    #[test]
    fn case_a_three() {
        assert!(valid_passphrase(&vec![
            String::from("aa"),
            String::from("bb"),
            String::from("cc"),
            String::from("dd"),
            String::from("aaa"),
        ]))
    }
}
