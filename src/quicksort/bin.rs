use std::fmt::Debug;

fn quicksort<T: Ord + Clone + Copy + Debug>(list: &[T]) -> Vec<T> {
    if list.len() < 2 {
        list.to_vec()
    } else {
        let pivot = list[0];
        let (less, greater): (Vec<T>, Vec<T>) = list[1..].iter().partition(|v| **v <= pivot);

        [quicksort(&less), vec![pivot], quicksort(&greater)].concat()
    }
}

/*
partitionner(tableau T, entier premier, entier dernier, entier pivot)
    échanger T[pivot] et T[dernier]  // échange le pivot avec le dernier du tableau , le pivot devient le dernier du tableau
    j := premier
    pour i de premier à dernier - 1 // la boucle se termine quand i = (dernier élément du tableau).
        si T[i] <= T[dernier] alors
            échanger T[i] et T[j]
            j := j + 1
    échanger T[dernier] et T[j]
    renvoyer j

tri_rapide(tableau T, entier premier, entier dernier)
        si premier < dernier alors
            pivot := choix_pivot(T, premier, dernier)
            pivot := partitionner(T, premier, dernier, pivot)
            tri_rapide(T, premier, pivot-1)
            tri_rapide(T, pivot+1, dernier)
*/

fn main() {
    assert_eq!(quicksort(&vec![45, 23, 1, 90, 5]), [1, 5, 23, 45, 90]);
    assert_eq!(
        quicksort(&["david", "jean", "bob", "zoé", "sarah", "alice"]),
        ["alice", "bob", "david", "jean", "sarah", "zoé"]
    );
}
