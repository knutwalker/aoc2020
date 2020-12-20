use std::{collections::HashSet, iter::successors};

type Input = String;
type Output = u64;

register!(
    "input/day20.txt";
    run(input: chunk Input) -> Output {
        run(input)
    }
);

fn run(input: Vec<Vec<Input>>) -> (Output, Output) {
    let tiles = input
        .into_iter()
        .map(|t| {
            let id = t[0][5..9].parse::<i16>().unwrap();
            let sum = t[1..].iter().fold(0_u128, |sum, line| {
                line.bytes()
                    .map(|c| match c {
                        b'#' => 1,
                        b'.' => 0,
                        x => unreachable!("not # or . : {}", x),
                    })
                    .fold(sum, |sum, digit| sum << 1 | digit)
            });
            (id, sum)
        })
        .collect::<Vec<_>>();

    let dim = (tiles.len() as f32).sqrt() as usize;

    let mut grid = (0..dim)
        .map(|_| vec![(-1_i16, 0_u128); dim])
        .collect::<Vec<_>>();

    let mut used = HashSet::new();

    let mut pt1 = Vec::new();
    let mut pt2 = Vec::new();

    dfs(
        &mut grid,
        0,
        0,
        &tiles,
        &mut used,
        &mut |r| pt1.push(r),
        &mut |r| pt2.push(r),
    );

    pt1.sort_unstable();
    pt1.dedup();
    pt2.sort_unstable();
    pt2.dedup();

    (
        pt1.into_iter().next().unwrap(),
        pt2.into_iter().next().unwrap(),
    )
}

type Tl = (i16, u128);

fn dfs(
    grid: &mut Vec<Vec<Tl>>,
    row: usize,
    column: usize,
    tiles: &[Tl],
    used: &mut HashSet<i16>,
    solution1: &mut impl FnMut(Output),
    solution2: &mut impl FnMut(Output),
) {
    if grid.len() == row {
        part2(grid, solution2);
        let pt1 = (grid[0][0].0 as Output)
            * (grid[0][grid[0].len() - 1].0 as Output)
            * (grid[grid.len() - 1][0].0 as Output)
            * (grid[grid.len() - 1][grid[0].len() - 1].0 as Output);

        solution1(pt1);

        return;
    }

    if grid[row].len() == column {
        return dfs(grid, row + 1, 0, tiles, used, solution1, solution2);
    }

    for (id, tile) in tiles {
        if used.contains(id) {
            continue;
        }
        for candidate in all_rotations(*tile) {
            let mut good = true;
            if row > 0 {
                good &= top(candidate) == bottom(grid[row - 1][column].1);
            }
            if column > 0 {
                good &= left(candidate) == right(grid[row][column - 1].1);
            }
            if good {
                grid[row][column] = (*id, candidate);
                used.insert(*id);
                dfs(grid, row, column + 1, tiles, used, solution1, solution2);
                used.remove(id);
                grid[row][column] = (-1, 0);
            }
        }
    }
}

fn all_rotations(tile: u128) -> impl Iterator<Item = u128> {
    successors(Some(tile), |t| Some(rotate(*t)))
        .take(4)
        .chain(successors(Some(flip(tile)), |t| Some(rotate(*t))).take(4))
}

fn rotate(tile: u128) -> u128 {
    (0..10)
        .flat_map(|i| {
            (0..10).filter_map(move |j| {
                if (tile & (1_u128 << (10 * i + j))) != 0 {
                    Some(1_u128 << (10 * j + (9 - i)))
                } else {
                    None
                }
            })
        })
        .fold(0_u128, |sum, digit| sum | digit)
}

fn flip(tile: u128) -> u128 {
    (0..10)
        .flat_map(|i| {
            (0..10).filter_map(move |j| {
                if (tile & (1_u128 << (10 * i + j))) != 0 {
                    Some(1_u128 << (10 * i + 9 - j))
                } else {
                    None
                }
            })
        })
        .fold(0_u128, |sum, digit| sum | digit)
}

fn top(tile: u128) -> u128 {
    (0..10)
        .rev()
        .scan(99, |index, i| {
            let digit = if tile & (1_u128 << *index) != 0 {
                Some(1 << i)
            } else {
                Some(0)
            };
            *index -= 1;
            digit
        })
        .fold(0_u128, |sum, digit| sum | digit)
}

fn bottom(tile: u128) -> u128 {
    tile & 1023
}

fn left(tile: u128) -> u128 {
    (0..10)
        .rev()
        .scan(99, |index, i| {
            let digit = if tile & (1_u128 << *index) != 0 {
                Some(1 << i)
            } else {
                Some(0)
            };
            *index -= 10;
            digit
        })
        .fold(0_u128, |sum, digit| sum | digit)
}

fn right(tile: u128) -> u128 {
    (0..10)
        .rev()
        .scan(90, |index, i| {
            let digit = if tile & (1_u128 << *index) != 0 {
                Some(1 << i)
            } else {
                Some(0)
            };
            *index -= 10;
            digit
        })
        .fold(0_u128, |sum, digit| sum | digit)
}

