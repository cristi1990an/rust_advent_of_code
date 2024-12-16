fn read_input() -> Vec<(u64, Vec<u64>)> {
    use std::fs::File;
    use std::io::{self, BufRead};

    let mut result: Vec<(u64, Vec<u64>)> = vec![];

    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines().flatten() {
        // Split the line between the sum and the values.
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

mod util {
    use std::collections::HashMap;

    fn generate_combinations_helper(
        n: u64,
        current: Vec<&'static str>,
        result: &mut Vec<Vec<&'static str>>,
        operators: &[&'static str],
    ) {
        if current.len() == n.try_into().unwrap() {
            // Save result if we computed a combination with the max length.
            result.push(current);
            return;
        }

        for &op in operators {
            // Next 2 lines are essentially a .clone() but we're ensuring that the copy will
            // have the same preallocated capacity as the original.
            let mut next: Vec<&str> = Vec::with_capacity(current.capacity());
            // Clones and appends elements from current.
            next.extend_from_slice(&current);
            next.push(op);
            generate_combinations_helper(n, next, result, operators);
        }
    }

    fn generate_combinations(n: u64, operators: &[&'static str]) -> Vec<Vec<&'static str>> {
        // We know the resulting vector will have <num_of_operators> to the power of <n> elements,
        // so we use the Vec::with_capacity method to create the vector with enough preallocated memory
        // from the start.
        let mut result: Vec<Vec<&'static str>> =
            Vec::with_capacity(operators.len().pow(n.try_into().unwrap()));
        generate_combinations_helper(
            n,
            Vec::with_capacity(operators.len()),
            &mut result,
            operators,
        );

        return result;
    }
    pub(crate) struct CombinationRegerator {
        cache: HashMap<u64, Vec<Vec<&'static str>>>,
        operators: Vec<&'static str>,
    }

    // Class that creates all the possible combinations of length n over the given operations.
    // For n = 4 and op = ["+", "*"] the class generates a vector containing:
    // ["+", "+", "+", "+"],
    // ["+", "+", "+", "*"],
    // ["+", "+", "*", "+"],
    // ["+", "+", "*", "*"],
    // ...
    // ["*", "*", "*", "*"]
    //
    // We're encapsulating this into a class in order to also cache the computed result for each n.
    impl CombinationRegerator {
        // Constructing the instance requires a list of operations supported being passed.
        pub fn new(operators: Vec<&'static str>) -> Self {
            Self {
                cache: HashMap::new(),
                operators,
            }
        }

        // Returns the vector of combinations for a given n length.
        // If the result was already computed before, we return the cached result,
        // otherwise we do the new computation.
        pub fn get(&mut self, n: u64) -> &Vec<Vec<&'static str>> {
            return self
                .cache
                .entry(n)
                .or_insert(generate_combinations(n, &self.operators));
        }
    }
}

// Function for the fold algorithm taking the left value and a tuple between the right value
// and the string representing the operation to be applied between them. 
fn evaluator(lhs: u64, (rhs, operation): (&u64, &&str)) -> u64 {
    // In Rust we can match any value type, including strings.
    match *operation {
        "+" => lhs + rhs,
        "*" => lhs * rhs,
        // For concatenation we create the string representation of the two values concatenated
        // and then we parse the result back into u64.
        "||" => format!("{}{}", lhs, rhs).parse::<u64>().unwrap(),
        _ => panic!("Shouldn't happen"),
    }
}

fn main() {
    let input = read_input();
    let mut total_calibration_result = 0u64;
    // This is a valid solution for both day 1 and 2, simply add/remove the "||" operator.
    let mut generator = util::CombinationRegerator::new(vec!["+", "*", "||"]);

    for (sum, values) in input {
        println!("{}: {:?}", sum, values);

        // One operation must be placed between each value, so the length of the vector of operation combinations must
        // be the length of the values - 1.
        for combo in generator.get((values.len() - 1).try_into().unwrap()) {
            // Split off the initial value so we have our init value for the fold algorithm. 
            let (first, rest) = values.split_first().unwrap();
            // Zip the rest of the values alongside an operation.
            let zipped_with_operation = rest.iter().zip(combo.iter());
            // Fold is Rust's equivalent of std::accumulate or std::fold_left, it requires an initial value and a predicate.
            let result = zipped_with_operation.fold(*first, evaluator);

            // If our evaluated expression results in the specified sum, add said sum to total_calibration_result.
            if result == sum {
                print!("Solution found: ");
                print!("{}", values.first().unwrap());

                for (op, value) in combo.iter().zip(&values[1..]) {
                    print!("{op}{value}");
                }

                println!();

                total_calibration_result += sum;

                break;
            }
        }
    }

    // 169122112716571
    println!("Total calibration result = {total_calibration_result}");
}
