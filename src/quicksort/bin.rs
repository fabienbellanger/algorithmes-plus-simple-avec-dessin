use std::fmt::Debug;

fn quicksort<T: Ord + Clone + Debug>(list: &[T]) -> Vec<T> {
    if list.len() < 2 {
        list.to_vec()
    } else {
        let pivot = list[0].clone();

        let less: Vec<T> = (1..list.len())
            .filter_map(|i| match list[i] <= pivot {
                true => Some(list[i].clone()),
                false => None,
            })
            .collect();
        let greater: Vec<T> = (1..list.len())
            .filter_map(|i| match list[i] > pivot {
                true => Some(list[i].clone()),
                false => None,
            })
            .collect();

        [quicksort(&less), vec![pivot], quicksort(&greater)].concat()
    }
}

fn main() {
    assert_eq!(quicksort(&[45, 23, 1, 90, 5]), [1, 5, 23, 45, 90]);
    assert_eq!(
        quicksort(&["david", "jean", "bob", "zoé", "sarah", "alice"]),
        ["alice", "bob", "david", "jean", "sarah", "zoé"]
    );
}
