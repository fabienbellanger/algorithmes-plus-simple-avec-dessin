use rand::{thread_rng, Rng};

fn quicksort<T: Ord + Clone + Copy>(list: &[T]) -> Vec<T> {
    if list.len() < 2 {
        list.to_vec()
    } else {
        let pivot = list[0];
        let (less, greater): (Vec<T>, Vec<T>) = list[1..].iter().partition(|v| **v <= pivot);

        [quicksort(&less), vec![pivot], quicksort(&greater)].concat()
    }
}

fn choose_pivot(low: usize, hight: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(low..=hight)
}

fn partition<T: Ord + Clone + Copy>(list: &mut [T], low: usize, hight: usize) -> usize {
    let pivot = choose_pivot(low, hight);
    list.swap(hight, pivot);

    let mut j = low;

    for i in low..hight {
        if list[i] <= list[hight] {
            list.swap(i, j);
            j += 1;
        }
    }

    list.swap(hight, j);

    j
}

fn quicksort2<T: Ord + Clone + Copy>(list: &mut [T], low: usize, hight: usize) {
    if low < hight {
        let pivot = partition(list, low, hight);

        quicksort2(list, low, pivot.saturating_sub(1));
        quicksort2(list, pivot.saturating_add(1), hight);
    }
}

fn main() {
    assert_eq!(quicksort(&vec![45, 23, 1, 90, 5]), [1, 5, 23, 45, 90]);
    assert_eq!(
        quicksort(&["david", "jean", "bob", "zoé", "sarah", "alice"]),
        ["alice", "bob", "david", "jean", "sarah", "zoé"]
    );

    let mut list = vec![45, 23, 1, 90, 5];
    quicksort2(&mut list, 0, 4);
    assert_eq!(list, [1, 5, 23, 45, 90]);
}
