/*
Example:

ParcoursLargeur(Graphe G, Sommet s):
    f = CreerFile();
    f.enfiler(s);
    marquer(s);
    tant que la file est non vide
        s = f.defiler();
        afficher(s);
        pour tout voisin t de s dans G
            si t non marqué
                f.enfiler(t);
                marquer(t);

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
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

fn _person_to_search(name: &str) -> bool {
    name.len() == 5
}

// TODO: Search function as parameter
fn search<T: Hash + Eq + Clone + Debug>(graph: &HashMap<T, Vec<T>>, node: T) -> Option<T> {
    let mut search_queue: Vec<T> = graph.get(&node).unwrap_or(&vec![]).clone();
    let mut searched: HashSet<T> = HashSet::new();
    searched.insert(node.clone());

    println!("==> {node:?}");

    while !search_queue.is_empty() {
        println!("          {:?}", &search_queue);
        println!("          {:?}", &searched);
        let s = search_queue.pop().unwrap();

        println!("==> {s:?}");

        let next_nodes = graph.get(&s).unwrap_or(&vec![]).clone();
        for n in next_nodes {
            if searched.get(&n).is_none() {
                search_queue.push(n.clone());
                searched.insert(n);
            }
        }
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
    search(&graph, "me");
    // assert_eq!(search(HashMap::new(), "you"), None);
}
