use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, Eq, Clone)]
struct Node {
    symbol: char,
    gt: Vec<char>,
    lt: Vec<char>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.gt.contains(&other.symbol) {
            return Ordering::Greater;
        } else if self.lt.contains(&other.symbol) {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

fn join_same_node(symbol: char, nodes: &Vec<Node>) -> Node {
    let same_nodes: Vec<&Node> = nodes.iter().filter(|node| node.symbol == symbol).collect();
    let total_gt: HashSet<char> = same_nodes
        .iter()
        .flat_map(|&node| node.gt.iter().clone().map(|&n| n).collect::<Vec<char>>())
        .collect();
    let total_lt: HashSet<char> = same_nodes
        .iter()
        .flat_map(|&node| node.lt.iter().clone().map(|&n| n).collect::<Vec<char>>())
        .collect();

    let sup_total_gt: HashSet<char> = total_gt
        .iter()
        .flat_map(|&x| {
            nodes
                .iter()
                .filter(|node| node.symbol == x)
                .flat_map(|node| node.gt.iter().clone().map(|&n| n))
                .collect::<HashSet<char>>()
        })
        .collect();

    let sup_total_lt: HashSet<char> = total_lt
        .iter()
        .flat_map(|&x| {
            nodes
                .iter()
                .filter(|node| node.symbol == x)
                .flat_map(|node| node.lt.iter().clone().map(|&n| n))
                .collect::<HashSet<char>>()
        })
        .collect();

    Node {
        symbol,
        gt: [
            total_gt.into_iter().collect::<Vec<char>>(),
            sup_total_gt.into_iter().collect::<Vec<char>>(),
        ]
        .concat(),
        lt: [
            total_lt.into_iter().collect::<Vec<char>>(),
            sup_total_lt.into_iter().collect::<Vec<char>>(),
        ]
        .concat(),
    }
}

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    // triplets preserve order
    let mut nodes: Vec<Node> = vec![];
    let mut unique: HashSet<char> = HashSet::new();
    for triplet in triplets.iter() {
        let left = triplet[0];
        let middle = triplet[1];
        let right = triplet[2];

        nodes.push(Node {
            symbol: left,
            gt: vec![middle, right],
            lt: vec![],
        });

        nodes.push(Node {
            symbol: middle,
            gt: vec![right],
            lt: vec![left],
        });

        nodes.push(Node {
            symbol: right,
            gt: vec![],
            lt: vec![left, middle],
        });
        unique.insert(left);
        unique.insert(middle);
        unique.insert(right);
    }

    let mut joined_nodes: Vec<Node> = vec![];
    let mut result = String::new();

    for &character in unique.iter() {
        let joined_node = join_same_node(character, &nodes);
        joined_nodes.push(joined_node)
    }

    loop {
        let copy = joined_nodes.clone();
        joined_nodes.clear();

        for &character in unique.iter() {
            let joined_node = join_same_node(character, &copy);
            joined_nodes.push(joined_node)
        }

        joined_nodes.sort();
        joined_nodes.reverse();

        let next_result = joined_nodes
            .iter()
            .map(|node| node.symbol)
            .map(|symbol| symbol.to_string())
            .collect::<Vec<String>>()
            .join("");

        if result == next_result {
            break;
        }
        result = next_result;
    }

    return result;
}

fn main() {
    let recovered = recover_secret(vec![
        ['t', 'u', 'p'],
        ['w', 'h', 'i'],
        ['t', 's', 'u'],
        ['a', 't', 's'],
        ['h', 'a', 'p'],
        ['t', 'i', 's'],
        ['w', 'h', 's'],
    ]);
    println!("Recovered: {}", recovered);
}

#[test]
fn example_test() {
    assert_eq!(
        recover_secret(vec![
            ['t', 'u', 'p'],
            ['w', 'h', 'i'],
            ['t', 's', 'u'],
            ['a', 't', 's'],
            ['h', 'a', 'p'],
            ['t', 'i', 's'],
            ['w', 'h', 's']
        ]),
        "whatisup"
    );
}
