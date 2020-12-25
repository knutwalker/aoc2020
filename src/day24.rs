use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

type Input = Vec<u8>;
type Output = usize;

register!(
    "input/day24.txt";
    run(input: Input) -> Output {
        run_both(intial_grid(input))
    }
);

type Tile = (i32, i32);
type Grid = HashSet<Tile>;

#[derive(Debug, Copy, Clone)]
enum State {
    Neutral,
    North,
    South,
}

impl Default for State {
    fn default() -> Self {
        Self::Neutral
    }
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

fn intial_grid(input: Vec<Vec<u8>>) -> Grid {
    input
        .into_iter()
        .map(|line| {
            line.into_iter()
                .scan(State::Neutral, |s, c| {
                    Some(match c {
                        b'e' => Some(match std::mem::take(s) {
                            State::Neutral => Dir::E,
                            State::North => Dir::NE,
                            State::South => Dir::SE,
                        }),
                        b'w' => Some(match std::mem::take(s) {
                            State::Neutral => Dir::W,
                            State::North => Dir::NW,
                            State::South => Dir::SW,
                        }),
                        b'n' => {
                            *s = State::North;
                            None
                        }
                        b's' => {
                            *s = State::South;
                            None
                        }
                        x => unreachable!("invalid input: {}", x),
                    })
                })
                .flatten()
                .fold((0, 0), |(x, y), dir| match dir {
                    Dir::E => (x + 2, y),
                    Dir::SE => (x + 1, y - 1),
                    Dir::SW => (x - 1, y - 1),
                    Dir::W => (x - 2, y),
                    Dir::NW => (x - 1, y + 1),
                    Dir::NE => (x + 1, y + 1),
                })
        })
        .fold(Grid::new(), |mut grid, tile| {
            if !grid.remove(&tile) {
                grid.insert(tile);
            }
            grid
        })
}

fn run_both(grid: Grid) -> (Output, Output) {
    (grid.len(), flipped(grid).nth(100).unwrap())
}

fn flipped(grid: Grid) -> impl Iterator<Item = Output> {
    successors(Some(grid), |g| Some(cycle(g))).map(|g| g.len())
}

fn cycle(grid: &Grid) -> Grid {
    count_neighbors(grid)
        .into_iter()
        .filter_map(|(tile, flipped)| match (grid.contains(&tile), flipped) {
            (true, 1..=2) | (false, 2) => Some(tile),
            _ => None,
        })
        .collect()
}

fn count_neighbors(grid: &Grid) -> HashMap<Tile, isize> {
    let cap = grid.len() * 36;
    let mut counts = HashMap::with_capacity(cap);
    for tile in grid {
        for tile in neighbours(tile) {
            *counts.entry(tile).or_default() += 1;
        }
    }
    counts
}

fn neighbours(&tile: &Tile) -> impl Iterator<Item = Tile> {
    Neighbors {
        tile,
        dir: Some(Dir::E),
    }
}

struct Neighbors {
    tile: Tile,
    dir: Option<Dir>,
}

impl Iterator for Neighbors {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let dir = self.dir.as_mut()?;
        let (x, y) = self.tile;
        let pos = match dir {
            Dir::E => {
                *dir = Dir::SE;
                (x + 2, y)
            }
            Dir::SE => {
                *dir = Dir::SW;
                (x + 1, y - 1)
            }
            Dir::SW => {
                *dir = Dir::W;
                (x - 1, y - 1)
            }
            Dir::W => {
                *dir = Dir::NW;
                (x - 2, y)
            }
            Dir::NW => {
                *dir = Dir::NE;
                (x - 1, y + 1)
            }
            Dir::NE => {
                self.dir = None;
                (x + 1, y + 1)
            }
        };
        Some(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2020::Solution;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 523);
        assert_eq!(res2, 4225);
    }

    #[test]
    fn test_ex() {
        assert_eq!(
            (10, 2208),
            Solver::run_on(
                "
            sesenwnenenewseeswwswswwnenewsewsw
            neeenesenwnwwswnenewnwwsewnenwseswesw
            seswneswswsenwwnwse
            nwnwneseeswswnenewneswwnewseswneseene
            swweswneswnenwsewnwneneseenw
            eesenwseswswnenwswnwnwsewwnwsene
            sewnenenenesenwsewnenwwwse
            wenwwweseeeweswwwnwwe
            wsweesenenewnwwnwsenewsenwwsesesenwne
            neeswseenwwswnwswswnw
            nenwswwsewswnenenewsenwsenwnesesenew
            enewnwewneswsewnwswenweswnenwsenwsw
            sweneswneswneneenwnewenewwneswswnese
            swwesenesewenwneswnwwneseswwne
            enesenwswwswneneswsenwnewswseenwsese
            wnwnesenesenenwwnenwsewesewsesesew
            nenewswnwewswnenesenwnesewesw
            eneswnwswnwsenenwnwnwwseeswneewsenese
            neswnwewnwnwseenwseesewsenwsweewe
            wseweeenwnesenwwwswnew
        "
            )
        );
    }
}