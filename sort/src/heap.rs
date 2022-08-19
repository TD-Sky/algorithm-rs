fn sink<T: Ord>(arr: &mut [T], mut parent: usize) {
    let last = arr.len() - 1;

    loop {
        let left = 2 * parent + 1;

        if left > last {
            break;
        }

        let right = left + 1;

        // 确认左子节点不为最后父节点，
        // 再取最大子节点
        let max = match left != last && arr[right] > arr[left] {
            true => right,
            false => left,
        };

        // 将传入的根下沉到终点
        if arr[parent] < arr[max] {
            arr.swap(parent, max);
        }

        parent = max;
    }

    // 局部堆有序了
}

pub fn heap<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    if len < 2 {
        return;
    }

    // 构建大顶堆
    // 从底向顶遍历所有父节点
    for parent in (0..=len / 2 - 1).rev() {
        sink(arr, parent);
    }

    for end in (1..len).rev() {
        // 释放最大节点至数组末
        arr.swap(0, end);

        // 重新堆化
        sink(&mut arr[..end], 0);
    }
}

#[cfg(test)]
mod tests {
    use super::heap;

    #[test]
    fn empty() {
        let mut arr: [u32; 0] = [];

        heap(&mut arr);
    }

    #[test]
    fn basic() {
        let mut arr: [u32; 11] = [0, 2, 5, 9, 8, 7, 4, 3, 10, 16, 13];

        heap(&mut arr);

        assert_eq!(arr, [0, 2, 3, 4, 5, 7, 8, 9, 10, 13, 16]);
    }
}
