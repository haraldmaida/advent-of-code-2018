use std::str::FromStr;

pub type Frequency = i32;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> Frequency {
    sum_frequencies(
        input
            .split('\n')
            .filter(|s| !s.is_empty())
            .enumerate()
            .map(|(idx, text)| {
                Frequency::from_str(text.trim()).expect(&format!(
                    "input text at line {:03} is not an i32 but is {:?}",
                    idx + 1,
                    text
                ))
            }),
    )
}

pub fn sum_frequencies(frequencies: impl IntoIterator<Item = Frequency>) -> Frequency {
    frequencies.into_iter().sum()
}

#[cfg(test)]
mod tests;
