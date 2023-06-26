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
    fn test_find_longest_common_sequence() {
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

    #[test]
    fn test_find_longest_common_sequence_2() {
        let left = vec![
            String::from("foo"),
        ];
        let right = vec![
            String::from("bar"),
        ];

        assert_eq!(find_longest_common_sequence(&left, &right), (0, 0, 0));
    }

    #[test]
    fn test_find_longest_common_sequence_3() {
        let left = vec![
            String::from("foo"),
        ];
        let right = vec![
            String::from("foo"),
        ];

        assert_eq!(find_longest_common_sequence(&left, &right), (0, 0, 1));
    }

    #[test]
    fn test_find_longest_common_sequence_4() {
        let left = vec![
            String::from("foo"),
            String::from("bar"),
        ];
        let right = vec![
            String::from("foo"),
            String::from("bar"),
        ];

        assert_eq!(find_longest_common_sequence(&left, &right), (0, 0, 2));
    }

    #[test]
    fn test_find_longest_common_sequence_5() {
        let left = vec![
        ];
        let right = vec![
            String::from("foo"),
            String::from("bar"),
        ];

        assert_eq!(find_longest_common_sequence(&left, &right), (0, 0, 0));
    }

    #[test]
    fn test_find_longest_common_sequence_6() {
        let left = vec![
            String::from("foo"),
            String::from("bar"),
        ];
        let right = vec![
        ];

        assert_eq!(find_longest_common_sequence(&left, &right), (0, 0, 0));
    }
}
