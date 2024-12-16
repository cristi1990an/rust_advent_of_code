use itertools::Itertools;

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

fn generate_combinations_helper(
    n: u32,
    current: Vec<&'static str>,
    result: &mut Vec<Vec<&'static str>>,
    operators: &[&'static str],
) {
    if current.len() == n.try_into().unwrap() {
        result.push(current);
        return;
    }

    for &op in operators {
        let mut next = current.clone();
        next.push(op);
        generate_combinations_helper(n, next, result, operators);
    }
}

fn generate_combinations(n: u32, operators: &[&'static str]) -> Vec<Vec<&'static str>> {
    let mut result: Vec<Vec<&'static str>> = Vec::with_capacity(operators.len().pow(n));
    generate_combinations_helper(
        n,
        Vec::with_capacity(operators.len()),
        &mut result,
        operators,
    );

    return result;
}

fn main() {
    let input = read_input();
    let mut total_calibration_result = 0u64;

    let evaluator = |lhs: u64, (rhs, operation): (&u64, &str)| -> u64 {
        match operation {
            "+" => lhs + rhs,
            "*" => lhs * rhs,
            "||" => format!("{}{}", lhs, rhs).parse::<u64>().unwrap(),
            _ => panic!("Shouldn't happen"),
        }
    };

    for (sum, operators) in input {
        println!("{}: {:?}", sum, operators);

        for combo in
            generate_combinations((operators.len() - 1).try_into().unwrap(), &["+", "*", "||"])
        {
            let (first, rest) = operators.split_first().unwrap();
            let zipped_with_operation = rest.iter().zip(combo.iter().cloned());
            let result = zipped_with_operation.fold(*first, evaluator);

            if result == sum {
                print!("Solution found: ");
                print!("{}", operators.first().unwrap());

                for (op, value) in combo.iter().zip(operators.split_first().unwrap().1) {
                    print!("{op}{value}");
                }

                println!();

                total_calibration_result += sum;

                break;
            }
        }
    }

    println!("Total calibration result = {total_calibration_result}");
}
