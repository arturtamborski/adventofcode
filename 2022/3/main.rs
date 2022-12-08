use std::collections::HashSet;
use std::fs;

fn prioritize(s: &str) -> Vec<u32> {
    s.chars()
        .map(|c| match c {
            'a'..='z' => (c as u32 - 96),
            'A'..='Z' => (c as u32 - 38),
            _ => panic!(),
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();

    let data: Vec<Vec<Vec<u32>>> = input
        .split('\n')
        .filter(|&x| !x.is_empty())
        .map(prioritize)
        .map(|v| v.chunks(v.len() / 2).map(|s| s.into()).collect())
        .collect();

    let mut sum = 0;
    for bag in data {
        let a = bag[0].iter().cloned().collect::<HashSet<u32>>();
        let b = bag[1].iter().cloned().collect::<HashSet<u32>>();
        let i = a.intersection(&b).collect::<Vec<&u32>>();
        sum += i.iter().cloned().sum::<u32>();
    }

    println!("{:?}", sum);
}
