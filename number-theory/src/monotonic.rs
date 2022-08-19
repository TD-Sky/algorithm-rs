use std::cmp::Ordering;

pub fn is_monotonic<T: Ord>(arr: &[T]) -> bool {
    /* 前置条件: 数组不能为空，因为没有意义 */

    // 宽松单调性标识，初始化为真
    let mut up = true;
    let mut down = true;

    for i in 0..(arr.len() - 1) {
        match arr[i].cmp(&arr[i + 1]) {
            // 出现递减，说明数组不单增
            Ordering::Greater => {
                up = false;
            }

            // 出现递增，说明数组不单减
            Ordering::Less => {
                down = false;
            }

            // 相邻元素相等，符合宽松单调
            Ordering::Equal => (),
        }

        // 或非测试，仅当输入都为假时，输出为真。
        // 两种单调性都被破坏，说明数组不单调
        if !(up || down) {
            return false;
        }
    }

    // 单增 or 单减 or 常数组
    up || down
}

#[cfg(test)]
mod tests {
    use super::is_monotonic;

    #[test]
    fn increasing() {
        let arr = [1, 2, 3, 4];
        assert!(is_monotonic(&arr));
    }

    #[test]
    fn equaling() {
        let arr = [4, 4, 4, 4];
        assert!(is_monotonic(&arr));
    }

    #[test]
    fn single() {
        let arr = [13];
        assert!(is_monotonic(&arr));
    }

    #[test]
    fn not_monotonic() {
        let arr = [1, 1, 4, 5, 1, 4];
        assert!(!is_monotonic(&arr));
    }
}
