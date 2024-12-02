use aoc_collections::count_elements;
use aoc_generators::processors::line_processor_trait::LineProcessor;
use aoc_generators::processors::regex_line_processor::{
    RegexLineProcessor, RegexLineProcessorMode,
};

use tracing::{debug, info};

pub struct Day1LineProcessor {
    regex_line_processor: RegexLineProcessor,
}

impl Day1LineProcessor {
    pub fn new() -> Day1LineProcessor {
        let regex_line_processor =
            RegexLineProcessor::new(r"\s+", RegexLineProcessorMode::Split(false));

        Self {
            regex_line_processor,
        }
    }
}

impl LineProcessor for Day1LineProcessor {
    type Item = (i32, i32);
    type ProcessorError = String;

    fn process(&self, line: &str) -> Result<Self::Item, Self::ProcessorError> {
        let regex_processor_result = self.regex_line_processor.process(line);

        if let Ok(tokens) = regex_processor_result {
            if tokens.len() != 2 {
                Err(format!("expected 2 tokens, got {}", tokens.len()))
            } else {
                if let (Ok(x), Ok(y)) = (tokens[0].parse::<i32>(), tokens[1].parse::<i32>()) {
                    Ok((x, y))
                } else {
                    Err(format!(
                        "Failed to parse token {:?} into (i32, i32)",
                        tokens
                    ))
                }
            }
        } else {
            Err(regex_processor_result.err().unwrap())
        }
    }
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<(i32, i32)> {
    aoc_generators::apply_processor_to_input(input, &Day1LineProcessor::new())
        .expect("Failed to process input")
}

#[aoc(day1, part2)]
fn part_2(input: &[(i32, i32)]) -> u32 {
    info!("*** Start Part 2 ***");

    let mut first_values: Vec<i32> = Vec::new();
    let mut second_values: Vec<i32> = Vec::new();

    info!("Number of Processed Lines: {}", input.len());

    extract_values_from_tuples(input, &mut first_values, &mut second_values);

    let element_count = count_elements(&second_values);
    debug!("Element Count: {:?}", element_count);

    debug!("First Values: {:?}", first_values);
    let result: i32 = first_values
        .into_iter()
        .filter(|x| element_count.contains_key(x))
        .map(|x| x * element_count.get(&x).unwrap())
        .sum();
    debug!("Result:       {:?}", result);

    info!("*** End Part 2 ***");
    result as u32
}

#[aoc(day1, part1)]
pub fn part_1(input: &[(i32, i32)]) -> u32 {
    info!("*** Start Part 1 ***");

    let mut first_values: Vec<i32> = Vec::new();
    let mut second_values: Vec<i32> = Vec::new();

    info!("Number of Processed Lines: {}", input.len());
    extract_values_from_tuples(input, &mut first_values, &mut second_values);

    if first_values.len() != second_values.len() {
        panic!("First and Second Value vectors are not the same length");
    }

    info!("Sorting First and Second Value vectors");
    first_values.sort();
    second_values.sort();

    let mut sum = 0;
    for i in 0..first_values.len() {
        let difference = first_values[i] - second_values[i];

        sum += difference.abs();
    }

    info!("Part 1 Result: {}", sum);

    info!("*** End Part 1 ***");
    sum as u32
}

fn extract_values_from_tuples(
    input: &[(i32, i32)],
    first_values: &mut Vec<i32>,
    second_values: &mut Vec<i32>,
) {
    input.iter().for_each(|i| {
        debug!("First: {}, Second: {}", i.0, i.1);
        first_values.push(i.0);
        second_values.push(i.1);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_day_1_file_processor() {
        let test_input_file = "resources/test_input_1";

        let binding = fs::read_to_string(test_input_file).expect("failed to read file");
        let lines = binding.lines().collect::<Vec<&str>>();

        let processor = Day1LineProcessor::new();

        let expected_results = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];

        for i in 0..lines.len() {
            let line_result = processor.process(lines[i]);

            assert!(line_result.is_ok());
            if let Ok(result_tuple) = line_result {
                assert_eq!(result_tuple.0, expected_results[i].0);
                assert_eq!(result_tuple.1, expected_results[i].1);
            }
        }
    }
}
