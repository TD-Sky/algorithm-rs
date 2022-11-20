// 全排列生成器，按字典序进行
// 前提：切片的元素互异
// 结果：
// - true —— 字典序数+1的全排列
// - false —— 切片长度 ≤ 1 或 降序排列
pub fn next_permutation<T>(seq: &mut [T]) -> bool
where
    T: PartialOrd,
{
    if seq.len() < 2 {
        return false;
    }

    // 寻找最右侧、符合小于关系的下标
    // 找不到就说明传入数列字典序最大了
    let rmost_lt = match (0..seq.len() - 1).rfind(|&i| seq[i] < seq[i + 1]) {
        Some(r) => r,
        None => return false,
    };

    // 寻找 rmost_lt 的最小上确界之下标
    // 由于 rmost_lt 的性质，supremum 至少比它多1
    let supremum = (rmost_lt + 1..seq.len())
        .rfind(|&i| seq[i] > seq[rmost_lt])
        .unwrap();

    // 连同下文
    // 严格控制排列序数+1
    seq.swap(rmost_lt, supremum);

    // rmost_lt 之后的部分按升序列举
    let mut rest = (rmost_lt + 1)..seq.len();
    while let [Some(head), Some(tail)] = [rest.next(), rest.next_back()] {
        seq.swap(head, tail);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::next_permutation;

    #[test]
    fn permute_123() {
        let mut arr = [1, 2, 3];

        next_permutation(&mut arr);
        assert_eq!(arr, [1, 3, 2]);
        next_permutation(&mut arr);
        assert_eq!(arr, [2, 1, 3]);
        next_permutation(&mut arr);
        assert_eq!(arr, [2, 3, 1]);
        next_permutation(&mut arr);
        assert_eq!(arr, [3, 1, 2]);
        next_permutation(&mut arr);
        assert_eq!(arr, [3, 2, 1]);
    }

    #[test]
    fn empty() {
        let mut arr: [(); 0] = [];

        assert!(!next_permutation(&mut arr));
    }

    #[test]
    fn single() {
        let mut arr = [1];

        assert!(!next_permutation(&mut arr));
    }
}
