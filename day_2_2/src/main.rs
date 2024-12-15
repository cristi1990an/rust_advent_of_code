fn read_input() -> Vec<Vec<u32>> {
    use std::fs::File;
    use std::io::{self, BufRead};

    let mut result: Vec<Vec<u32>> = vec![];

    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    for line_result in reader.lines() {
        result.push(vec![]);

        let line = line_result.unwrap();

        // Split a string into slices delimited by whitespaces
        for word in line.split_whitespace() {
            let last_vec = result.last_mut().unwrap();
            last_vec.push(word.parse::<u32>().unwrap());
        }
    }

    return result;
}

fn is_safe_report(report: &Vec<u32>) -> bool {

    if !report.is_sorted() && !report.is_sorted_by(|lhs, rhs| lhs > rhs) {
        return false;
    }
    let slider = report.windows(2);

    let mut diffs = slider.map(|window| window[0].abs_diff(window[1]));

    if diffs.any(|diff| diff < 1 || diff > 3) {
        return false;
    }

    return true;
}

fn get_alternatives(report: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut result : Vec<Vec<u32>> = vec![];

    // Copies are explicit in Rust.
    let mut cpy = report.clone();

    for i in 0..report.len()
    {
        cpy.remove(i);
        result.push(cpy);
        cpy = report.clone();
    }

    return result;
}

fn is_safe(report: &Vec<u32>) -> bool
{
    if is_safe_report(report)
    {
        println!("{:?}: Safe", report);
        return true;
    }
    else
    {
        println!("{:?}: Unsafe", report);
    }

    for alternative in get_alternatives(report)
    {
        if is_safe_report(&alternative)
        {
            println!("{:?}: Safe alternative", alternative);
            return true;
        }
    }

    println!("No safe alternatives");
    return false;
}

fn main() {
    let input = read_input();

    let safe_count = input
        .iter()
        .filter(|report: &&Vec<u32>| is_safe(*report))
        .count();

    println!("Safe count: {safe_count}");
}
