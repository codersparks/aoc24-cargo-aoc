use aoc_file::processors::regex_line_processor::{RegexLineProcessor, RegexLineProcessorMode};
use aoc_file::processors::line_processor_trait::LineProcessor;

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
                    Err(format!("Failed to parse token {:?} into (i32, i32)", tokens))
                }
            }
        } else {
            Err(regex_processor_result.err().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use aoc_file::processors::line_processor_trait::LineProcessor;

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
