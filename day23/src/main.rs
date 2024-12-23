use std::collections::{HashMap, HashSet};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Graph {
    vertices: HashSet<&'static str>,
    edges: HashSet<(&'static str, &'static str)>,
    neighbours: HashMap<&'static str, HashSet<&'static str>>,
}

pub fn find_cliques(graph: &Graph) -> Vec<(&'static str, &'static str, &'static str)> {
    let mut cliques = Vec::new();
    for c in graph.vertices.iter().combinations(3) {
        let mut nodes = c.clone();
        nodes.sort();
        if graph.edges.contains(&(nodes[0], nodes[1])) && graph.edges.contains(&(nodes[0], nodes[2])) && graph.edges.contains(&(nodes[1], nodes[2])) {
            cliques.push((*nodes[0], *nodes[1], *nodes[2]));
        }

    }

    cliques
}

pub fn find_maximal_cliques(graph: &Graph) -> Vec<Vec<&'static str>> {
    let mut found_cliques = bron_kerbosch(graph, HashSet::new(), graph.vertices.clone(), HashSet::new());

    found_cliques.sort_by_key(|s| s.len());
    found_cliques.iter().map(|s| {
        let mut s = s.iter().copied().collect::<Vec<_>>();
        s.sort();
        s
    }).collect()

}

pub fn bron_kerbosch(graph: &Graph, r: HashSet<&'static str>, p: HashSet<&'static str>, x: HashSet<&'static str>) -> Vec<HashSet<&'static str>> {
    if p.is_empty() && x.is_empty() {
        return vec![r.clone()];
    }

    let mut cliques = Vec::new();
    let mut p2 = p.clone();
    let mut x2 = x.clone();
    for v in p {
        let mut r2 = r.clone();
        r2.insert(v);
        let restricted_p = p2.intersection(graph.neighbours.get(v).unwrap()).copied().collect::<HashSet<_>>();
        let restricted_x = x2.intersection(graph.neighbours.get(v).unwrap()).copied().collect::<HashSet<_>>();
        let res = bron_kerbosch(graph, r2, restricted_p, restricted_x);
        cliques.extend(res);
        p2.remove(v);
        x2.insert(v);
    }

    cliques
}

pub fn part_1(graph: &Graph) -> usize {
    let mut answer = 0;
    let cliques = find_cliques(graph);
    for (c1, c2, c3) in cliques {
        if c1.starts_with('t') || c2.starts_with('t') || c3.starts_with('t') {
            answer += 1;
        }
    }

    answer
}

pub fn part_2(graph: &Graph) -> String {
    let cliques = find_maximal_cliques(graph);
    let best = cliques.last().unwrap();
    best.join(",")
}

pub fn parse_input(input: &'static str) -> Graph {
    let mut vertices = HashSet::new();
    let mut edges = HashSet::new();
    let mut neighbours = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once("-").unwrap();
        vertices.insert(left);
        vertices.insert(right);
        neighbours.entry(left).or_insert(HashSet::new()).insert(right);
        neighbours.entry(right).or_insert(HashSet::new()).insert(left);
        if left < right {
            edges.insert((left, right));
        } else {
            edges.insert((right, left));
        }
    }

    Graph { vertices, edges, neighbours }
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
pub fn test() {
    let input = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

    let graph = parse_input(input);
    let cliques = find_cliques(&graph);
    assert_eq!(cliques.len(), 12);
    assert_eq!(part_1(&graph), 7);
    // assert_eq!(next, vec![15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254])
}
