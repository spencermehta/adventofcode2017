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
    let ans_b = part_b(&nums);

    println!("{}\n{}", ans_a, ans_b);

    Ok(())
}

fn part_a(lines: &[Vec<String>]) -> u32 {
    lines
        .iter()
        .map(valid_passphrase)
        .fold(0, |acc, e| if e { acc + 1 } else { acc })
}

fn part_b(lines: &[Vec<String>]) -> u32 {
    lines
        .iter()
        .map(valid_passphrase_anagrams)
        .fold(0, |acc, e| if e { acc + 1 } else { acc })
}

fn valid_passphrase(phrase: &Vec<String>) -> bool {
    let mut p = phrase.clone();
    p.sort();
    !p.iter()
        .enumerate()
        .map(|(index, word)| {
            if index == p.len() - 1 {
                return false;
            }
            word == &p[index + 1]
        })
        .reduce(|acc, e| acc || e)
        .unwrap()
}

fn valid_passphrase_anagrams(phrase: &Vec<String>) -> bool {
    let mut p = phrase.clone();
    p = p
        .iter()
        .map(|word| {
            let mut chars: Vec<char> = word[..].chars().collect();
            chars.sort();
            chars.iter().collect()
        })
        .collect();
    p.sort();
    !p.iter()
        .enumerate()
        .map(|(index, word)| {
            if index == p.len() - 1 {
                return false;
            }
            word == &p[index + 1]
        })
        .reduce(|acc, e| acc || e)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{valid_passphrase, valid_passphrase_anagrams};

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

    #[test]
    fn case_b_one() {
        assert!(valid_passphrase_anagrams(&vec![
            String::from("abcde"),
            String::from("fghij"),
        ]))
    }

    #[test]
    fn case_b_two() {
        assert!(!valid_passphrase_anagrams(&vec![
            String::from("abcde"),
            String::from("xyz"),
            String::from("ecdab"),
        ]))
    }

    #[test]
    fn case_b_three() {
        assert!(valid_passphrase_anagrams(&vec![
            String::from("a"),
            String::from("ab"),
            String::from("abc"),
            String::from("abd"),
            String::from("abf"),
            String::from("abj"),
        ]))
    }

    #[test]
    fn case_b_four() {
        assert!(valid_passphrase_anagrams(&vec![
            String::from("iiii"),
            String::from("oiii"),
            String::from("ooii"),
            String::from("oooi"),
            String::from("oooo"),
        ]))
    }

    #[test]
    fn case_b_five() {
        assert!(!valid_passphrase_anagrams(&vec![
            String::from("oiii"),
            String::from("ioii"),
            String::from("iioi"),
            String::from("iiio"),
        ]))
    }
}
