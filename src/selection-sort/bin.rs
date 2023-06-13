fn selection_sort<T: PartialOrd + Clone>(mut list: Vec<T>) -> Vec<T> {
    let mut new_list = vec![];

    for _ in 0..list.len() {
        let mut idx = 0;
        let mut smallest = list[0].clone();

        for (i, _) in list.iter().enumerate().skip(1) {
            if list[i] < smallest {
                smallest = list[i].clone();
                idx = i;
            }
        }

        new_list.push(smallest);
        list.remove(idx);
    }

    new_list
}

fn main() {
    let list = vec![42, 2, 96, 74, 78, 78, 1, 9, 18];
    let expected = vec![1, 2, 9, 18, 42, 74, 78, 78, 96];
    assert_eq!(selection_sort(list), expected);

    let list = vec!["dog", "cat", "John", "car", "zorro", "at", "lime"];
    let expected = vec!["John", "at", "car", "cat", "dog", "lime", "zorro"];
    assert_eq!(selection_sort(list), expected);
}
