// 全排列生成器，按字典序进行
// 前提：切片的元素互异
// 结果：
// - Some(&[T]) —— 字典序数+1的全排列
// - None —— 切片长度 ≤ 1 或 降序排列
pub fn next_permutation<T>(seq: &mut [T]) -> Option<&[T]>
where
    T: PartialOrd,
{
    if seq.len() < 2 {
        return None;
    }

    // 寻找最右侧、符合小于关系的下标
    // 找不到就说明传入数列字典序最大了
    let rmost_lt = (0..seq.len() - 1).rfind(|&i| seq[i] < seq[i + 1])?;

    // 寻找 rmost_lt 的最小上确界之下标
    // 由于 rmost_lt 的性质，supermum 至少比它多1
    let supermum = (rmost_lt + 1..seq.len())
        .rfind(|&i| seq[i] > seq[rmost_lt])
        .unwrap();

    // 连同下文
    // 严格控制排列序数+1
    seq.swap(rmost_lt, supermum);

    // rmost_lt 之后的部分按升序列举
    let mut rest = (rmost_lt + 1..seq.len()).into_iter();
    while let [Some(head), Some(tail)] = [rest.next(), rest.next_back()] {
        seq.swap(head, tail);
    }

    return Some(seq);
}

#[cfg(test)]
mod tests {
    use super::next_permutation;

    #[test]
    fn permute_123() {
        let mut arr = [1, 2, 3];

        assert_eq!(next_permutation(&mut arr), Some([1, 3, 2].as_slice()));
        assert_eq!(next_permutation(&mut arr), Some([2, 1, 3].as_slice()));
        assert_eq!(next_permutation(&mut arr), Some([2, 3, 1].as_slice()));
        assert_eq!(next_permutation(&mut arr), Some([3, 1, 2].as_slice()));
        assert_eq!(next_permutation(&mut arr), Some([3, 2, 1].as_slice()));
    }

    #[test]
    fn empty() {
        let mut arr: [(); 0] = [];

        assert_eq!(next_permutation(&mut arr), None);
    }

    #[test]
    fn single() {
        let mut arr = [1];

        assert_eq!(next_permutation(&mut arr), None);
    }
}
