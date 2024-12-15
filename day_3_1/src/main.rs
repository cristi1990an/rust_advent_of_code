fn read_input() -> String {
    let result = std::fs::read_to_string("input.txt").unwrap();
    return result;
}

fn parse_memory(input: &String) -> Vec<(u32, u32)> {
    let mut result: Vec<(u32, u32)> = vec![];
    for str in input.split_inclusive("mul(") {
        // Match the result of the .find() function. Cases Some()/None must be treated.
        let trimmed = match str.find(')') {
            // If character was found, use the 'end' index and return a slice of the string
            // that ends at that index.
            Some(end) => &str[..end],
            // If ')' wasn't found, continue the for loop.
            None => continue,
        };

        // Split by ',' into two slices.
        let (lhs_str, rhs_str) = match trimmed.split_once(',') {
            // If ',' was found, split was successful, return the two slices.
            Some(slit) => slit,
            // Continue the for loop.
            None => continue,
        };

        // Try parsing both slices into u32.
        let lhs = match lhs_str.parse::<u32>() {
            Ok(lhs) => lhs,
            Err(_) => continue,
        };

        let rhs: u32 = match rhs_str.parse::<u32>() {
            Ok(rhs) => rhs,
            Err(_) => continue,
        };

        result.push((lhs, rhs));
    }

    return result;
}

fn main() {
    let input = read_input();
    let operations = parse_memory(&input);

    // Extra care, in Rust even a for loop can move a collection,
    // the '&' in '&operations' is required
    for op in &operations {
        println!("mul({},{})", op.0, op.1);
    }

    let sum_product = operations.iter().map(|(lhs, rhs)| lhs * rhs).sum::<u32>();
    println!("Sum: {sum_product}");
}
