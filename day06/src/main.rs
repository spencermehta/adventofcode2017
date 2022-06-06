use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let banks: Vec<usize> = lines[0].split('\t').map(|c| c.parse().unwrap()).collect();

    // println!("{:?}", nums);

    let ans_a = cycles(banks.clone());
    let ans_b = loops(banks);

    println!("{}\n{}", ans_a, ans_b);

    Ok(())
}

fn loops(banks: Vec<usize>) -> usize {
    let mut bank_states = HashMap::new();

    let mut cur_state = banks;
    let mut cycles = 0;
    loop {
        if bank_states.contains_key(&cur_state) {
            break;
        }
        bank_states.insert(cur_state.clone(), cycles);
        let new_cur_state = redistribute(&cur_state);
        cur_state = new_cur_state;
        cycles += 1;
    }

    cycles - bank_states.get(&cur_state).unwrap()
}

fn cycles(banks: Vec<usize>) -> usize {
    let mut bank_states = HashSet::new();

    let mut cur_state = banks;
    let mut cycles = 0;
    loop {
        if bank_states.contains(&cur_state) {
            break;
        }
        bank_states.insert(cur_state.clone());
        let new_cur_state = redistribute(&cur_state);
        cur_state = new_cur_state;
        cycles += 1;
    }

    cycles
}

fn redistribute(banks: &Vec<usize>) -> Vec<usize> {
    let (max_index, max) = max_index(banks);

    let mut b = banks.clone();

    b[max_index] = 0;
    let l = b.len();

    for n in max_index + 1..max_index + 1 + max {
        b[n % l] += 1;
    }

    b
}

fn max_index(banks: &Vec<usize>) -> (usize, usize) {
    let max = banks.iter().max().unwrap();

    (banks.iter().position(|el| el == max).unwrap(), *max)
}

#[cfg(test)]
mod tests {
    use crate::{cycles, loops, max_index, redistribute};

    #[test]
    fn gets_max_integer_index() {
        assert_eq!(max_index(&vec![1, 2, 3]), (2, 3));
        assert_eq!(max_index(&vec![1, 4, 3]), (1, 4));
    }

    #[test]
    fn redistributes_once() {
        assert_eq!(redistribute(&vec![0, 2, 7, 0]), vec![2, 4, 1, 2])
    }

    #[test]
    fn gets_cycles() {
        assert_eq!(cycles(vec![0, 2, 7, 0]), 5)
    }

    #[test]
    fn gets_loops() {
        assert_eq!(loops(vec![0, 2, 7, 0]), 4)
    }
}
