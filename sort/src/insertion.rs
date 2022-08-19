pub fn insertion<T: Ord>(arr: &mut [T]) {
    for out in 1..arr.len() {
        let mut n = out;
        while out > 0 && arr[n] < arr[n - 1] {
            arr.swap(n, n - 1);

            n -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::insertion;

    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        insertion(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn single_element() {
        let mut arr: [i32; 1] = [7];
        insertion(&mut arr);
        assert_eq!(arr, [7]);
    }

    #[test]
    fn already_sorted() {
        let mut arr: [char; 3] = ['a', 'b', 'c'];
        insertion(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c']);
    }

    #[test]
    fn basic() {
        let mut arr = [2, 5, 9, 8, 7, 4, 3, 10, 16, 13];
        insertion(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 7, 8, 9, 10, 13, 16]);
    }

    #[test]
    fn repeated_elements() {
        let mut arr: [i32; 4] = [542, 542, 542, 542];
        insertion(&mut arr);
        assert_eq!(arr, [542, 542, 542, 542]);
    }
}
