#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiffType {
    Common,
    Add,
    Remove,
}

fn find_longest_common_sequence<T: std::cmp::PartialEq>(
    left: &[T],
    right: &[T],
) -> (usize, usize, usize) {
    let n = left.len();
    let m = right.len();

    if n == 0 || m == 0 {
        return (0, 0, 0);
    }

    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    let mut max_len = 0;
    let mut max_left = 0;
    let mut max_right = 0;

    for i in 1..=n {
        for j in 1..=m {
            if left[i - 1] == right[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                if dp[i][j] > max_len {
                    max_len = dp[i][j];
                    max_left = i - max_len;
                    max_right = j - max_len;
                }
            }
        }
    }

    (max_left, max_right, max_len)
}

#[must_use]
pub fn annotate_sequence<T: std::cmp::PartialEq + std::clone::Clone>(
    left: &[T],
    right: &[T],
) -> Vec<(T, DiffType)> {
    enum WorkItem<'a, T> {
        Annotate(&'a [T], &'a [T]),
        Emit(&'a [T], DiffType),
    }

    let mut stack = vec![WorkItem::Annotate(left, right)];
    let mut ret = Vec::new();

    while let Some(item) = stack.pop() {
        match item {
            WorkItem::Emit(items, diff_type) => {
                for x in items {
                    ret.push((x.clone(), diff_type.clone()));
                }
            }
            WorkItem::Annotate(left, right) => {
                let (lo, ro, len) = find_longest_common_sequence(left, right);

                if len == 0 {
                    stack.push(WorkItem::Emit(right, DiffType::Add));
                    stack.push(WorkItem::Emit(left, DiffType::Remove));
                } else {
                    stack.push(WorkItem::Annotate(&left[lo + len..], &right[ro + len..]));
                    stack.push(WorkItem::Emit(&left[lo..lo + len], DiffType::Common));
                    stack.push(WorkItem::Annotate(&left[..lo], &right[..ro]));
                }
            }
        }
    }

    ret
}

#[must_use]
pub fn annotate_strings(left: &str, right: &str) -> Vec<(String, DiffType)> {
    let left_words: Vec<String> = left
        .split_whitespace()
        .map(std::string::ToString::to_string)
        .collect();
    let right_words: Vec<String> = right
        .split_whitespace()
        .map(std::string::ToString::to_string)
        .collect();

    annotate_sequence(&left_words, &right_words)
}

pub fn colorize(ret: &[(String, DiffType)], out: &mut impl std::io::Write) {
    for x in ret {
        let color = match x.1 {
            DiffType::Common => "\x1b[0m",
            DiffType::Add => "\x1b[32m",
            DiffType::Remove => "\x1b[31m",
        };
        write!(out, "{}{} \x1b[0m", color, x.0).unwrap();
    }
    writeln!(out).unwrap();
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
        let left = vec![String::from("foo")];
        let right = vec![String::from("bar")];

        assert_eq!(find_longest_common_sequence(&left, &right), (0, 0, 0));
    }

    #[test]
    fn test_find_longest_common_sequence_3() {
        let left = vec![String::from("foo")];
        let right = vec![String::from("foo")];

        assert_eq!(find_longest_common_sequence(&left, &right), (0, 0, 1));
    }

    #[test]
    fn test_find_longest_common_sequence_4() {
        let left = vec![String::from("foo"), String::from("bar")];
        let right = vec![String::from("foo"), String::from("bar")];

        assert_eq!(find_longest_common_sequence(&left, &right), (0, 0, 2));
    }

    #[test]
    fn test_find_longest_common_sequence_5() {
        let left = vec![];
        let right = vec![String::from("foo"), String::from("bar")];

        assert_eq!(find_longest_common_sequence(&left, &right), (0, 0, 0));
    }

    #[test]
    fn test_find_longest_common_sequence_6() {
        let left = vec![String::from("foo"), String::from("bar")];
        let right = vec![];

        assert_eq!(find_longest_common_sequence(&left, &right), (0, 0, 0));
    }

    #[test]
    fn test_annotate_strings_1() {
        let left = String::from("foo");
        let right = String::from("bar");

        let output = vec![
            (String::from("foo"), DiffType::Remove),
            (String::from("bar"), DiffType::Add),
        ];

        assert_eq!(annotate_strings(&left, &right), output);
    }

    #[test]
    fn test_annotate_strings_2() {
        let left = String::from("foo bar");
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
        let left = String::from("foo bar baz");
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
        let left =
            String::from("This is a pretty longish string. I don't know if you can get it right.");
        let right =
            String::from("This is a pretty long string. I don't know if you can get it wrong.");

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
        let left = String::from("hi there");
        let right = String::from("there");

        let output = vec![
            (String::from("hi"), DiffType::Remove),
            (String::from("there"), DiffType::Common),
        ];

        assert_eq!(annotate_strings(&left, &right), output);
    }
}
