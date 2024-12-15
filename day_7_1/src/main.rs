use std::usize;

use itertools::iproduct;

fn read_input() -> Vec<(u64, Vec<u64>)> {
    use std::fs::File;
    use std::io::{self, BufRead};

    let mut result: Vec<(u64, Vec<u64>)> = vec![];

    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines().flatten() {
        let (sum_str, mut values_str) = line.split_once(':').unwrap();
        let sum = sum_str.parse::<u64>().unwrap();
        values_str = values_str.trim();

        let values = values_str
            .split(' ')
            .flat_map(|value_str| value_str.parse::<u64>())
            .collect::<Vec<u64>>();
        result.push((sum, values));
    }

    return result;
}

fn read_bit_at_index(num: u64, index: usize) -> bool {
    return num & (1 << index) != 0;
}

fn generate_combinations(size: usize) {
    let symbols = vec!["*".to_string(), "+".to_string(), "||".to_string()];
    let combinations = vec![symbols; size];

    // Generate Cartesian product of the symbols repeated `size` times
    for combination in iproduct!(combinations) {
        println!("{:?}", combination);
    }
}

fn main() {
    generate_combinations(4);

    // let input = read_input();
    // let mut total_calibration = 0u64;

    // for (sum, values) in &input {
    //     println!("{}: {:?}", sum, values);
    // }

    // println!("\nResults:\n");

    // for (sum, values) in input {
    //     for val in 0..1 << values.len() {
    //         let mut index = 0;
    //         let (first, rest) = values.split_first().unwrap();
    //         let result = rest.iter().fold(*first, |lhs, rhs| {
    //             let cpy = index;
    //             index += 1;
    //             if read_bit_at_index(val, cpy) == false {
    //                 return lhs * rhs;
    //             } else {
    //                 return lhs + rhs;
    //             }
    //         });

    //         if result == sum {
    //             let (first, rest) = values.split_first().unwrap();
    //             print!("{sum} = {first} ");

    //             for i in 0..rest.len() {
    //                 if read_bit_at_index(val, i) == false {
    //                     print!("* ");
    //                 } else {
    //                     print!("+ ");
    //                 }
    //                 print!("{} ", rest[i]);
    //             }
    //             println!();

    //             total_calibration += sum;
    //             break;
    //         }
    //     }
    // }

    // println!("Total calibration result: {total_calibration}");
}
