extern crate utils;

use std::collections::HashSet;

use utils::parse_input;

type Point3D = (i32, i32, i32);
type Point4D = (i32, i32, i32, i32);
type Grid3D = HashSet<Point3D>;
type Grid4D = HashSet<Point4D>;

fn pocket_dimension_parser(contents: String) -> Grid3D {
    let mut res = Grid3D::new();
    let mut x = 0;

    for line in contents.split('\n') {
        let mut y = 0;
        for c in line.chars() {
            if c == '#' {
                res.insert((x, y, 0));
            }
            y += 1;
        }
        x += 1;
    }

    res
}

fn pocket_dimension_parser_4d(contents: String) -> Grid4D {
    let mut res = Grid4D::new();
    let mut x = 0;

    for line in contents.split('\n') {
        let mut y = 0;
        for c in line.chars() {
            if c == '#' {
                res.insert((x, y, 0, 0));
            }
            y += 1;
        }
        x += 1;
    }

    res
}

fn count_active_neighbours_3d(grid: &Grid3D, p: Point3D) -> i32 {
    let mut count = 0;
    for &n in get_neighbours_3d(p).iter() {
        if grid.contains(&n) {
            count += 1;
        }
    }
    count
}

fn get_neighbours_3d(p: Point3D) -> Vec<Point3D> {
    let mut res = vec![];
    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }

                res.push((p.0 + x, p.1 + y, p.2 + z));
            }
        }
    }
    res
}

fn count_active_neighbours_4d(grid: &Grid4D, p: Point4D) -> i32 {
    let mut count = 0;
    for &n in get_neighbours_4d(p).iter() {
        if grid.contains(&n) {
            count += 1;
        }
    }
    count
}

fn get_neighbours_4d(p: Point4D) -> Vec<Point4D> {
    let mut res = vec![];
    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                for w in -1..2 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }

                    res.push((p.0 + x, p.1 + y, p.2 + z, p.3 + w));
                }
            }
        }
    }
    res
}

fn run_cycle_3d(grid: Grid3D) -> Grid3D {
    let mut new_grid = Grid3D::new();

    for &p in grid.iter() {
        let active_neighbours = count_active_neighbours_3d(&grid, p);
        if active_neighbours == 2 || active_neighbours == 3 {
            new_grid.insert(p);
        }

        for &n in get_neighbours_3d(p).iter() {
            if !grid.contains(&n) && count_active_neighbours_3d(&grid, n) == 3 {
                new_grid.insert(n);
            }
        }
    }

    new_grid
}

fn run_cycle_4d(grid: Grid4D) -> Grid4D {
    let mut new_grid = Grid4D::new();

    for &p in grid.iter() {
        let active_neighbours = count_active_neighbours_4d(&grid, p);
        if active_neighbours == 2 || active_neighbours == 3 {
            new_grid.insert(p);
        }

        for &n in get_neighbours_4d(p).iter() {
            if !grid.contains(&n) && count_active_neighbours_4d(&grid, n) == 3 {
                new_grid.insert(n);
            }
        }
    }

    new_grid
}

fn problem1(initial_state: Grid3D) {
    let mut grid = initial_state;
    for _ in 0..6 {
        grid = run_cycle_3d(grid);
    }
    println!("Problem 1 -> {}", grid.iter().len());
}

fn problem2(initial_state: Grid4D) {
    let mut grid = initial_state;
    for _ in 0..6 {
        grid = run_cycle_4d(grid);
    }
    println!("Problem 2 -> {}", grid.iter().len());
}

fn main() {
    let grid = parse_input::<Grid3D>(
        "input",
        pocket_dimension_parser,
    );
    problem1(grid);

    let grid = parse_input::<Grid4D>(
        "input",
        pocket_dimension_parser_4d,
    );
    problem2(grid);
}
