use std::fs;

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    let mut biggest = 0;

    for elf in input.split("\n\n") {
        let mut sum = 0;

        for cal in elf.split('\n') {
            if let Ok(val) = cal.parse::<u32>() {
                sum += val;
            }
        }

        if sum > biggest {
            biggest = sum;
        }

    }
    
    println!("{:#?}", biggest);
}