fn part2(grid: &mut Vec<Vec<Tl>>, solution: impl FnMut(Output)) {
    let mut possible_grid = Vec::new();
    let n = grid.len();
    for i in 0..n {
        for j in 0..n {
            // part1: 0..10
            let tile = (1..9)
                .map(|k| {
                    // part1: 0..10
                    (1..9)
                        .map(|a| {
                            if grid[i][j].1 & (1_u128 << (99 - (10 * k + a))) != 0 {
                                b'#'
                            } else {
                                b'.'
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let tile_row = tile.len() * i;
            for (row_index, row) in tile.into_iter().enumerate() {
                if tile_row + row_index >= possible_grid.len() {
                    possible_grid.resize_with(tile_row + row_index + 1, || Vec::new());
                }
                possible_grid[tile_row + row_index].extend(row);
            }
        }
    }

    find(possible_grid, solution)
}

fn find(grid: Vec<Vec<u8>>, mut solution: impl FnMut(Output)) {
    const SEA_MONSTER: &str = "
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
";

    let pattern = SEA_MONSTER
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.as_bytes())
        .collect::<Vec<_>>();
    for grid in find_rotations(grid) {
        let mut used = grid
            .iter()
            .map(|row| vec![false; row.len()])
            .collect::<Vec<_>>();
        let mut i = 0;
        while i + pattern.len() <= grid.len() {
            let mut j = 0;
            while j + pattern[0].len() <= grid[i].len() {
                let mut good = true;
                'outer: for a in 0..pattern.len() {
                    for b in 0..pattern[a].len() {
                        if pattern[a][b] == b'#' && grid[i + a][j + b] != b'#' {
                            good = false;
                            break 'outer;
                        }
                    }
                }
                if good {
                    for a in 0..pattern.len() {
                        for b in 0..pattern[a].len() {
                            if pattern[a][b] == b'#' {
                                used[i + a][j + b] = true;
                            }
                        }
                    }
                }
                j += 1
            }
            i += 1
        }

        let candidate = (0..grid.len())
            .map(move |i| {
                (0..grid[i].len())
                    .filter(|&j| grid[i][j] == b'#' && !used[i][j])
                    .count() as Output
            })
            .sum();
        solution(candidate);
    }
}

fn find_rotations(grid: Vec<Vec<u8>>) -> impl Iterator<Item = Vec<Vec<u8>>> {
    let flipped = flip_grid(&grid);
    successors(Some(grid), |g| Some(rotate_grid(g)))
        .take(4)
        .chain(successors(Some(flipped), |g| Some(rotate_grid(g))).take(4))
}

fn rotate_grid(grid: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let n = grid.len();

    let line = vec![b'#'; n];
    let mut new_grid = vec![line; n];

    for i in 0..n {
        for j in 0..n {
            new_grid[j][n - 1 - i] = grid[i][j];
        }
    }

    new_grid
}

fn flip_grid(grid: &[Vec<u8>]) -> Vec<Vec<u8>> {
    grid.iter()
        .map(|line| line.iter().rev().copied().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2020::Solution;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 47213728755493);
        assert_eq!(res2, 1599);
    }

    #[test]
    fn test_ex() {
        assert_eq!(
            (20899048083289, 273),
            Solver::run_on(
                r#"
    Tile 2311:
    ..##.#..#.
    ##..#.....
    #...##..#.
    ####.#...#
    ##.##.###.
    ##...#.###
    .#.#.#..##
    ..#....#..
    ###...#.#.
    ..###..###

    Tile 1951:
    #.##...##.
    #.####...#
    .....#..##
    #...######
    .##.#....#
    .###.#####
    ###.##.##.
    .###....#.
    ..#.#..#.#
    #...##.#..

    Tile 1171:
    ####...##.
    #..##.#..#
    ##.#..#.#.
    .###.####.
    ..###.####
    .##....##.
    .#...####.
    #.##.####.
    ####..#...
    .....##...

    Tile 1427:
    ###.##.#..
    .#..#.##..
    .#.##.#..#
    #.#.#.##.#
    ....#...##
    ...##..##.
    ...#.#####
    .#.####.#.
    ..#..###.#
    ..##.#..#.

    Tile 1489:
    ##.#.#....
    ..##...#..
    .##..##...
    ..#...#...
    #####...#.
    #..#.#.#.#
    ...#.#.#..
    ##.#...##.
    ..##.##.##
    ###.##.#..

    Tile 2473:
    #....####.
    #..#.##...
    #.##..#...
    ######.#.#
    .#...#.#.#
    .#########
    .###.#..#.
    ########.#
    ##...##.#.
    ..###.#.#.

    Tile 2971:
    ..#.#....#
    #...###...
    #.#.###...
    ##.##..#..
    .#####..##
    .#..####.#
    #..#.#..#.
    ..####.###
    ..#.#.###.
    ...#.#.#.#

    Tile 2729:
    ...#.#.#.#
    ####.#....
    ..#.#.....
    ....#..#.#
    .##..##.#.
    .#.####...
    ####.#.#..
    ##.####...
    ##..#.##..
    #.##...##.

    Tile 3079:
    #.#.#####.
    .#..######
    ..#.......
    ######....
    ####.#..#.
    .#...#.##.
    #.#####.##
    ..#.###...
    ..#.......
    ..#.###...
                        "#,
            )
        );
    }
}
