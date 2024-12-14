use std::cmp::Ordering;
use std::collections::HashMap;
use aoc_collections::search::{find_middle_element, FindMiddleElementMode};

struct SafetyPageOrderer {
    page_mappings: HashMap<u32, Vec<u32>>
}

impl SafetyPageOrderer {
    pub fn new(page_mappings:HashMap<u32, Vec<u32>>) -> Self {
        Self {
            page_mappings
        }
    }
    
    pub fn cmp(&self, a: u32, b: u32) -> Ordering {
        if let Some(a_after) = self.page_mappings.get(&a) {
            if a_after.contains(&b) {
                return Ordering::Less;
            } 
        }
        if let Some(b_after) = self.page_mappings.get(&b) {
            if b_after.contains(&a) {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }

    pub fn is_ordered(&self, pages: &Vec<u32>) -> bool {
        
        for i in 0..pages.len() {
            for j in i+1..pages.len() {
                if self.cmp(pages[i], pages[j]) == Ordering::Greater {
                    return false;
                }
            }
        }
        true
    }
}

#[aoc_generator(day5)]
fn input_generator_day5(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let (page_order, page_sequence) = split_input_into_sections(input);

    let order_map = generate_order_hashmap(page_order);

    let processed_page_sequence = page_sequence.iter().map(|line| { line.split(",").map(|s| { s.parse::<u32>().unwrap() }).collect() }).collect();
    (order_map, processed_page_sequence)
}

fn generate_order_hashmap(page_order: Vec<&str>) -> HashMap<u32, Vec<u32>> {
    let mut order_map: HashMap<u32, Vec<u32>> = HashMap::new();

    // Our first section lists the ordering so we process that into a map to allow lookup
    page_order.iter().map(|order: &&str| {
        let s = order.split_once("|");
        if let Some((a, b)) = s {
            let a = a.parse::<u32>().unwrap();
            let b = b.parse::<u32>().unwrap();
            (a, b)
        } else {
            panic!("Cannot split order string on '|'");
        }
    }).for_each(|order| {
        let this = order.0;
        // this has to be before the 1 value
        let after = order.1;

        if let Some(order_vec) = order_map.get_mut(&this) {
            order_vec.push(after);
        } else {
            let order_vec = vec![after];
            order_map.insert(this, order_vec);
        }
    });

    order_map
}


fn split_input_into_sections(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut section_1 = vec![];
    let mut section_2 = vec![];

    let mut workind = &mut section_1;
    for line in input.lines() {
        if line.is_empty() {
            workind = &mut section_2;
        } else {
            workind.push(line);
        }
    }
    (section_1, section_2)
}

#[aoc(day5, part1)]
fn part1(input: &(HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)) -> u32 {
    let (order_map, page_sequence) = input;
    let orderer = SafetyPageOrderer::new(order_map.clone());

    page_sequence.iter().filter(|page| { orderer.is_ordered(page) }).map(|s| find_middle_element(s, FindMiddleElementMode::Error).unwrap()).sum()

}

#[aoc(day5, part2)]
fn part2(input: &(HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)) -> u32 {
    let (order_map, page_sequence) = input;
    let orderer = SafetyPageOrderer::new(order_map.clone());

    page_sequence.iter().filter(|page| { !orderer.is_ordered(page) }).map(|page| {
        println!("Unordered page: {:?}", page);
        let mut sorted_page = page.clone();
        sorted_page.sort_by(|a, b| { orderer.cmp(*a, *b)});
        println!("Sorted page: {:?}", sorted_page);
        let middle_element = find_middle_element(&sorted_page, FindMiddleElementMode::Error).unwrap().clone();
        middle_element
    }).sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    #[test]
    fn test_part2() {

        let input = read_to_string("test_input/2024/day5.txt").unwrap();
        let (order_map, page_sequence) = input_generator_day5(&input);

        let result = part2(&(order_map, page_sequence));

        assert_eq!(result, 123)
    }

    #[test]
    fn test_part1() {

        let input = read_to_string("test_input/2024/day5.txt").unwrap();
        let (order_map, page_sequence) = input_generator_day5(&input);

        let result = part1(&(order_map, page_sequence));

        assert_eq!(result, 143)
    }
    #[test]
    fn test_safety_page_orderer_is_ordered() {
        let mut page_mappings = HashMap::new();
        page_mappings.insert(1, vec![]);
        page_mappings.insert(2, vec![1, 3]);

        let orderer = SafetyPageOrderer::new(page_mappings);

        let ordered_pages_1 = vec![2, 1, 3];
        assert!(orderer.is_ordered(&ordered_pages_1));

        let ordered_pages_2 = vec![1, 3];
        assert!(orderer.is_ordered(&ordered_pages_2)); // There's no explicit order defined for 1, 3, so this should be true

        let unordered_pages = vec![3, 2, 1];
        assert!(!orderer.is_ordered(&unordered_pages));
    }
    #[test]
    fn test_safety_page_orderer_cmp() {
        let mut page_mappings = HashMap::new();
        page_mappings.insert(1, vec![]);
        page_mappings.insert(2, vec![1, 3]);

        let orderer = SafetyPageOrderer::new(page_mappings);

        assert_eq!(orderer.cmp(1, 2), Ordering::Greater);
        assert_eq!(orderer.cmp(2, 1), Ordering::Less);
        assert_eq!(orderer.cmp(1, 3), Ordering::Equal);
        assert_eq!(orderer.cmp(3, 1), Ordering::Equal); // no specific order for these
        assert_eq!(orderer.cmp(2, 3), Ordering::Less);
        assert_eq!(orderer.cmp(3, 2), Ordering::Greater);
    }
    #[test]
    fn test_input_generator_day5() {
        let input = "1|2\n2|3\n\n1,2\n3,4";
        let (order_map, processed_page_sequence) = input_generator_day5(input);

        let mut expected_order_map = HashMap::new();
        expected_order_map.insert(1, vec![2]);
        expected_order_map.insert(2, vec![3]);

        let expected_page_sequence = vec![vec![1, 2], vec![3, 4]];

        assert_eq!(order_map, expected_order_map);
        assert_eq!(processed_page_sequence, expected_page_sequence);
    }
    #[test]
    fn test_generate_order_hashmap() {
        let page_order = vec!["1|2", "2|3", "1|3", "3|4"];
        let expected_map = {
            let mut map = HashMap::new();
            map.insert(1, vec![2]);
            map.insert(2, vec![3]);
            map.insert(3, vec![4]);
            map.get_mut(&1).unwrap().push(3);
            map
        };

        let result_map = generate_order_hashmap(page_order);

        assert_eq!(result_map, expected_map);
    }

    #[test]
    #[should_panic(expected = "Cannot split order string on '|'")]
    fn test_generate_order_hashmap_panic() {
        let page_order = vec!["1|2", "invalid", "3|4"];
        generate_order_hashmap(page_order);
    }
    #[test]
    fn test_split_input_into_sections() {
        let input = "a\nb\n\nc\nd\n";
        let (first, second) = split_input_into_sections(input);
        assert_eq!(first, vec!["a", "b"]);
        assert_eq!(second, vec!["c", "d"]);
    }
}