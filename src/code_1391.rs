use crate::Solution;

impl Solution {
    fn next((last_x, last_y): (i32, i32),
            (x, y): (i32, i32),
            path: &Vec<Vec<i32>>) -> (i32, i32) {
        let mut next = match path[y as usize][x as usize] {
            1 => {
                if last_y != y {
                    (-1, -1)
                } else {
                    if last_x == x + 1 {
                        let mut next = (x - 1, y);
                        let next_turn = path[next.1 as usize][next.0 as usize];
                        if next_turn != 1 && next_turn != 4 && next_turn != 6 {
                            next = (-1, -1);
                        }
                        next
                    } else {
                        let mut next = (x + 1, y);
                        let next_turn = path[next.1 as usize][next.0 as usize];
                        if next_turn != 1 && next_turn != 3 && next_turn != 5 {
                            next = (-1, -1);
                        }
                        next
                    }
                }
            }

            2 => {
                if last_x != x {
                    (-1, -1)
                } else {
                    if last_y == y + 1 {
                        let mut next = (x, y - 1);
                        let next_turn = path[next.1 as usize][next.0 as usize];
                        if next_turn != 2 && next_turn != 3 && next_turn != 4 {
                            next = (-1, -1);
                        }
                        next
                    } else {
                        let mut next = (x, y + 1);
                        let next_turn = path[next.1 as usize][next.0 as usize];
                        if next_turn != 2 && next_turn != 5 && next_turn != 6 {
                            next = (-1, -1);
                        }
                        next
                    }
                }
            }

            3 => {
                if last_x == x - 1 && last_y == y {
                    let mut next = (x, y + 1);
                    let next_turn = path[next.1 as usize][next.0 as usize];
                    if next_turn != 2 && next_turn != 5 && next_turn != 6 {
                        next = (-1, -1);
                    }
                    next
                } else if last_x == x && last_y == y + 1 {
                    let mut next = (x - 1, y);
                    let next_turn = path[next.1 as usize][next.0 as usize];
                    if next_turn != 1 && next_turn != 4 && next_turn != 6 {
                        next = (-1, -1);
                    }
                    next
                } else {
                    (-1, -1)
                }
            }

            4 => {
                if last_x == x + 1 && last_y == y {
                    let mut next = (x, y + 1);
                    let next_turn = path[next.1 as usize][next.0 as usize];
                    if next_turn != 2 && next_turn != 5 && next_turn != 6 {
                        next = (-1, -1);
                    }
                    next
                } else if last_x == x && last_y == y + 1 {
                    let mut next = (x + 1, y);
                    let next_turn = path[next.1 as usize][next.0 as usize];
                    if next_turn != 1 && next_turn != 3 && next_turn != 5 {
                        next = (-1, -1);
                    }
                    next
                } else {
                    (-1, -1)
                }
            }

            5 => {
                if last_x == x - 1 && last_y == y {
                    let mut next = (x, y - 1);
                    let next_turn = path[next.1 as usize][next.0 as usize];
                    if next_turn != 2 && next_turn != 3 && next_turn != 4 {
                        next = (-1, -1);
                    }
                    next
                } else if last_x == x && last_y == y - 1 {
                    let mut next = (x - 1, y);
                    let next_turn = path[next.1 as usize][next.0 as usize];
                    if next_turn != 1 && next_turn != 5 && next_turn != 6 {
                        next = (-1, -1);
                    }
                    next
                } else {
                    (-1, -1)
                }
            }

            6 => {
                if last_x == x - 1 && last_y == y {
                    let mut next = (x, y - 1);
                    let next_turn = path[next.1 as usize][next.0 as usize];
                    if next_turn != 2 && next_turn != 3 && next_turn != 4 {
                        next = (-1, -1);
                    }
                    next
                } else if last_x == x && last_y == y - 1 {
                    let mut next = (x + 1, y);
                    let next_turn = path[next.1 as usize][next.0 as usize];
                    if next_turn != 1 && next_turn != 3 && next_turn != 5 {
                        next = (-1, -1);
                    }
                    next
                } else {
                    (-1, -1)
                }
            }

            _ => { (-1, -1) }
        };

        next
    }

    fn iter_check_path(mut last: (i32, i32),
                       (mut x, mut y): (i32, i32),
                       grid: &Vec<Vec<i32>>) -> bool {
        let y_bound = grid.len() as i32;
        let x_bound = grid[0].len() as i32;
        let mut visit = vec![vec![false; x_bound as usize]; y_bound as usize];
        visit[last.1 as usize][last.0 as usize] = true;

        loop {
            if x < 0 || x >= x_bound || y < 0 || y >= y_bound {
                return false;
            }

            if visit[y as usize][x as usize] {
                return false;
            }
            let next = Solution::next(last, (x, y), grid);
            visit[y as usize][x as usize] = true;


            last = (x, y);
            x = next.0;
            y = next.1;

            if x == x_bound - 1 && y == y_bound - 1 {
                return true;
            }
        }
    }

    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        if grid.is_empty() {
            return true;
        }
        if grid[0].len() <= 1 {
            return true;
        }

        match grid[0][0] {
            1 | 6 => {
                Solution::iter_check_path((0, 0), (1, 0), &grid)
            }
            2 | 3 => {
                Solution::iter_check_path((0, 0), (0, 1), &grid)
            }
            4 => {
                Solution::iter_check_path((0, 0), (0, 1), &grid) ||
                    Solution::iter_check_path((0, 0), (1, 0), &grid)
            }
            5 => {
                return false;
            }
            _ => {
                return false;
            }
        }
    }
}

#[test]
fn test_code_1391() {
    println!("{}", Solution::has_valid_path(vec![vec![4, 3, 3], vec![6, 5, 2]]));
    println!("{}", Solution::has_valid_path(vec![vec![1, 1, 2]]));
    println!("{}", Solution::has_valid_path(vec![vec![1, 3, 1], vec![2, 6, 1]]));
    println!("{}", Solution::has_valid_path(vec![vec![2, 4, 3], vec![6, 5, 2]]));
    println!("{}", Solution::has_valid_path(vec![vec![1, 1, 1, 1, 1, 1, 3]]));
    println!("{}", Solution::has_valid_path(vec![vec![2], vec![2], vec![2], vec![2], vec![2], vec![2], vec![6]]));
}