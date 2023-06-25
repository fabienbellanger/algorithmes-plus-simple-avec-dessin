use std::{collections::HashMap, fmt::Debug, hash::Hash};

type Graph<T: Hash + Debug> = HashMap<T, HashMap<T, usize>>;
type GraphCost<T: Hash + Debug> = HashMap<T, usize>;
type GraphParent<T: Hash + Debug> = HashMap<T, T>;

fn main() {
    let mut graph: Graph<&str> = HashMap::new();

    let mut graph_start: HashMap<&str, usize> = HashMap::new();
    graph_start.insert("a", 6);
    graph_start.insert("b", 2);
    graph.insert("start", graph_start);

    /*graph.insert("a", HashMap::new());
    graph["a"].insert("end", 1);

    graph.insert("b", HashMap::new());
    graph["b"].insert("a", 3);
    graph["b"].insert("end", 5);

    graph.insert("end", HashMap::new());*/

    dbg!(&graph);

    println!("Dijkstra algorithm");
}
