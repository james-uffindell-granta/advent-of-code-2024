pub fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(|l| {
        l.split_ascii_whitespace().map(|n| {
            n.parse().unwrap()
        }).collect()
    }).collect()
}

pub fn is_valid(report: &[i64]) -> bool {
    report.windows(2).map(|w| w[1] - w[0]).all(|d| d == 1 || d == 2 || d == 3)
    || report.windows(2).map(|w| w[1] - w[0]).all(|d| d == -1 || d == -2 || d == -3)
}

pub fn is_valid_with_tolerance(report: &[i64]) -> bool {
    (0 .. report.len())
        .any(|n| {
            let mut modified_list = report.to_vec();
            modified_list.remove(n);
            is_valid(&modified_list)
        })
}

pub fn part_1(reports: &[Vec<i64>]) -> usize {
    let valid_reports = reports.iter()
    .filter(|r| is_valid(r)).collect::<Vec<_>>();
    valid_reports.len()
}

pub fn part_2(reports: &[Vec<i64>]) -> usize {
    let valid_reports = reports.iter()
    .filter(|r| is_valid_with_tolerance(r)).collect::<Vec<_>>();
    valid_reports.len()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", part_1(&input)); 
    println!("Part 2: {}", part_2(&input));
}

#[test]
pub fn test() {
    let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    let input = parse_input(input);
    assert_eq!(part_1(&input), 2);
    assert_eq!(part_2(&input), 4);
}
