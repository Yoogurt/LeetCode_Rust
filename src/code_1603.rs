use crate::Solution;

impl Solution {
    fn point_in_line(start: &Vec<i32>, end: &Vec<i32>, (x, _y): (f64, f64)) -> bool {
        return x >= start[0] as f64 && x <= end[0] as f64;
    }

    fn point_in_line_y<'a>(mut start: &'a Vec<i32>, mut end: &'a Vec<i32>, y: f64) -> bool {
        if start[1] > end[1] {
            let tunple = (start, end);
            start = tunple.1;
            end = tunple.0;
        }

        y >= start[1] as f64 && y <= end[1] as f64
    }

    pub fn intersection(
        mut start1: Vec<i32>,
        mut end1: Vec<i32>,
        mut start2: Vec<i32>,
        mut end2: Vec<i32>,
    ) -> Vec<f64> {
        // swap
        if end2[0] - start2[0] == 0 {
            let tunple = (start2, end2, start1, end1);
            start1 = tunple.0;
            end1 = tunple.1;
            start2 = tunple.2;
            end2 = tunple.3;
        }

        if start1[0] > end1[0] {
            let tunple = (end1, start1);
            start1 = tunple.0;
            end1 = tunple.1;
        }

        if start2[0] > end2[0] {
            let tunple = (end2, start2);
            start2 = tunple.0;
            end2 = tunple.1;
        }

        if end1[0] - start1[0] == 0 {
            if end2[0] - start2[0] == 0 {
                if end2[0] == end1[0] {
                    let mut ret = vec![
                        (start1[0], start1[1]),
                        (start2[0], start2[1]),
                        (end1[0], end1[1]),
                        (end2[0], end2[1]),
                    ];

                    ret.sort_by(|left, right| left.1.cmp(&right.1));
                    let common = (ret[1].0 as f64, ret[1].1 as f64);

                    if Solution::point_in_line_y(&start1, &end1, common.1)
                        && Solution::point_in_line_y(&start2, &end2, common.1)
                    {
                        return vec![common.0, common.1];
                    } else {
                        return vec![];
                    }
                } else {
                    return vec![];
                }
            } else {
                let common_x = end1[0] as f64;
                let c = (start2[1] - end2[1]) as f64 / (start2[0] - end2[0]) as f64;
                let d = start2[1] as f64 - c * start2[0] as f64;

                let common_y = c * common_x as f64 + d;

                if start1[1] > end1[1] {
                    let tunple = (end1, start1);
                    start1 = tunple.0;
                    end1 = tunple.1;
                }

                if Solution::point_in_line(&start2, &end2, (common_x, common_y))
                    && (common_y >= start1[1] as f64 && common_y <= end1[1] as f64)
                {
                    return vec![common_x, common_y];
                } else {
                    return vec![];
                }
            }
        }

        let a = (start1[1] - end1[1]) as f64 / (start1[0] - end1[0]) as f64;
        let b = start1[1] as f64 - a * start1[0] as f64;
        let c = (start2[1] - end2[1]) as f64 / (start2[0] - end2[0]) as f64;
        let d = start2[1] as f64 - c * start2[0] as f64;

        if (a - c).abs() < 10e-6 {
            // same
            if (b - d).abs() < 10e-6 {
                let mut ret = vec![
                    (start1[0], start1[1]),
                    (start2[0], start2[1]),
                    (end1[0], end1[1]),
                    (end2[0], end2[1]),
                ];

                ret.sort_by(|left, right| left.0.cmp(&right.0));

                let common_x = ret[1].0 as f64;
                let common_y = ret[1].1 as f64;
                if Solution::point_in_line(&start2, &end2, (common_x, common_y))
                    && Solution::point_in_line(&start1, &end1, (common_x, common_y))
                {
                    return vec![common_x, common_y];
                } else {
                    return vec![];
                }
            } else {
                return vec![];
            }
        }

        let common_y = (c * b - a * d) / (c - a);
        let common_x = if (a - 0f64).abs() < 10e-6 {
            (common_y - d) / c
        } else {
            (common_y - b) / a
        };

        if Solution::point_in_line(&start1, &end1, (common_x, common_y))
            && Solution::point_in_line(&start2, &end2, (common_x, common_y))
        {
            return vec![common_x, common_y];
        }

        return vec![];
    }
}

#[test]
fn test_code_1603() {
    println!(
        "{:?}",
        Solution::intersection(vec![1, 1], vec![-1, 1], vec![1, 0], vec![-3, 2])
    );
}
