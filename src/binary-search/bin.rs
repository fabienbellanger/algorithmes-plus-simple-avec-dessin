fn binary_search<T: PartialEq + PartialOrd + Ord + Copy>(list: &[T], item: T) -> Option<usize> {
    if list.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut hight = list.len() - 1;

    while low <= hight {
        let middle = (low + hight) / 2;
        let guess = list[middle];

        if guess == item {
            return Some(middle);
        }
        else if guess > item {
            hight = middle.saturating_sub(1);
        } else {
            low = middle.saturating_add(1);
        }
    }

    None
}

fn main() {
    let list = vec![12,56,102,568,700];
    assert_eq!(binary_search(&list, 102), Some(2));
    assert_eq!(binary_search(&list, 12), Some(0));
    assert_eq!(binary_search(&list, 700), Some(4));
    assert_eq!(binary_search(&list, 103), None);

    let mut list = vec![700,568,102,56,12];
    list.sort_unstable();
    assert_eq!(binary_search(&list, 102), Some(2));
    assert_eq!(binary_search(&list, 12), Some(0));
    assert_eq!(binary_search(&list, 700), Some(4));
    assert_eq!(binary_search(&list, 103), None);
}
