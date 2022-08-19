use std::mem;

pub fn msort<T: Ord + Clone>(arr: &mut [T]) {
    let mut aux: Vec<T> = arr.into();

    merge(arr, &mut aux);
}

fn merge<T: Ord + Clone>(arr: &mut [T], aux: &mut [T]) {
    let len = arr.len();

    if len < 2 {
        return;
    }

    let mid = len / 2;

    let (arr_l, arr_r) = arr.split_at_mut(mid);
    let (aux_l, aux_r) = aux.split_at_mut(mid);

    merge(arr_l, aux_l);
    merge(arr_r, aux_r);

    let mut left = 0;
    let mut right = mid;

    for x in aux.iter_mut() {
        if left >= mid || (right < len && arr[left] > arr[right]) {
            mem::swap(x, &mut arr[right]);
            right += 1;
        } else {
            mem::swap(x, &mut arr[left]);
            left += 1;
        }
    }

    arr.clone_from_slice(aux);
}

#[cfg(test)]
mod tests {
    use super::msort;

    #[test]
    fn empty() {
        let mut arr: [u32; 0] = [];

        msort(arr.as_mut_slice());
    }

    #[test]
    fn single() {
        let mut arr = [1];

        msort(arr.as_mut_slice());
    }

    #[test]
    fn top_to_btm() {
        let mut arr = [7, 5, 9, 8, 2, 4, 3, 10, 16, 13, 17, 14, 6u32];

        msort(&mut arr);

        assert_eq!(arr, [2, 3, 4, 5, 6, 7, 8, 9, 10, 13, 14, 16, 17]);
    }
}
