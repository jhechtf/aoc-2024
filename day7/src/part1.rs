pub fn part1(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            let Some((target, args)) = line.split_once(": ") else {
                panic!("invalid line: {line}");
            };
            let target = target.parse::<i64>().expect("a target");
            let args = args
                .split(' ')
                .map(|x| x.parse::<i64>().expect(" a number"))
                .collect::<Vec<_>>();
            (target, args)
        })
        .map(|(target, args)| {
            let mut results = vec![args[0]];
            for arg in args.iter().skip(1) {
                let copy = results.clone();
                results.clear();
                for result in copy {
                    let add = result + arg;
                    if add <= target {
                        results.push(add);
                    }

                    let mul = result * arg;
                    if mul <= target {
                        results.push(mul);
                    }
                }
            }

            if results.contains(&target) {
                target
            } else {
                0
            }
        })
        .sum()
}
