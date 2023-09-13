use std::thread::current;
use Base::into_vec::IntoVec;
use crate::Solution;

impl Solution {
    fn has_next_valid_step((current_x, current_y): (i32, i32), step: i32, grid: &Vec<Vec<i32>>) -> Option<(i32, i32)> {
        if grid.is_empty() {
            return None;
        }
        let y_bound = grid.len() as i32;
        let x_bound = grid[0].len() as i32;

        let next_step = step + 1;

        for x in [-1, 1] {
            for y in [-2, 2] {
                let x = current_x + x;
                let y = current_y + y;

                if x >= 0 && x < x_bound && y >= 0 && y < y_bound {
                    if grid[y as usize][x as usize] == next_step {
                        return Some((x, y));
                    }
                }
            }
        }

        for x in [-2, 2] {
            for y in [-1, 1] {
                let x = current_x + x;
                let y = current_y + y;

                if x >= 0 && x < x_bound && y >= 0 && y < y_bound {
                    if grid[y as usize][x as usize] == next_step {
                        return Some((x, y));
                    }
                }
            }
        }

        None
    }

    pub fn check_valid_grid(mut grid: Vec<Vec<i32>>) -> bool {
        let mut step = 0;
        let mut current = (0, 0);

        if grid.is_empty() {
            return true;
        }

        if grid[0][0] != 0{
            return false
        }

        let final_index = (grid.len() * grid.len()) as i32 - 1;

        loop {
            if final_index == step {
                return true;
            }

            match Solution::has_next_valid_step(current, step, &grid) {
                None => {
                    return false;
                }

                Some(next_position) => {
                    current = next_position;
                    step += 1;
                    grid[current.1 as usize][current.0 as usize] = -1;
                }
            }
        }
    }
}

#[test]
fn test_code_2596() {
    println!("{}", Solution::check_valid_grid([[0,11,16,5,20],[17,4,19,10,15],[12,1,8,21,6],[3,18,23,14,9],[24,13,2,7,22]].into_vec()))
}