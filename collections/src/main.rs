mod collections;

use collections::linked_list::LinkedList;

fn main() {
    let mut list: LinkedList<f32> = LinkedList::new();
    let mut line: String = String::new();

    std::io::stdin().read_line(&mut line).unwrap();
    line = line.trim().to_string();

    let lines = &line
        .split(' ')
        .map(|s| s.to_string())
        .filter(|f| !f.is_empty())
        .collect::<Vec<String>>();

    let result: Option<f32> = solve_upn_calculator(&mut list, lines);

    if let Some(res) = result {
        println!("The result is {res}");
    } else {
        println!("ERROR: Invalid UPN-Notation in => {:?}", line);
    }
}

/// Upn-calculator
///
/// it takes a list of arguments like:
///
/// 6 5 2 3 + 8 * + 3 + *
///
/// with that it calculates
///
/// => 6 5 5 8 * + 3 + * 
/// => 6 5 40 + 3 + *
/// => 6 45 3 + *
/// => 6 48 *
/// => 288
///
/// and returns the value calulated
fn solve_upn_calculator(list: &mut LinkedList<f32>, input: &Vec<String>) -> Option<f32> {
    for i in 0..input.len() {
        let op = input[i].chars().next().unwrap();
        if op == '+' || op == '-' || op == '*' || op == '/' {
            let mut values: [f32; 2] = [0f32; 2];

            for i in 0..2usize {
                if let Some(value) = list.pop() {
                    values[i] = value;
                } else {
                    return None;
                }
            }

            match &input[i][0..=0] {
                "+" => list.push(values[0] + values[1]),
                "-" => list.push(values[0] - values[1]),
                "*" => list.push(values[0] * values[1]),
                "/" => list.push(values[0] / values[1]),
                _ => {
                    return None;
                }
            }
        } else {
            let parse = input[i].parse::<f32>();

            match parse {
                Ok(value) => list.push(value),
                Err(_err) => return None
            }
        }
    }

    let result: Option<f32> = list.pop();

    if list.is_empty() {
        result
    } else {
        None
    }
}
