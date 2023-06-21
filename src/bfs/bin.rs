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

fn main() {
    println!("Breadth-first search");
}
