/*
def search(name):
    search_queue = deque()
    search_queue += graph[name]
    searched = []

    while search_queue:
        person = search_queue.popleft()

        if not person in searched:
            if person_is_seller(person):
                print person + “ is a mango seller!”
                return True
            else:
                search_queue += graph[person] searched.append(person)

    return False
*/

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

fn list<T: Hash + Eq + Clone + Debug>(graph: &HashMap<T, Vec<T>>, node: T) -> Option<T> {
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

    None
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
}
