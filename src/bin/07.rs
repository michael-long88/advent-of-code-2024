advent_of_code::solution!(7);

pub struct Equation {
    pub result: u64,
    pub values: Vec<u64>,
}

impl Equation {
    pub fn new(result: u64, values: Vec<u64>) -> Self {
        Self { result, values }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn as_str(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Multiply => "*",
            Operator::Concatenate => "||",
        }
    }
}

#[derive(Debug)]
pub struct Solution {
    pub expression: String,
    pub result: u64,
}

pub fn evaluate(numbers: &[u64], operators: &[Operator]) -> u64 {
    let mut result = numbers[0];

    for (i, op) in operators.iter().enumerate() {
        match op {
            Operator::Add => result += numbers[i + 1],
            Operator::Multiply => result *= numbers[i + 1],
            Operator::Concatenate => {
                result = format!("{}{}", result, numbers[i + 1])
                    .parse::<u64>()
                    .unwrap()
            }
        }
    }

    result
}

pub fn build_expression(numbers: &[u64], operators: &[Operator]) -> String {
    let mut expression = numbers[0].to_string();

    for (i, op) in operators.iter().enumerate() {
        expression.push_str(&format!(" {} {}", op.as_str(), numbers[i + 1]));
    }

    expression
}

pub fn generate_expressions(
    numbers: &[u64],
    target: u64,
    pos: usize,
    current_ops: &mut Vec<Operator>,
    part_two: bool,
) -> Option<Solution> {
    // Base case: we've placed all operators
    if pos == numbers.len() - 1 {
        let result = evaluate(numbers, current_ops);
        if result == target {
            return Some(Solution {
                expression: build_expression(numbers, current_ops),
                result,
            });
        }
        return None;
    }

    // Try addition
    current_ops.push(Operator::Add);
    if let Some(solution) = generate_expressions(numbers, target, pos + 1, current_ops, part_two) {
        return Some(solution);
    }
    current_ops.pop();

    // Try multiplication
    current_ops.push(Operator::Multiply);
    if let Some(solution) = generate_expressions(numbers, target, pos + 1, current_ops, part_two) {
        return Some(solution);
    }
    current_ops.pop();

    if part_two {
        // Try concatenation
        current_ops.push(Operator::Concatenate);
        if let Some(solution) =
            generate_expressions(numbers, target, pos + 1, current_ops, part_two)
        {
            return Some(solution);
        }
        current_ops.pop();
    }

    None
}

pub fn find_expression(equation: &Equation, part_two: bool) -> Option<Solution> {
    let mut current_ops = Vec::new();
    generate_expressions(
        &equation.values,
        equation.result,
        0,
        &mut current_ops,
        part_two,
    )
}

pub fn parse(input: &str) -> Vec<Equation> {
    let equations = input
        .split('\n')
        .filter(|equation| !equation.is_empty())
        .map(|equation| {
            let parts: Vec<&str> = equation.split(": ").collect();
            let result = parts[0].parse::<u64>().unwrap();
            let values: Vec<u64> = parts[1]
                .split_ascii_whitespace()
                .map(|value| value.parse::<u64>().unwrap())
                .collect();
            Equation::new(result, values)
        })
        .collect();

    equations
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse(input);

    let total_results = equations
        .iter()
        .map(|equation| find_expression(equation, false))
        .filter(|solution| solution.is_some())
        .map(|solution| solution.unwrap().result)
        .sum::<u64>();

    Some(total_results)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse(input);

    let total_results = equations
        .iter()
        .map(|equation| find_expression(equation, true))
        .filter(|solution| solution.is_some())
        .map(|solution| solution.unwrap().result)
        .sum::<u64>();

    Some(total_results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
