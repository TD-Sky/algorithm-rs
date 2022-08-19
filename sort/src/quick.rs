use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;

pub fn qsort<T: Ord + Clone>(arr: &mut [T]) {
    // 打乱数组，防止最坏情况出现
    let mut rng = thread_rng();
    arr.shuffle(&mut rng);

    quick(arr);
}

fn quick<T: Ord + Clone>(part: &mut [T]) {
    if part.len() <= 1 {
        return;
    }

    let mut lt = 0; // 区间[0, lt)的元素都小于sample
    let mut idx = 1; // 待比较游标，不是相等游标！[lt, gt]才是相等区间
    let mut gt = part.len() - 1; // 区间(gt, len)的元素都大于sample
    let sample = part[0].clone();

    // 内循环结束时，idx == gt + 1
    while idx <= gt {
        match part[idx].cmp(&sample) {
            // lt与idx拉开距离也不要紧，
            // 此交换总会拉取更小的元素到lt位置上
            Ordering::Less => {
                part.swap(lt, idx);
                lt += 1;
                idx += 1;
            }

            Ordering::Greater => {
                part.swap(idx, gt);
                gt -= 1;
            }

            Ordering::Equal => idx += 1,
        }
    }

    // 相等区间(含有样本元素)不再参与递归
    quick(&mut part[0..lt]);
    quick(&mut part[(gt + 1)..]);
}

#[cfg(test)]
mod tests {
    use super::qsort;

    #[test]
    fn test_empty() {
        let mut arr: [u32; 0] = [];

        qsort(&mut arr);
    }

    #[test]
    fn test_normal() {
        let mut arr = [7, 5, 9, 8, 2, 4, 3, 10, 16, 13, 17, 14, 6u32];

        qsort(&mut arr);

        assert_eq!(arr, [2, 3, 4, 5, 6, 7, 8, 9, 10, 13, 14, 16, 17]);
    }

    #[test]
    fn test_chars() {
        let mut arr: Vec<char> = "RBWWRWBRRWBR".chars().collect();

        qsort(&mut arr);

        assert_eq!(String::from_iter(arr), "BBBRRRRRWWWW");
    }
}
