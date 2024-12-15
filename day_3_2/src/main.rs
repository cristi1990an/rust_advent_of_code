fn read_input() -> String {
    let result = std::fs::read_to_string("input.txt").unwrap();
    return result;
}

fn parse_parameters(input: &str) -> Option<(u32, u32, &str)> {
    // Split by ')' into to slices
    let (trimmed, rest) = match input.split_once(')') {
        Some(slit) => slit,
        None => return None,
    };

    // Split by ',' into two slices.
    let (lhs_str, rhs_str) = match trimmed.split_once(',') {
        // If ',' was found, split was successful, return the two slices.
        Some(slit) => slit,
        // Continue the for loop.
        None => return None,
    };

    // Try parsing both slices into u32.
    let lhs = match lhs_str.parse::<u32>() {
        Ok(lhs) => lhs,
        Err(_) => return None,
    };

    let rhs: u32 = match rhs_str.parse::<u32>() {
        Ok(rhs) => rhs,
        Err(_) => return None,
    };

    return Some((lhs, rhs, rest));
}

mod util {
    fn parse_do_dont(str: &str) -> Option<bool> {
        // Find the index of the last occurence of the "do()" operation.
        let enabler = str.rfind("do()");
        // Find the index of the last occurence of the "don't()" operation.
        let disabler = str.rfind("don't()");

        return match (enabler, disabler) {
            // If neither found, don't return any new state.
            (None, None) => None,
            // If only "don't()"" found return disabled state.
            (None, Some(_)) => Some(false),
            // If only "do()"" found return enabled state.
            (Some(_), None) => Some(true),
            // If both found, check which is last and return based on that.
            (Some(index_of_do), Some(index_of_dont)) => Some(index_of_do > index_of_dont),
        };
    }

    pub struct DoDontState {
        enabled: bool,
    }

    impl DoDontState {
        pub fn new() -> Self {
            // Default state is enabled = true.
            return DoDontState { enabled: true };
        }

        pub fn parse(&mut self, str: &str) {
            // Take the new state or keep initial one
            self.enabled = parse_do_dont(str).unwrap_or(self.enabled);
        }

        pub fn is_enabled(&self) -> bool {
            return self.enabled;
        }
    }
}

fn parse_memory(input: &String) -> Vec<(u32, u32)> {
    let mut result: Vec<(u32, u32)> = vec![];
    let mut state = util::DoDontState::new();

    // Split file by "mul(" delimiter, each resulting slice possibly begining with a valid continuation.
    // Ex:      xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)
    // Result:  ["x", "2,4)%&mul[3,7]!@^do_not_", "5,5)"]
    for str in input.split("mul(") {
        // parse_parameters returns a tuple of the parsed left and right numbers
        // in the operation + the rest of the string.
        // If operation wasn't successful, returns None.
        match parse_parameters(str) {
            // We print what we parsed and if state is enabled we add the operation
            // to the vector, or ignore it otherwise.
            // Regardless we parse the rest of the string, looking for futher
            // do()/don't() instructions.
            Some((lhs, rhs, rest)) => {
                if state.is_enabled() {
                    result.push((lhs, rhs));
                    println!("mul({},{})", lhs, rhs);
                } else {
                    println!("mul({},{}) (ignored)", lhs, rhs);
                }

                state.parse(rest);
            }
            // If it wasn't successful and no valid mul(x,y) operation was written
            // we parse the whole string for do()/dont() operations.
            None => {
                state.parse(str);
                continue;
            }
        };
    }

    return result;
}

fn main() {
    let input = read_input();
    let operations = parse_memory(&input);

    let sum_product = operations.iter().map(|(lhs, rhs)| lhs * rhs).sum::<u32>();
    println!("Sum: {sum_product}");
}
