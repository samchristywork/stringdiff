fn find_longest_common_sequence(left: &Vec<String>, right: &Vec<String>) -> (usize, usize, usize) {
    let max_possible_overlap = std::cmp::min(left.len(), right.len());

    for len in 0..max_possible_overlap + 1 {
        let len = max_possible_overlap - len;
        for left_offset in 0..(left.len() - len) + 1 {
            let left_slice = &left[left_offset..(left_offset + len)];
            for right_offset in 0..(right.len() - len + 1) {
                let right_slice = &right[right_offset..(right_offset + len)];
                if left_slice == right_slice {
                    return (left_offset, right_offset, len);
                }
            }
        }
    }

    (0, 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_sequence() {
        let left = vec![
            String::from("foo"),
            String::from("bar"),
            String::from("baz"),
            String::from("qux"),
        ];
        let right = vec![
            String::from("bar"),
            String::from("baz"),
            String::from("qux"),
            String::from("quux"),
        ];

        assert_eq!(find_longest_common_sequence(&left, &right), (1, 0, 3));
    }
}
