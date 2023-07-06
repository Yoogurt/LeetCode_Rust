use crate::Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();

        Solution::all_paths_source_target_internal(&graph, 0, vec![], &mut result);

        result
    }

    fn all_paths_source_target_internal(
        graph: &Vec<Vec<i32>>,
        index: i32,
        mut current: Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        current.push(index);

        let path = graph.get(index as usize);
        match path {
            Some(paths) => {
                if index as usize == graph.len() - 1 {
                    result.push(current);
                    return;
                }

                paths.iter().for_each(|&path| {
                    Solution::all_paths_source_target_internal(graph, path, current.clone(), result);
                });
            }
            None => {
                if index as usize == graph.len() - 1 {
                    result.push(current);
                }
            }
        }
    }
}

#[test]
fn test_code_797() {
    println!("{:?}", Solution::all_paths_source_target(vec![vec![1,2], vec![3], vec![3], vec![]]));
    println!("{:?}", Solution::all_paths_source_target(vec![vec![4,3,1], vec![3,2,4], vec![3], vec![4], vec![]]));
    println!("{:?}", Solution::all_paths_source_target(vec![vec![4,3,1], vec![3,2,4], vec![], vec![4], vec![]]));
    println!("{:?}", Solution::all_paths_source_target(vec![vec![2], vec![], vec![1]]));
}