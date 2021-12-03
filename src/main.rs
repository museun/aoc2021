fn main() {
    <&[(_, _, fn(String) -> String)]>::into_iter(&[
        (1, 1, move |input| {
            input
                .lines()
                .flat_map(<i32 as ::std::str::FromStr>::from_str)
                .collect::<Vec<_>>()
                .windows(2)
                .flat_map(|a| {
                    matches!(a, [_, _])
                        .then(|| (a[0] < a[1]).then(|| ()))
                        .flatten()
                })
                .count()
                .to_string()
        } as _),
        (1, 2, move |input| {
            input
                .lines()
                .flat_map(|s| s.split_terminator(' ').next())
                .flat_map(<_ as ::std::str::FromStr>::from_str)
                .collect::<Vec<i32>>()
                .windows(3)
                .map(|a| a.iter().sum())
                .collect::<Vec<i32>>()
                .windows(2)
                .flat_map(|a| {
                    matches!(a, [_, _])
                        .then(|| (a[0] < a[1]).then(|| ()))
                        .flatten()
                })
                .count()
                .to_string()
        } as _),
        (2, 1, move |input| {
            input
                .lines()
                .fold([0_i32, 0_i32], |mut t, s| {
                    s.split_once(' ')
                        .and_then(|(a, b)| b.parse().ok().map(|b| (a, b)))
                        .map::<(fn(&mut i32, i32), usize, i32), _>(|(dir, amt)| match dir {
                            "up" => (<_ as std::ops::SubAssign>::sub_assign, 0, amt),
                            "down" => (<_ as std::ops::AddAssign>::add_assign, 0, amt),
                            "forward" => (<_ as std::ops::AddAssign>::add_assign, 1, amt),
                            _ => unreachable!(),
                        })
                        .map(|(f, i, amt)| f(&mut t[i], amt))
                        .map(|_| t)
                        .unwrap()
                })
                .into_iter()
                .product::<i32>()
                .to_string()
        } as _),
        (2, 2, move |input| {
            input
                .lines()
                .fold([0_i32, 0_i32, 0_i32], |mut t, s| {
                    s.split_once(' ')
                        .and_then(|(a, b)| b.parse::<i32>().ok().map(|b| (a, b)))
                        .and_then(|(dir, amt)| {
                            Some(match dir {
                                "up" => t[2] -= amt,
                                "down" => t[2] += amt,
                                "forward" => t[0] += t[2] * amt,
                                _ => (),
                            })
                            .map(|_| match dir {
                                "forward" => t[1] += amt,
                                _ => (),
                            })
                        })
                        .map(|_| t)
                        .unwrap()
                })
                .into_iter()
                .take(2)
                .product::<i32>()
                .to_string()
        } as _),
    ])
    .flat_map(|(day, part, func)| {
        std::fs::read_to_string(&format!(
            "./input/day_{day}_{part}.txt",
            day = day,
            part = part
        ))
        .map(|out| {
            println!(
                "day {day}, part: {part}: {solution}",
                day = day,
                part = part,
                solution = func(out)
            )
        })
    })
    .last()
    .unwrap_or_default()
}
