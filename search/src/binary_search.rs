pub fn binary_search<T: Ord>(seq: &[T], x: T) -> Option<usize> {
    let mut low = 0;
    let mut high = seq.len();

    while low < high {
        let mid = (low + high) / 2;

        if x > seq[mid] {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    (x == seq[low]).then_some(low)
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn basic() {
        let mut arr: [i32; 5] = [85, 63, 24, 56, 45];
        arr.sort();

        assert_eq!(binary_search(&arr, 24), Some(0));
    }
}
