type Input = Vec<u8>;
type Output = u64;

register!(
    "input/day23.txt";
    run(input: Input) -> Output {
        let input = input.remove(0);
        (run1(&input, 100), run2(&input))
    }
);

fn run1(input: &[u8], games: u32) -> Output {
    let cups = run_any(input, 10, games);

    let mut cup = cups[1].next;
    let mut result = String::new();
    while cup != 1 {
        result.push((b'0' + cup as u8) as char);
        cup = cups[cup as usize].next;
    }

    result.parse().unwrap()
}

fn run2(input: &[u8]) -> Output {
    let cups = run_any(input, 1_000_001, 10_000_000);

    let start = cups[1];
    let c1 = cups[start.next as usize];
    let c2 = cups[c1.next as usize];
    u64::from(c1.value) * u64::from(c2.value)
}

#[derive(Debug, Copy, Clone)]
struct Cup {
    value: u32,
    next: u32,
}

fn run_any(input: &[u8], cards: u32, games: u32) -> Vec<Cup> {
    let mut cups = Vec::<Cup>::with_capacity(cards as usize);
    cups.resize(cards as usize, Cup { value: 0, next: 0 });

    let mut prev = -1;
    for value in input
        .iter()
        .copied()
        .map(|c| u32::from(c - b'0'))
        .chain(10..cards)
    {
        let cup = Cup { value, next: 0 };
        cups[value as usize] = cup;

        if prev >= 0 {
            cups[prev as usize].next = value;
        }
        prev = value as i32;
    }

    let mut current = usize::from(input[0] - b'0') as u32;
    cups[prev as usize].next = current;

    #[cfg(debug_assertions)]
    {
        for (i, cup) in cups.iter().enumerate().skip(1) {
            debug_assert!(
                cup.next != 0,
                "Cup {} at index {} is not linked",
                cup.value,
                i
            );
        }
    }

    let limit = cards - 1;
    for _ in 0..games {
        let cup = cups[current as usize];
        let pickup1 = cups[cup.next as usize];
        let pickup2 = cups[pickup1.next as usize];
        let pickup3 = cups[pickup2.next as usize];

        let c1 = pickup1.value;
        let c2 = pickup2.value;
        let c3 = pickup3.value;

        let mut target = cup.value - 1;
        while target == 0 || target == c1 || target == c2 || target == c3 {
            if target == 0 {
                target = limit;
            } else {
                target -= 1;
            }
        }

        let target = &mut cups[target as usize];
        let after_target = target.next;
        target.next = pickup1.value;
        cups[c3 as usize].next = after_target;

        let after_current = pickup3.next;
        cups[current as usize].next = after_current;
        current = after_current;
    }

    cups
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2020::Solution;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 97624853);
        assert_eq!(res2, 664642452305);
    }

    #[test]
    fn test_ex() {
        assert_eq!((67384529, 149245887792), Solver::run_on("389125467"));
    }

    #[test]
    fn test_small_ex() {
        assert_eq!(92658374, run1("389125467".as_bytes(), 10));
    }
}
