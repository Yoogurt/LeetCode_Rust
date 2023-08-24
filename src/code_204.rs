use crate::Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        match n {
            i32::MIN..=1 => 0,
            _ => {
                let mut vec = vec![true; (n + 1) as usize];
                let mut result = 0;
                for index in 2..n as usize {
                    if vec[index] {
                        result += 1;

                        for non_prime_index in 1..=(n as usize / index) {
                            vec[index * non_prime_index] = false;
                        }
                    }
                }

                result
            }
        }
    }
}

#[test]
fn test_code_204() {
    println!("{}", Solution::count_primes(10));
    println!("{}", Solution::count_primes(4));
    println!("{}", Solution::count_primes(15));
}