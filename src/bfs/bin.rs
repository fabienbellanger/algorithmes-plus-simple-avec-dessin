use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

fn list<T: Hash + Eq + Clone + Debug>(graph: &HashMap<T, Vec<T>>, node: T) {
    let mut search_queue: VecDeque<T> = graph
        .get(&node)
        .unwrap_or(&vec![])
        .clone()
        .into_iter()
        .collect();
    let mut marked: HashSet<T> = HashSet::new();
    marked.insert(node.clone());

    println!("{node:?}");

    while !search_queue.is_empty() {
        let s = search_queue.pop_front().unwrap();
        marked.insert(s.clone());

        let next_nodes = graph.get(&s).unwrap_or(&vec![]).clone();
        for n in next_nodes {
            if marked.get(&n).is_none() {
                search_queue.push_back(n.clone());
            }
        }

        println!("{s:?}");
    }
}

fn search<T: Hash + Eq + Clone + Debug>(
    graph: &HashMap<T, Vec<T>>,
    node: T,
    function: fn(wanted: &T, node: &T) -> bool,
    w: &T,
) -> bool {
    let mut search_queue: VecDeque<T> = graph
        .get(&node)
        .unwrap_or(&vec![])
        .clone()
        .into_iter()
        .collect();
    let mut marked: HashSet<T> = HashSet::new();

    while !search_queue.is_empty() {
        let s = search_queue.pop_front().unwrap();

        if marked.get(&s).is_none() {
            if function(w, &s) {
                return true;
            } else {
                for n in graph.get(&s).unwrap_or(&vec![]).clone() {
                    search_queue.push_back(n.clone());
                }
                marked.insert(s.clone());
            }
        }
    }

    false
}

fn tt<T: Hash + Eq + Clone + Debug>(s: &T, q: &T) -> bool {
    s == q
}

fn main() {
    println!("Breadth-first search");

    let mut graph = HashMap::new();
    graph.insert("me", vec!["alice", "bob"]);
    graph.insert("alice", vec!["john"]);
    graph.insert("john", vec!["luke"]);
    graph.insert("bob", vec!["yoyo"]);
    graph.insert("yoyo", vec!["alice"]); // <- bug ?
    list(&graph, "me");

    let result = search(&graph, "me", tt, &"alice");
    assert!(result);

    let result = search(&graph, "me", tt, &"fabien");
    assert!(!result);

    let result = search(&graph, "me", tt, &"me");
    assert!(!result);
}
