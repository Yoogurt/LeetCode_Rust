use crate::Solution;

impl Solution {
    fn mark_island_visit((x, y): (usize, usize), grid: &mut Vec<Vec<char>>) {
        if 0 > x || x >= grid.len() {
            return;
        }

        if 0 > y || y >= grid[0].len() {
            return;
        }
        if grid[x][y] == '0' {
            return;
        }

        grid[x][y] = '0';

        if x > 0 {
            Solution::mark_island_visit((x - 1, y), grid);
        }
        Solution::mark_island_visit((x + 1, y), grid);

        if y > 0 {
            Solution::mark_island_visit((x, y - 1), grid);
        }
        Solution::mark_island_visit((x, y + 1), grid);
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut result = 0;

        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                let island = &grid[x][y];

                if island == &'1' {
                    Solution::mark_island_visit((x, y), &mut grid);
                    result += 1;
                }
            }
        }

        result
    }
}

#[test]
fn test_code_200() {
    println!(
        "{}",
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0']
        ])
    );

    println!(
        "{}",
        Solution::num_islands(vec![
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '1', '1']
        ])
    )
}
