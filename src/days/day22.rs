use crate::day_output::DayOutput;

#[derive(Debug, Clone, Copy)]
struct SecretNumber(usize);

pub fn main(input: &str, output: &mut DayOutput) {
    let mut secrets = Vec::<SecretNumber>::new();
    for line in input.lines() {
        let sn = line.parse::<usize>().unwrap();
        secrets.push(SecretNumber(sn));
    }
    let secrets = secrets;

    let mut all_frequencies = vec![0; 2usize.pow(20)];
    let mut part1 = 0usize;

    let mut history = Vec::<usize>::new();
    let mut frequencies = vec![usize::MAX; 2usize.pow(20)];
    for (sn_idx, sn) in secrets.iter().enumerate() {
        history.clear();
        let mut sn = sn.0;
        history.push(sn);
        for _ in 0..2000 {
            // Multiply by 64
            let mulby64 = sn * 64;
            // Mix
            let sn2 = sn ^ mulby64;
            // Prune
            let sn2 = sn2 % 16777216;
            // Divide by 32 and round down
            let divby32 = sn2 / 32;
            // Mix
            let sn3 = sn2 ^ divby32;
            // Prune
            let sn3 = sn3 % 16777216;
            // Multiply by 2048
            let mulby2048 = sn3 * 2048;
            // Mix
            let sn4 = sn3 ^ mulby2048;
            // Prune
            let sn4 = sn4 % 16777216;

            sn = sn4;
            history.push(sn);
        }
        part1 += sn;

        for i in 4..history.len() {
            let delta = |off: usize| -> i8 {
                let lhs = (history[i - off] % 10) as i8;
                let rhs = (history[i - off - 1] % 10) as i8;
                lhs - rhs
            };
            let arr = [delta(3), delta(2), delta(1), delta(0)];
            let bananas = history[i] % 10;

            let compress = |idx: usize| -> usize { ((arr[idx] + 10) as usize) << (idx * 5) };
            let compressed = compress(0) | compress(1) | compress(2) | compress(3);

            if frequencies[compressed] != sn_idx {
                frequencies[compressed] = sn_idx;
                all_frequencies[compressed] += bananas;
            }
        }
    }
    output.part1(part1.to_string());

    let part2 = all_frequencies.iter().max().unwrap();
    output.part2(part2.to_string());
}
