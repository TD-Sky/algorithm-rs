pub fn bubble<T: Ord>(arr: &mut [T]) {
    for out in 1..arr.len() {
        let mut swapped = false;

        for i in 0..(arr.len() - out) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);

                // 发生了交换
                swapped = true;
            }
        }

        // 内循环没交换，说明排序完成
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble;

    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        bubble(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn single_element() {
        let mut arr: [i32; 1] = [7];
        bubble(&mut arr);
        assert_eq!(arr, [7]);
    }

    #[test]
    fn already_sorted() {
        let mut arr: [char; 3] = ['a', 'b', 'c'];
        bubble(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c']);
    }

    #[test]
    fn basic() {
        let mut arr: [char; 4] = ['d', 'a', 'c', 'b'];
        bubble(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c', 'd']);
    }

    #[test]
    fn repeated_elements() {
        let mut arr: [i32; 4] = [542, 542, 542, 542];
        bubble(&mut arr);
        assert_eq!(arr, [542, 542, 542, 542]);
    }
}
