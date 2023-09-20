use crate::Solution;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();
        result.resize_with(mat.len(), || {
            let mut inner = Vec::<i32>::new();
            inner.resize(mat.first().unwrap().len(), i32::MAX);
            inner
        });

        fn index_value(data: &Vec<Vec<i32>>, (x, y): (usize, usize)) -> i32 {
            if (0..data.len()).contains(&x)
                && (0..data.first().unwrap().len()).contains(&y)
            {
                data[x][y]
            } else {
                -2
            }
        }

        let mut wait_for_visit_index = Vec::<(usize, usize)>::new();
        mat.iter().enumerate().for_each(|(x, row)| {
            row.iter().enumerate().for_each(|(y, value)| {
                if *value == 0 {
                    result[x][y] = 0;
                } else {
                    if y > 0 {
                        let left = (x, y - 1);
                        if index_value(&mat, left) == 0 {
                            wait_for_visit_index.push((x, y));
                            return;
                        }
                    }

                    if x > 0 {
                        let top = (x - 1, y);
                        if index_value(&mat, top) == 0 {
                            wait_for_visit_index.push((x, y));
                            return;
                        }
                    }

                    if y < mat.first().unwrap().len() - 1 {
                        let right = (x, y + 1);
                        if index_value(&mat, right) == 0 {
                            wait_for_visit_index.push((x, y));
                            return;
                        }
                    }

                    if x < mat.len() - 1 {
                        let bottom = (x + 1, y);
                        if index_value(&mat, bottom) == 0 {
                            wait_for_visit_index.push((x, y));
                            return;
                        }
                    }
                }
            });
        });

        let mut round = 1;
        while !wait_for_visit_index.is_empty() {
            let mut next_visit_index = Vec::<(usize, usize)>::new();
            wait_for_visit_index.iter().for_each(|(x, y)| {
                let (x, y) = (*x, *y);
                result[x][y] = std::cmp::min(round, result[x][y]);

                if y > 0 {
                    let left = (x, y - 1);
                    if index_value(&result, left) == i32::MAX {
                        next_visit_index.push(left);
                    }
                }

                if x > 0 {
                    let top = (x - 1, y);
                    if index_value(&result, top) == i32::MAX {
                        next_visit_index.push(top);
                    }
                }

                if y < mat.first().unwrap().len() - 1 {
                    let right = (x, y + 1);
                    if index_value(&result, right) == i32::MAX {
                        next_visit_index.push(right);
                    }
                }

                if x < mat.len() - 1 {
                    let bottom = (x + 1, y);
                    if index_value(&result, bottom) == i32::MAX {
                        next_visit_index.push(bottom);
                    }
                }
            });

            round += 1;
            wait_for_visit_index = next_visit_index;
        }

        result
    }
}

#[test]
fn test_code_542() {
    // println!("{:?}", Solution::update_matrix(vec![vec![0,0,0], vec![0,1,0], vec![0,0,0]]));
    println!(
        "{:?}",
        Solution::update_matrix(vec![
            vec![0, 1, 0],
            vec![0, 1, 0],
            vec![0, 1, 0],
            vec![0, 1, 0]
        ])
    );

    println!(
        "{:?}",
        Solution::update_matrix(vec![
            vec![0, 1, 0],
            vec![0, 1, 0],
            vec![1, 1, 1],
            vec![1, 1, 1]
        ])
    )
}
