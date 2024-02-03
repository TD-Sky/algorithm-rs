pub fn selection<T: Ord>(arr: &mut [T]) {
    for left in 0..arr.len() {
        let min = (left..arr.len()).min_by_key(|&i| &arr[i]).unwrap();
        arr.swap(left, min);
    }
}

#[cfg(test)]
mod tests {
    use super::selection;

    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        selection(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn single_element() {
        let mut arr: [i32; 1] = [7];
        selection(&mut arr);
        assert_eq!(arr, [7]);
    }

    #[test]
    fn already_sorted() {
        let mut arr: [char; 3] = ['a', 'b', 'c'];
        selection(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c']);
    }

    #[test]
    fn basic() {
        let mut arr: [char; 4] = ['d', 'a', 'c', 'b'];
        selection(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c', 'd']);
    }

    #[test]
    fn repeated_elements() {
        let mut arr: [i32; 4] = [542, 542, 542, 542];
        selection(&mut arr);
        assert_eq!(arr, [542, 542, 542, 542]);
    }
}
