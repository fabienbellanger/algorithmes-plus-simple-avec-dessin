use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

type Graph<T> = HashMap<T, HashMap<T, usize>>;
type GraphCost<T> = HashMap<T, usize>;
type GraphParent<T> = HashMap<T, T>;

fn find_lowest_cost_node<T: Hash + Eq + Debug + Clone>(
    costs: &GraphCost<T>,
    processed: &mut HashSet<T>,
) -> Option<T> {
    let mut lowest_cost = usize::MAX;
    let mut lowest_cost_node = None;

    for (node, cost) in costs {
        let exists = processed.contains(node);
        if *cost < lowest_cost && !exists {
            lowest_cost = *cost;
            lowest_cost_node = Some(node.clone());
        }
    }

    lowest_cost_node
}

fn find_path<T: Hash + Debug + Clone + Copy + Eq>(
    graph: &Graph<T>,
    costs: &mut GraphCost<T>,
    parents: &mut GraphParent<T>,
) {
    let mut processed: HashSet<T> = HashSet::new();

    let mut best_node = find_lowest_cost_node(costs, &mut processed);
    while let Some(node) = best_node {
        let cost = *costs.get(&node).unwrap_or(&usize::MAX);
        let neighbors = graph.get(&node).unwrap();
        for n in neighbors.keys() {
            let new_cost = cost.saturating_add(*neighbors.get(n).unwrap_or(&0));

            if costs[n] > new_cost {
                let e = costs.entry(*n).or_insert(new_cost);
                *e = new_cost;
                let e = parents.entry(*n).or_insert(node);
                *e = node;
            }
        }

        processed.insert(node);
        best_node = find_lowest_cost_node(costs, &mut processed);
    }
}

fn get_best_path<T: Hash + Eq + Clone + Debug>(
    parents: &GraphParent<T>,
    start: &T,
    end: &T,
) -> Vec<T> {
    let mut result: Vec<T> = Vec::with_capacity(parents.len() + 1);
    let mut current = end.clone();

    for _node in parents {
        let next = parents.get(&current);
        if let Some(next) = next {
            result.push(current.clone());
            current = next.clone();

            if next == start {
                result.push(current.clone());
            }
        }
    }

    result.reverse();

    result
}

fn main() {
    println!("Dijkstra algorithm\n");

    // Graph
    let mut graph: Graph<&str> = HashMap::new();
    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("a", 6);
    map.insert("b", 2);
    graph.insert("start", map);
    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("end", 1);
    graph.insert("a", map);
    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("a", 3);
    map.insert("end", 5);
    graph.insert("b", map);
    graph.insert("end", HashMap::new());

    // Graph cost
    let mut graph_costs: GraphCost<&str> = HashMap::new();
    graph_costs.insert("a", 6);
    graph_costs.insert("b", 2);
    graph_costs.insert("end", usize::MAX);

    // Graph parents
    let mut graph_parents: GraphParent<&str> = HashMap::new();
    graph_parents.insert("a", "start");
    graph_parents.insert("b", "start");

    find_path(&graph, &mut graph_costs, &mut graph_parents);
    println!(
        "Best path: {:?}",
        get_best_path(&graph_parents, &"start", &"end")
    );
    println!("Best cost: {}", graph_costs.get(&"end").unwrap_or(&0));
}
