use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Hash)]
pub struct Buyer {
    start: u32,
}

pub fn get_next_number(number: u32) -> u32 {
    let step_1 = (number ^ (number << 6)) & 0b1111_1111_1111_1111_1111_1111;
    let step_2 = (step_1 ^ (step_1 >> 5)) & 0b1111_1111_1111_1111_1111_1111;
    (step_2 ^ (step_2 << 11)) & 0b1111_1111_1111_1111_1111_1111
}

pub fn get_next_iter(start: u32) -> impl Iterator<Item=u32> {
    let mut current = start;
    std::iter::from_fn(move || {
        current = get_next_number(current);
        Some(current)
    })
}

pub fn get_sales(start: u32) -> HashMap<[i32; 4], i32> {
    let mut numbers = get_next_iter(start).take(2000).collect::<Vec<_>>();
    numbers.insert(0, start);

    let prices = numbers.iter().map(|&n| (n % 10) as i32).collect::<Vec<_>>();
    let changes = prices.windows(2).map(|pair| pair[1] - pair[0]).collect::<Vec<_>>();


    let mut sales = HashMap::new();
    for (index, changes) in changes.windows(4).enumerate() {
        let key = changes.try_into().unwrap();
        let price = prices.get(index + 4).unwrap();
        sales.entry(key).or_insert(*price);
    }

    sales
}

pub fn parse_input(input: &str) -> Vec<Buyer> {
    input.lines().map(|line| Buyer { start: line.parse().unwrap() }).collect()
}

pub fn part_1(buyers: &[Buyer]) -> usize {
    buyers.iter().map(|m| get_next_iter(m.start).nth(1999).unwrap() as usize).sum()
}

pub fn part_2(buyers: &[Buyer]) -> i32 {
    let mut all_sequences = HashSet::new();

    let sales = buyers.iter().map(|b| get_sales(b.start)).collect::<Vec<_>>();

    for sale in &sales {
        all_sequences.extend(sale.keys().copied());
    }

    all_sequences.iter().map(|seq| {
        sales.iter().map(|s| s.get(seq).copied().unwrap_or_default()).sum()
    }).max().unwrap()
}


fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    use std::time::Instant;
    let now = Instant::now();
    println!("Part 1: {}", part_1(&input));
    println!("{:?}", now.elapsed());
    let now = Instant::now();
    println!("Part 2: {}", part_2(&input));
    println!("{:?}", now.elapsed());
}

#[test]
pub fn test_generation() {
    let start = 123;
    let next = get_next_iter(start).take(10).collect::<Vec<_>>();
    assert_eq!(next, vec![15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254])
}

#[test]
pub fn test() {
    let input = r#"1
10
100
2024"#;
    let buyers = parse_input(input);
    assert_eq!(part_1(&buyers), 37327623);
}

#[test]
pub fn test_2() {
    let input = r#"1
2
3
2024"#;
    let buyers = parse_input(input);
    assert_eq!(part_2(&buyers), 23);
}


#[test]
pub fn test_sale_1() {
    let sales = get_sales(123);
    println!("{:?}", sales);

    assert_eq!(sales.get(&[-3, 6, -1, -1]), Some(&4));
    assert_eq!(sales.get(&[6, -1, -1, 0]), Some(&4));
    assert_eq!(sales.get(&[-1, -1, 0, 2]), Some(&6));
}