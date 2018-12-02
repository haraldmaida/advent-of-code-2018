use super::*;

fn prepare(input: Vec<&str>) -> Vec<String> {
    input.into_iter().map(ToOwned::to_owned).collect()
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = prepare(vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ]);

        let answer = checksum(&input);

        assert_eq!(answer, 12);
    }
}
