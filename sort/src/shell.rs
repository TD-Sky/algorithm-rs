pub fn shell<T: Ord>(arr: &mut [T]) {
    // 初始化间隔
    let mut itrval = 1;
    while itrval < arr.len() / 3 {
        itrval = 3 * itrval + 1;
    }

    // 间隔值为1时变为高完成度的插入排序
    while itrval >= 1 {
        // 间隔值的步进至出界，如此能收集到子数组
        // 子数组可能遍历多遍，从后往前比较、交换，
        // 但它是自前部分做插入排序，然后一直囊括后部分，
        // 或后部分插入排序中止，但此处前部分已有序
        for right in itrval..arr.len() {
            // 插入时挪动的距离为间隔值
            for left in (itrval..=right).rev().step_by(itrval) {
                // 子数组内的值比较
                if arr[left - itrval] > arr[left] {
                    arr.swap(left, left - itrval);
                } else {
                    // 子数组内的双双遍历一定会使其有序，不再有额外动作
                    break;
                }
            }
        }

        // 不断缩小间隔，即减少子数组数量
        itrval /= 3;
    }
}

#[cfg(test)]
mod tests {
    use super::shell;

    #[test]
    fn basic() {
        let mut arr = [7, 5, 9, 8, 2, 4, 3, 10, 16, 13, 17, 14, 6];
        shell(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 6, 7, 8, 9, 10, 13, 14, 16, 17]);
    }
}
