use std::{cmp::Ordering, num::ParseIntError};

fn main() {
    let input = std::fs::read_to_string("./files/02.txt").unwrap();
    let reports = read_input(&input).unwrap();

    println!(
        "PART 1. Number of safe reports: {}",
        count_safe_reports(&reports)
    );
    println!(
        "PART 2. Number of safe reports with Problem Dampener(can skip one error): {}",
        count_safe_reports_with_problem_dampener(&reports)
    );
}

pub fn read_input(input: &str) -> Result<Vec<Vec<u8>>, ParseIntError> {
    input
        .lines()
        .map(|s| {
            s.split(" ")
                .map(|s| s.parse::<u8>())
                .collect::<Result<Vec<u8>, ParseIntError>>()
        })
        .collect()
}

pub fn count_safe_reports(reports: &[Vec<u8>]) -> usize {
    reports
        .iter()
        .filter(|report| check_report(report, false).is_ok())
        .collect::<Vec<_>>()
        .len()
}

pub fn count_safe_reports_with_problem_dampener(reports: &[Vec<u8>]) -> usize {
    reports
        .iter()
        .filter(|report| check_report(report, true).is_ok())
        .collect::<Vec<_>>()
        .len()
}

#[derive(Debug)]
pub struct IncorrectReportError;

pub fn check_report(
    report: &[u8],
    can_skip_one_digit_error: bool,
) -> Result<(), IncorrectReportError> {
    let try_report = |report: &[u8]| {
        let mut previous_n: Option<u8> = None;
        let mut previous_ord: Option<Ordering> = None;
        for n in report.iter() {
            let result = check_digit(n, &mut previous_n, &mut previous_ord);
            if result.is_err() {
                return Err(());
            }
        }

        Ok(())
    };

    let Err(_) = try_report(report) else {
        return Ok(());
    };
    if !can_skip_one_digit_error {
        return Err(IncorrectReportError);
    }

    let mut index = 0;
    loop {
        if index == report.len() {
            return Err(IncorrectReportError);
        }

        let mut vec = Vec::from_iter(report)
            .into_iter()
            .copied()
            .collect::<Vec<_>>();
        vec.remove(index);

        if let Ok(()) = try_report(&vec) {
            return Ok(());
        };

        index += 1;
    }
}

pub enum CheckDigitError {
    Ordering,
    LevelDifferenceOutOfBounds,
}

pub fn check_digit(
    n: &u8,
    previous_n: &mut Option<u8>,
    previous_ord: &mut Option<Ordering>,
) -> Result<(), CheckDigitError> {
    let Some(prev) = previous_n else {
        *previous_n = Some(*n);
        return Ok(());
    };

    if !matches!(n.abs_diff(*prev), 1..=3) {
        return Err(CheckDigitError::LevelDifferenceOutOfBounds);
    };

    let ord = n.cmp(prev);
    let prev_ord = match *previous_ord {
        Some(p) => p,
        None => ord,
    };

    if ord != prev_ord {
        return Err(CheckDigitError::Ordering);
    }

    *previous_n = Some(*n);
    *previous_ord = Some(ord);

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn count_safe_reports() {
        let example: &[Vec<u8>] = &[
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(2, super::count_safe_reports(example))
    }

    #[test]
    fn count_safe_reports_with_problem_dampener() {
        let example: &[Vec<u8>] = &[
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(4, super::count_safe_reports_with_problem_dampener(example))
    }
}
