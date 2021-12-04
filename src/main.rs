#![cfg_attr(debug_assertions, allow(dead_code, unused_variables, unreachable_code))]
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
                        .map::<Vec<(fn(&mut i32, i32), _, _)>, _>(|(dir, amt)| match dir {
                            "up" => vec![(<_ as std::ops::SubAssign>::sub_assign, 2, amt)],
                            "down" => vec![(<_ as std::ops::AddAssign>::add_assign, 2, amt)],
                            "forward" => vec![
                                (<_ as std::ops::AddAssign>::add_assign, 0, t[2] * amt),
                                (<_ as std::ops::AddAssign>::add_assign, 1, amt),
                            ],
                            _ => unreachable!(),
                        })
                        .map(|v| v.into_iter().for_each(|(f, i, amt)| f(&mut t[i], amt)))
                        .map(|_| t)
                        .unwrap()
                })
                .into_iter()
                .take(2)
                .product::<i32>()
                .to_string()
        } as _),
        (3, 1, move |input| {
            input
                .lines()
                .map(|s| s.trim().chars().map(|c| c as u8 - b'0').collect())
                .map(Some)
                .collect::<Option<Vec<Vec<_>>>>()
                .into_iter()
                .map(|s| {
                    s.into_iter()
                        .enumerate()
                        .flat_map(|(x, e)| e.into_iter().enumerate().map(move |(y, f)| (x, y, f)))
                        .fold::<Vec<Vec<_>>, _>(
                            std::iter::repeat(
                                std::iter::repeat(0_u8)
                                    .take(input.lines().count())
                                    .collect(),
                            )
                            .take(input.lines().map(str::len).max().unwrap_or_default())
                            .collect(),
                            |mut v, (x, y, e)| Some(v[y][x] = e).map(|_| v).unwrap(),
                        )
                })
                .last()
                .into_iter()
                .map(|seq| {
                    [<_ as Iterator>::max_by, <_ as Iterator>::min_by]
                        .into_iter()
                        .map(|f| {
                            seq.iter()
                                .flat_map(|s| {
                                    f(
                                        s.iter()
                                            .fold(
                                                <std::collections::HashMap<u8, usize>>::new(),
                                                |mut h, a| {
                                                    std::iter::once(*h.entry(*a).or_default() += 1)
                                                        .next()
                                                        .map(|_| h)
                                                        .unwrap()
                                                },
                                            )
                                            .into_iter(),
                                        |(_, l): &(u8, usize), (_, r): &(u8, usize)| l.cmp(&r),
                                    )
                                    .map(|(k, _)| k)
                                })
                                .map(|a| a as usize)
                                .fold(0usize, |g, a| 2 * g + a)
                        })
                        .product::<usize>()
                })
                .last()
                .unwrap()
                .to_string()
        } as _),
        (3, 2, move |input| {
            // input
            // .lines()
            todo!();
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
