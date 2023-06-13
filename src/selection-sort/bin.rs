fn selection_sort<T: PartialOrd + Clone>(mut list: Vec<T>) -> Result<Vec<T>, ()> {
    let mut new_list = vec![];

    for _ in 0..list.len() {
        let mut min_idx = 0;
        let mut smallest = list.get(0).ok_or(())?.clone();

        for (i, _) in list.iter().enumerate().skip(1) {
            if list[i] < smallest {
                smallest = list.get(i).ok_or(())?.clone();
                min_idx = i;
            }
        }

        new_list.push(smallest);
        list.remove(min_idx);
    }

    Ok(new_list)
}

fn selection_sort_2<T: Ord>(list: &mut [T]) {
    for i in 0..list.len() {
        let min_idx = list[i..]
            .iter()
            .enumerate()
            .min_by_key(|&(_, v)| v)
            .map(|(i, _)| i)
            .unwrap_or(0);

        list.swap(i, min_idx + i);
    }
}

fn main() {
    let list = vec![42, 2, 96, 74, 78, 78, 1, 9, 18];
    let expected = vec![1, 2, 9, 18, 42, 74, 78, 78, 96];
    assert_eq!(selection_sort(list).unwrap(), expected);

    let list = vec!["dog", "cat", "John", "car", "zorro", "at", "lime"];
    let expected = vec!["John", "at", "car", "cat", "dog", "lime", "zorro"];
    assert_eq!(selection_sort(list).unwrap(), expected);

    let mut list = vec![42, 2, 96, 74, 78, 78, 1, 9, 18];
    let expected = vec![1, 2, 9, 18, 42, 74, 78, 78, 96];
    selection_sort_2(&mut list);
    assert_eq!(list, expected);

    let mut list = vec!["dog", "cat", "John", "car", "zorro", "at", "lime"];
    let expected = vec!["John", "at", "car", "cat", "dog", "lime", "zorro"];
    selection_sort_2(&mut list);
    assert_eq!(list, expected);
}
