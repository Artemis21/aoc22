pub fn max_n<const N: usize, T: PartialOrd>(from: impl IntoIterator<Item = T>) -> [T; N] {
    let mut max: [Option<T>; N] = std::array::from_fn(|_| None);
    for item in from {
        for idx in 0..N {
            if max[idx].as_ref().map_or(true, |max| item > *max) {
                max[idx..].rotate_right(1);
                max[idx] = Some(item);
                break;
            }
        }
    }
    max.map(Option::unwrap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_n() {
        assert_eq!(max_n::<3, usize>(vec![1, 2, 3, 4, 5]), [5, 4, 3]);
        assert_eq!(
            max_n::<3, usize>(vec![2, 6, 5, 4, 10, 9, 1, 7, 3, 8]),
            [10, 9, 8]
        );
    }
}
