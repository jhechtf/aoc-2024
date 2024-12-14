pub fn part2(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            let Some((target, args)) = line.split_once(": ") else {
                panic!("Invalid line: {line}");
            };
            let target = target.parse::<i64>().expect("a number");
            let args = args
                .split(' ')
                .map(|x| x.parse::<i64>().expect("a number"))
                .collect::<Vec<_>>();
            (target, args)
        })
        .map(|(target, args)| {
            let mut results = vec![args[0]];
            for arg in args.iter().skip(1) {
                let mut new_results = vec![];
                for result in results.iter() {
                    let add = result + arg;
                    if add <= target {
                        new_results.push(add);
                    }

                    let mul = result * arg;
                    if mul <= target {
                        new_results.push(mul);
                    }

                    let concat = match arg {
                        c if *c < 10 => result * 10 + arg,
                        c if *c < 100 => result * 100 + arg,
                        c if *c < 1000 => result * 1000 + arg,
                        _ => format!("{}{}", result, arg)
                            .parse::<i64>()
                            .expect(" a number"),
                    };
                    if concat <= target {
                        new_results.push(concat);
                    }
                }
                results = new_results;
            }

            if results.contains(&target) {
                target
            } else {
                0
            }
        })
        .sum()
}
