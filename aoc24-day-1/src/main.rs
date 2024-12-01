use aoc_collections::count_elements;
use aoc_file::apply_processor_to_file_lines;
use tracing::{debug, info};
use aoc24_day_1::Day1LineProcessor;

fn main() {


    tracing_subscriber::fmt::init();

    part_1();
    part_2();

}

fn part_2() {
    info!("*** Start Part 2 ***");

    //let part_2_input_file = "aoc24-day-1/resources/test_input_2";
    let part_2_input_file = "aoc24-day-1/resources/input_1";
    info!("Input File: {}", part_2_input_file);

    let part_2_processor = &Day1LineProcessor::new();


    if let Ok(processed_lines) = apply_processor_to_file_lines(part_2_input_file, part_2_processor) {
        let mut first_values: Vec<i32> = Vec::new();
        let mut second_values: Vec<i32> = Vec::new();

        info!("Number of Processed Lines: {}", processed_lines.len());
        processed_lines.iter().for_each(|line| {
            debug!("First: {}, Second: {}", line.0, line.1);
            first_values.push(line.0);
            second_values.push(line.1);
        });

        let element_count = count_elements(&second_values);
        debug!("Element Count: {:?}", element_count);

        debug!("First Values: {:?}", first_values);
        let result : i32 = first_values
            .into_iter()
            .filter(|x| element_count.contains_key(x))
            .map(|x| x * element_count.get(&x).unwrap())
            .sum();
        debug!("Result:       {:?}", result);
    } else {
        panic!("Failed to process file");
    }

    info!("*** End Part 2 ***");
}

fn part_1() {
    info!("*** Start Part 1 ***");

    //let input_file = "aoc24-day-1/resources/test_input_1";
    let input_file = "aoc24-day-1/resources/input_1";
    info!("Input File: {}", input_file);

    let part_1_processor = &Day1LineProcessor::new();


    if let Ok(processed_lines) = apply_processor_to_file_lines(input_file, part_1_processor) {
        let mut first_values: Vec<i32> = Vec::new();
        let mut second_values: Vec<i32> = Vec::new();

        info!("Number of Processed Lines: {}", processed_lines.len());
        processed_lines.iter().for_each(|line| {
            debug!("First: {}, Second: {}", line.0, line.1);
            first_values.push(line.0);
            second_values.push(line.1);
        });

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


    }
    info!("*** End Part 1 ***");
}

