#[derive(Debug, PartialEq)]
pub enum DiffType {
    Common,
    Add,
    Remove,
}

fn find_longest_common_sequence<T: std::cmp::PartialEq>(left: &Vec<T>, right: &Vec<T>) -> (usize, usize, usize) {
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

pub fn annotate_sequence<T: std::cmp::PartialEq + std::clone::Clone>(left: Vec<T>, right: Vec<T>) -> Vec<(T, DiffType)> {
    let (left_offset, right_offset, len) = find_longest_common_sequence(&left, &right);

    if len == 0 {
        let mut ret = Vec::new();
        for x in left {
            ret.push((x, DiffType::Remove));
        }
        for x in right {
            ret.push((x, DiffType::Add));
        }
        return ret;
    }

    let left_prefix = &left[0..left_offset];
    let left_suffix = &left[(left_offset + len)..left.len()];

    let common = &left[left_offset..(left_offset + len)];

    let right_prefix = &right[0..right_offset];
    let right_suffix = &right[(right_offset + len)..right.len()];

    let mut ret = Vec::new();

    let left_ret = annotate_sequence(left_prefix.to_vec(), right_prefix.to_vec());
    let right_ret = annotate_sequence(left_suffix.to_vec(), right_suffix.to_vec());

    for x in left_ret {
        ret.push(x);
    }

    for x in common {
        ret.push((x.clone(), DiffType::Common));
    }

    for x in right_ret {
        ret.push(x);
    }

    ret
}

pub fn annotate_strings(left: &String, right: &String) -> Vec<(String, DiffType)> {
    let left_words: Vec<String> = left.split_whitespace().map(|s| s.to_string()).collect();
    let right_words: Vec<String> = right.split_whitespace().map(|s| s.to_string()).collect();

    annotate_sequence(left_words, right_words)
}

pub fn colorize(ret: &Vec<(String, DiffType)>) {
        for x in ret {
            match x.1 {
                DiffType::Common => print!("\x1b[0m"),
                DiffType::Add => print!("\x1b[32m"),
                DiffType::Remove => print!("\x1b[31m"),
            }
            print!("{} ", x.0);
            print!("\x1b[0m");
        }

        println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_longest_common_sequence_1() {
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

    #[test]
    fn test_annotate_strings_1() {
        let left =  String::from("foo");
        let right = String::from("bar");

        let output = vec![
            (String::from("foo"), DiffType::Remove),
            (String::from("bar"), DiffType::Add),
        ];

        assert_eq!(annotate_strings(&left, &right), output);
    }

    #[test]
    fn test_annotate_strings_2() {
        let left =  String::from("foo bar");
        let right = String::from("bar baz");

        let output = vec![
            (String::from("foo"), DiffType::Remove),
            (String::from("bar"), DiffType::Common),
            (String::from("baz"), DiffType::Add),
        ];

        assert_eq!(annotate_strings(&left, &right), output);
    }

    #[test]
    fn test_annotate_strings_3() {
        let left =  String::from("foo bar baz");
        let right = String::from("bar baz qux");

        let output = vec![
            (String::from("foo"), DiffType::Remove),
            (String::from("bar"), DiffType::Common),
            (String::from("baz"), DiffType::Common),
            (String::from("qux"), DiffType::Add),
        ];

        assert_eq!(annotate_strings(&left, &right), output);
    }

    #[test]
    fn test_annotate_strings_4() {
        let left =  String::from("This is a pretty longish string. I don't know if you can get it right.");
        let right = String::from("This is a pretty long string. I don't know if you can get it wrong.");

        let output = vec![
            (String::from("This"), DiffType::Common),
            (String::from("is"), DiffType::Common),
            (String::from("a"), DiffType::Common),
            (String::from("pretty"), DiffType::Common),
            (String::from("longish"), DiffType::Remove),
            (String::from("long"), DiffType::Add),
            (String::from("string."), DiffType::Common),
            (String::from("I"), DiffType::Common),
            (String::from("don't"), DiffType::Common),
            (String::from("know"), DiffType::Common),
            (String::from("if"), DiffType::Common),
            (String::from("you"), DiffType::Common),
            (String::from("can"), DiffType::Common),
            (String::from("get"), DiffType::Common),
            (String::from("it"), DiffType::Common),
            (String::from("right."), DiffType::Remove),
            (String::from("wrong."), DiffType::Add),
        ];

        assert_eq!(annotate_strings(&left, &right), output);
    }

    #[test]
    fn test_annotate_strings_5() {
        let left =  String::from("hi there");
        let right = String::from("there");

        let output = vec![
            (String::from("hi"), DiffType::Remove),
            (String::from("there"), DiffType::Common),
        ];

        assert_eq!(annotate_strings(&left, &right), output);
    }
}
