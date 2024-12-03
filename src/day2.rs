#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|x| x.split(' ').map(|t| t.parse().unwrap()).collect()).collect()
}

fn is_safe(report: &[i32]) -> bool {
    (report.windows(2).all(|w| w[0] < w[1]) || report.windows(2).all(|w| w[1] < w[0]))
    && report.windows(2).all(|w| (w[0] - w[1]).abs() <= 3) && report.windows(2).all(|w| (w[0] - w[1]).abs() >= 1)
}

#[aoc(day2, part1)]
fn part_1(input: &Vec<Vec<i32>>) -> usize {
    input.iter().map(|row| is_safe(row)).filter(|&b| b).count()
}

fn and(a: &[bool], b: &[bool]) -> Vec<bool> {
    a.iter().zip(b).map(|(x, y)| x & y).collect()
}

#[aoc(day2, part2)]
fn part2(input: &Vec<Vec<i32>>) -> usize {
    fn is_safe_damp(report: &[i32]) -> bool {
        let increasing = report
            .windows(2)
            .map(|w| w[0] < w[1])
            .collect::<Vec<bool>>();

        let decreasing = report
            .windows(2)
            .map(|w| w[0] > w[1])
            .collect::<Vec<bool>>();

        let diff = report
            .windows(2)
            .map(|w| (w[0] - w[1]).abs() <= 3)
            .collect::<Vec<bool>>();

        let order_bools = if increasing.iter().filter(|&b| *b).count()
            > decreasing.iter().filter(|&b| *b).count()
        {
            increasing
        } else {
            decreasing
        };
        let my_bools = and(&order_bools, &diff);
        let pos = my_bools.iter().position(|&b| !b);
        match pos {
            Some(idx) => {
                fn remove_then_check(report: &[i32], idx: usize) -> bool {
                    let mut x = report.to_vec();
                    x.remove(idx);
                    is_safe(&x)
                }
                remove_then_check(report, idx) || remove_then_check(report, idx + 1)
            }
            None => true,
        }
    }

    let result = input
        .iter()
        .map(|row| is_safe_damp(row))
        .filter(|&b| b)
        .count();
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_generator() {
        let input = "1 2\n7 8 9";

        let output = input_generator(&input);

        assert_eq!(output.len(), 2);
        assert_eq!(output[0].len(), 2);
        assert_eq!(output[0][0], 1);
        assert_eq!(output[0][1], 2);
        assert_eq!(output[1].len(), 3);
        assert_eq!(output[1][0], 7);
        assert_eq!(output[1][1], 8);
        assert_eq!(output[1][2], 9);
    }

    #[test]
    fn test_is_safe() {

        let input = vec![
            vec![7, 6, 4, 2, 1],
            vec![1,2,7,8,9],
            vec![9,7,6,2,1],
            vec![1,3,2,4,5],
            vec![8,6,4,4,1],
            vec![1,3,6,7,9]
        ];

        assert!(is_safe(&input[0]));
        assert!(!is_safe(&input[1]));
        assert!(!is_safe(&input[2]));
        assert!(!is_safe(&input[3]));
        assert!(!is_safe(&input[4]));
        assert!(is_safe(&input[5]));
    }
}
