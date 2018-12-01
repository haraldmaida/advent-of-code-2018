use super::*;

#[test]
fn example() {
    let input = vec![1, -2, 3, 1];

    let answer = sum_frequencies(input);

    assert_eq!(answer, 3);
}

#[test]
fn example2() {
    let input = vec![1, 1, 1];

    let answer = sum_frequencies(input);

    assert_eq!(answer, 3);
}

#[test]
fn example3() {
    let input = vec![1, 1, -2];

    let answer = sum_frequencies(input);

    assert_eq!(answer, 0);
}

#[test]
fn example4() {
    let input = vec![-1, -2, -3];

    let answer = sum_frequencies(input);

    assert_eq!(answer, -6);
}

#[test]
fn example_input() {
    let input = "1\n -2\n3 \n1";

    let answer = solve_part1(input);

    assert_eq!(answer, 3);
}
