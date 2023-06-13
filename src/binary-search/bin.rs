use std::cmp::Ordering;

fn binary_search<T: Ord>(list: &[T], item: &T) -> Option<usize> {
    if !list.is_empty() {
        let mut low = 0;
        let mut hight = list.len() - 1;

        while low <= hight {
            let middle = (low + hight) / 2;

            match list[middle].cmp(item) {
                Ordering::Equal => return Some(middle),
                Ordering::Greater => hight = middle.checked_sub(1)?,
                Ordering::Less => low = middle.checked_add(1)?,
            }
        }
    }

    None
}

fn main() {
    let list = vec![12, 56, 102, 568, 700];
    assert_eq!(binary_search(&list, &102), Some(2));
    assert_eq!(binary_search(&list, &12), Some(0));
    assert_eq!(binary_search(&list, &700), Some(4));
    assert_eq!(binary_search(&list, &103), None);

    let mut list = vec![700, 568, 102, 56, 12];
    list.sort_unstable();
    assert_eq!(binary_search(&list, &102), Some(2));
    assert_eq!(binary_search(&list, &12), Some(0));
    assert_eq!(binary_search(&list, &700), Some(4));
    assert_eq!(binary_search(&list, &103), None);

    let list = vec!["a", "b", "c", "d"];
    assert_eq!(binary_search(&list, &"a"), Some(0));
    assert_eq!(binary_search(&list, &"e"), None);
}
