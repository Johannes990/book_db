pub fn edit_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    if s1_chars.is_empty() {
        return s2_chars.len();
    }

    if s2_chars.is_empty() {
        return s1_chars.len();
    }

    let m = s1_chars.len() + 1;
    let n = s2_chars.len() + 1;
    let mut table: Vec<usize> = vec![0; m * n];

    for i in 0..m {
        table[index(i, 0, n)] = i;
    }

    for j in 1..n {
        table[index(0, j, n)] = j;
    }

    for (i, &c1) in s1_chars.iter().enumerate() {
        for (j, &c2) in s2_chars.iter().enumerate() {
            let diff = if c1 == c2 { 0 } else { 1 };
            let prev_row = table[index(i, j + 1, n)] + 1;
            let prev_column = table[index(i + 1, j, n)] + 1;
            let prev_diagonal = table[index(i, j, n)] + diff;

            table[index(i + 1, j + 1, n)] = prev_row.min(prev_column).min(prev_diagonal);
        }
    }

    table[index(m - 1, n - 1, n)]
}

fn index(i: usize, j: usize, n: usize) -> usize {
    i * n + j
}

#[cfg(test)]
mod tests {
    use super::edit_distance;

    #[test]
    fn test_edit_distance_identical_strings_1() {
        let s1 = "Some string";
        let s2 = "Some string";

        assert_eq!(0, edit_distance(s1, s2));
    }

    #[test]
    fn test_edit_distance_identical_strings_2() {
        let s1 = "q";
        let s2 = "q";

        assert_eq!(0, edit_distance(s1, s2));
    }

    #[test]
    fn test_edit_distance_empty_strings_for_s1() {
        let s1 = "";
        let s2_vec = vec!["aaa", "test_string", "eoipcj*&hji43f897y40caikj3498"];

        for s2 in s2_vec {
            assert_eq!(edit_distance(s1, s2), s2.chars().count());
        }
    }

    #[test]
    fn test_edit_distance_empty_strings_for_s2() {
        let s1_vec = vec![
            "ax",
            "_________",
            "0wec9i43f08fduv9578y8gfas908dfubv465ohgijUOYTG(^T",
        ];
        let s2 = "";

        for s1 in s1_vec {
            assert_eq!(edit_distance(s1, s2), s1.chars().count());
        }
    }

    #[test]
    fn test_edit_distance_small_strings() {
        let s1_vec = vec!["aa", "aba", "bar"];
        let s2_vec = vec!["bb", "baa", "baz"];
        let true_distances = vec![2, 2, 1];

        for (idx, s1) in s1_vec.iter().enumerate() {
            assert_eq!(true_distances[idx], edit_distance(s1, s2_vec[idx]));
        }
    }

    #[test]
    fn test_edit_distance_medium_strings() {
        let s1_vec = vec!["ratas", "korgus", "inimene", "sizzlydoop"];
        let s2_vec = vec!["satar", "random", "imeloom", "fizzlygoon"];
        let true_distances = vec![2, 6, 6, 3];

        for (idx, s1) in s1_vec.iter().enumerate() {
            assert_eq!(true_distances[idx], edit_distance(&s1, s2_vec[idx]));
        }
    }

    #[test]
    fn test_edit_distance_longish_strings() {
        let s1_vec = vec![
            "somelongishstring",
            "edededrfrftgtgtgtzzxxccds",
            "12345678909876543321",
        ];
        let s2_vec = vec![
            "somelongisherstring",
            "deeddeedtgtgrfrfrfrxcxcxz",
            "12345678999876543212",
        ];
        let true_distances = vec![2, 16, 3];

        for (idx, s1) in s1_vec.iter().enumerate() {
            assert_eq!(true_distances[idx], edit_distance(s1, s2_vec[idx]));
        }
    }

    #[test]
    fn test_edit_distance_unicode() {
        let s1 = "cafe";
        let s2 = "café";

        let smiley1 = "🐱";
        let smiley2 = "🐶";

        assert_eq!(1, edit_distance(s1, s2));
        assert_eq!(1, edit_distance(smiley1, smiley2));
    }

    #[test]
    fn test_edit_distance_symmetricity() {
        let s1_vec = vec!["asdsds", "sxss", "239d8u897h587yt"];
        let s2_vec = vec!["aaaaa", "sxss", "4sd87y48v98u5gyb"];

        for (idx, s1) in s1_vec.iter().enumerate() {
            assert_eq!(
                edit_distance(s1, s2_vec[idx]),
                edit_distance(s2_vec[idx], s1)
            );
        }
    }
}
