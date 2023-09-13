use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use crate::Solution;
use Base::into_vec::IntoVec;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut edges_map = HashMap::<i32, HashSet<i32>>::new();

        edges.into_iter().for_each(|sides| {
            match edges_map.entry(sides[0]) {
                Entry::Vacant(to) => {
                    let mut set = HashSet::<i32>::new();
                    set.insert(sides[1]);
                    to.insert(set);
                }
                Entry::Occupied(mut to) => {
                    to.get_mut().insert(sides[1]);
                }
            }

            match edges_map.entry(sides[1]) {
                Entry::Vacant(to) => {
                    let mut set = HashSet::<i32>::new();
                    set.insert(sides[0]);
                    to.insert(set);
                }
                Entry::Occupied(mut to) => {
                    to.get_mut().insert(sides[0]);
                }
            }
        });

        let mut current = HashSet::new();
        current.insert(source);
        let mut visited = HashSet::<i32>::new();

        loop {
            if current.is_empty() {
                return false;
            }

            let mut new_current = HashSet::new();


            if current.into_iter().any(|to| {
                if to == destination {
                    return true;
                }

                let to_where = edges_map.get(&to);

                if let Some(to_where) = to_where {
                    to_where.iter().for_each(|to_where| {
                        if !visited.contains(&to_where) {
                            new_current.insert(*to_where);
                            visited.insert(*to_where);
                        }
                    });
                }

                false
            }) {
                return true;
            }

            current = new_current;
        }
    }
}

#[test]
fn test_code_1971() {
    println!("{}", Solution::valid_path(6, [[0,7],[0,8],[6,1],[2,0],[0,4],[5,8],[4,7],[1,3],[3,5],[6,5]].into_vec(), 7, 5));
}