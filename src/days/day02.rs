use crate::part_result::PartResult;

#[derive(Debug)]
struct Report {
    levels: Vec<u64>,
}

pub fn main(input: &str, output: &mut PartResult) {
    let mut reports = Vec::<Report>::new();
    for line in input.lines() {
        let mut report = Report { levels: vec![] };
        for token in line.split_ascii_whitespace() {
            report.levels.push(token.parse::<u64>().unwrap());
        }
        reports.push(report);
    }

    fn is_safe(levels: &[u64], reversed: bool, skip_idx: Option<usize>) -> bool {
        fn is_safe3<'a>(mut levels: impl Iterator<Item = &'a u64>) -> bool {
            let mut cur = levels.next().unwrap();
            for next in levels {
                if next <= cur {
                    return false;
                }
                let diff = next - cur;
                if diff < 1 || diff > 3 {
                    return false;
                }

                cur = next;
            }
            true
        }

        fn is_safe2<'a>(levels: impl Iterator<Item = &'a u64>, skip_idx: usize) -> bool {
            is_safe3(levels.enumerate().filter_map(
                |(i, v)| {
                    if i == skip_idx {
                        None
                    } else {
                        Some(v)
                    }
                },
            ))
        }

        match (reversed, skip_idx) {
            (false, None) => is_safe3(levels.iter()),
            (true, None) => is_safe3(levels.iter().rev()),
            (false, Some(idx)) => is_safe2(levels.iter(), idx),
            (true, Some(idx)) => is_safe2(levels.iter().rev(), idx),
        }
    }

    let safe_count = reports
        .iter()
        .map(|report| &report.levels)
        .filter_map(|levels| {
            if is_safe(levels, false, None) || is_safe(levels, true, None) {
                Some(())
            } else {
                None
            }
        })
        .count();

    output.part1(safe_count as i64);

    let safe_dampened_count = reports
        .iter()
        .map(|report| &report.levels)
        .filter_map(|levels| {
            if is_safe(levels, false, None) || is_safe(levels, true, None) {
                return Some(());
            }
            for skip_idx in 0..levels.len() {
                if is_safe(levels, false, Some(skip_idx)) || is_safe(levels, true, Some(skip_idx)) {
                    return Some(());
                }
            }
            return None;
        })
        .count();

    output.part2(safe_dampened_count as i64);
}
