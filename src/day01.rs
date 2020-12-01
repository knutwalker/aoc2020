register!(
    "input/day1.txt";
    run(input: parse u64) -> u64 {
        input.sort_unstable();
        (
            find_pair_and_prod(2020, &input).expect("no solution"),
            find_triple_and_prod(2020, &input),
        )
    }
);

fn find_triple_and_prod(target_sum: u64, mut items: &[u64]) -> u64 {
    while let Some((&item, rest)) = items.split_first() {
        if let Some(remainder) = target_sum.checked_sub(item) {
            if let Some(prod) = find_pair_and_prod(remainder, rest) {
                return prod * item;
            }
        };
        items = rest;
    }
    panic!("no solution")
}

fn find_pair_and_prod(target_sum: u64, mut items: &[u64]) -> Option<u64> {
    while let Some((&item, rest)) = items.split_first() {
        if let Some(remainder) = target_sum.checked_sub(item) {
            if let Ok(index) = rest.binary_search(&remainder) {
                return Some(item * rest[index]);
            }
        };
        items = rest;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2020::Solution;

    #[test]
    fn test_ex() {
        let items = vec![1721, 979, 366, 299, 675, 1456];
        let (res1, res2) = Solver::run(items);
        assert_eq!(res1, 514579);
        assert_eq!(res2, 241861950);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 805731);
        assert_eq!(res2, 192684960);
    }
}
