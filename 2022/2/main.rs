use std::fs;

fn calc_score(op: u8, me: u8) -> i32 {
    match op as i32 - me as i32 {
        0 => 3 + me as i32,
        1 | -2 => 0 + me as i32,
        2 | -1 => 6 + me as i32,
        _ => todo!(),
    }
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    let data: i32 = input
        .split('\n')
        .map(|s| s.split_once(' '))
        .flatten()
        .map(|(op, me)| (op.as_bytes()[0]-64, me.as_bytes()[0]-87))
        .map(|(op, me)| calc_score(op, me))
        .sum();

    println!("{:?}", data);
}
